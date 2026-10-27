module MicroHs.CodeGen where
import Prelude(); import MHSPrelude
import Data.List
import Data.Char(ord)
import qualified MicroHs.IdentMap as M
import Data.Maybe
import MicroHs.Desugar(LDef)
import MicroHs.EncodeData(encList)
import MicroHs.Exp
import MicroHs.Expr(Lit(..), showLit, errorMessage, HasLoc(..))
import MicroHs.Ident(Ident(..), showIdent, mkIdent)
import MicroHs.State
import MicroHs.AbstractESC
import MicroHs.IdentMap (toList)

type Arity = Int
type Ptr = Int

-- This IR is closer to machine's program representation
data Atom
  = Prm String -- Y, seq, operators and other `LPrim`
  | Int Int
  | Com Arity Ptr -- Pointer to the comb image
  | Fun Ptr -- Pointer to the heap image
  | Ptr Ptr Bool -- Pointer to other Apps within AExp
  | Apt Ptr Bool -- Argument pointer
  deriving(Show)
  
type App = [Atom]
type AExp = [App]

expToAtom :: Exp -> Atom
expToAtom ae =
  case ae of
    Var f -> if "FUN" `isPrefixOf` i then Fun $ read (drop 3 i)
               else error ("Strange Var exists: " ++ i) where i = showIdent f
    Lit (LInt i) -> Int i
    Lit (LPrim s) -> Prm s
    Lit (LForImp s _) ->
      error ("Ouros cannot emit C foreign import: " ++ s)
    Sc a _ _ -> Com a 42
    Esc a (Cbp p) -> Com a p
    Arg i -> Apt i True -- temp value
    _ -> error $ "Not an Atom: " ++ show ae

expToAExp :: Bool -> Exp -> AExp
expToAExp onComb ae =
  let
    -- states: 1. ptr counter; 2. current spine; 3. accumulator
    expToAExp' :: Exp -> State (Int, App, [App] -> [App]) ()
    expToAExp' e = do
      (i, spn, acc) <- get
      case e of
        App f nt@(App _ _) -> do
          put (i, [], acc)
          expToAExp' nt
          (i', spn', acc') <- get
          put (i' + 1, Ptr i' onComb : spn, acc' . (spn' :))
          expToAExp' f
        App f a -> do
          put (i, expToAtom a : spn, acc)
          expToAExp' f
        _ -> do
          put (i, expToAtom e : spn, acc)
    (_, (_, rspn, racc)) = runState (expToAExp' ae) (0, [], id)
  in rspn : racc []

substVar :: M.Map Exp -> Exp -> Exp
substVar m e =
  let
    findIdent n = findIdentIn n m
    substv aexp =
      case aexp of
        Var n ->
          case findIdent n of
            found@(Var f) ->
              if "FUN" `isPrefixOf` i
                then found
                else substVar m found
              where i = showIdent f
            others -> others
        App f a -> App (substv f) (substv a)
        _ -> aexp
  in substv e

findIdentIn :: Ident -> M.Map Exp -> Exp
findIdentIn n m = fromMaybe
  (errorMessage (getSLoc n) $ "No definition found for: " ++ showIdent n) $
  M.lookup n m

-- Expand string literals BEFORE compileEsc (called from Compile.hs for
-- Ouros targets).  Doing this after abstraction inserts an arbitrarily deep
-- list spine into an existing Esc body, violating the reducer format.
-- The resulting O/K primitives are handled by compileEsc's removeSKI pass.
lowerStrings :: Exp -> Exp
lowerStrings ae =
  case ae of
    -- encList uses O/K primitives; compileEsc turns these into supported
    -- Sc combinators before the Ouros ROM serializer sees the expression.
    Lit (LStr cs) -> encList (map (Lit . LInt . ord) cs)
    App f a -> App (lowerStrings f) (lowerStrings a)
    Lam x e -> Lam x (lowerStrings e)
    Esc ar e -> Esc ar (lowerStrings e)
    _ -> ae

-- MicroHs desugars non-exhaustive matches to an ordinary call to
-- Control.Exception.Internal.patternMatchFail, which depends on the native
-- exception/FFI runtime.  Ouros has a dedicated error Atom instead.
--
-- Keep this Ouros-only (CodeGen is not used by native MicroHs output).
-- Match the fully applied compiler-generated call, not arbitrary imports;
-- legitimate uses of foreign functions must still fail validation.
--
-- Run before compileEsc when possible, and again here as a safeguard for
-- previously compiled/cache-loaded module definitions.
lowerOurosPatternFailures :: Exp -> Exp
lowerOurosPatternFailures ex =
  case ex of
    App (Var f) _
      | showIdent f == "Control.Exception.Internal.patternMatchFail" ->
          Lit (LPrim "error0")
    App f x -> App (lowerOurosPatternFailures f) (lowerOurosPatternFailures x)
    Lam x body -> Lam x (lowerOurosPatternFailures body)
    Esc ar body -> Esc ar (lowerOurosPatternFailures body)
    other -> other

-- Takes the result of abstraction, generate heap image and comb image
codeGen :: (Ident, [LDef]) -> ([LDef], [AExp], [AExp])
codeGen (mainName, ds) =
  let
    -- Strings were lowered ahead of abstraction.  Do not lower them here:
    -- new Apps introduced within Esc bodies would bypass hardware limits.
    -- Some imports may come from cache; eliminate the synthetic exception
    -- dependency on the final combined graph before dead-code elimination.
    ourosDefs = [ (i, lowerOurosPatternFailures e) | (i, e) <- ds ]
    removed = rejectForeignImports (deadRemove (mainName, ourosDefs))
    absorbed = absorbSingleton removed
    (heap, comb) = extractCombs $ escToSc absorbed
    singletons = collectSingleton heap
    (heap', varMap) = numberFuns comb (mainName, heap) singletons
    comb' = map (expToAExp True . substVar varMap) comb
    checkedHeap = zipWith checkHeapAExp [0..] (map splitHeapAExp heap')
    checkedComb = zipWith checkCombAExp [0..] comb'
  in (removed, checkedHeap, dashArgs checkedComb)
  -- in (escToSc removed, heap', map (substVar varMap) comb)

-- A second, backend-level guard: every emitted reducer record must fit
-- in the physical format even if a future optimization changes an Esc body.
checkCombAExp :: Int -> AExp -> AExp
checkCombAExp i ae
  | length ae > 8 = bad "more than seven nested Apps"
  | any ((> 8) . length) ae = bad "App wider than eight Atoms"
  | any (any isNestedPtr) (drop 1 ae) = bad "nested App points to another nested App"
  | any (any invalidComArity) ae = bad "Com arity outside 0..7"
  | otherwise = ae
  where
    bad reason = error ("invalid Ouros comb AExp" ++ show i ++ ": " ++ reason)
    isNestedPtr (Ptr _ True) = True
    isNestedPtr _ = False

-- Heap Apps are unrestricted in nesting depth, but every physical App
-- must contain at most eight Atoms.  A wide left-associated application
-- [a,b,c,d,e,f,g,h,i] becomes:
--
--   root:   [Ptr n False, i]
--   app n:  [a,b,c,d,e,f,g,h]
--
-- When an App needs several chunks, each new chunk points to the preceding
-- chunk.  Existing nested-App indices (0 .. length ae - 2) are kept intact:
-- new chunks are appended AFTER all original Apps.  Therefore no existing
-- Ptr needs to be renumbered, and the new Ptr indices are allocated
-- consecutively from length ae - 1.
--
-- Do NOT apply this to combinator bodies: the reducer ROM forbids a nested
-- App from pointing to another nested App, so comb size is checked instead.
splitHeapAExp :: AExp -> AExp
splitHeapAExp ae =
  let
    (_, pieces) = mapAccumL splitOne (length ae - 1) ae
  in map fst pieces ++ concatMap snd pieces
  where
    splitOne :: Int -> App -> (Int, (App, [App]))
    splitOne next atoms =
      let (root, extra) = splitWide next atoms
      in (next + length extra, (root, extra))

    splitWide :: Int -> App -> (App, [App])
    splitWide next atoms =
      case splitAt 8 atoms of
        (prefix, []) -> (prefix, [])
        (prefix, suffix) ->
          let (root, further) = splitWide (next + 1) (Ptr next False : suffix)
          in (root, prefix : further)

checkHeapAExp :: Int -> AExp -> AExp
checkHeapAExp i ae
  | any ((> 8) . length) ae =
      error ("invalid Ouros heap AExp" ++ show i ++ ": App wider than eight Atoms")
  | any (any invalidComArity) ae =
      error ("invalid Ouros heap AExp" ++ show i ++ ": Com arity outside 0..7")
  | otherwise = ae

-- The backend must never emit a negative or unencodable Com arity.
-- The limit matches the maximum supported Ouros combinator arity.
invalidComArity :: Atom -> Bool
invalidComArity (Com ar _) = ar < 0 || ar > 7
invalidComArity _ = False

-- The standalone Ouros target cannot call native MicroHs C FFI entries.
-- Check only live bindings (after deadRemove), and report their original
-- names before singleton substitution obscures the provenance.  In
-- particular, showLit prints LForImp "pokeWord" as "^pokeWord".
rejectForeignImports :: [LDef] -> [LDef]
rejectForeignImports ds =
  case [ (showIdent name, imp)
       | (name, body) <- ds
       , imp <- foreignImports body ] of
    [] -> ds
    (name, imp) : _ ->
      error ("Ouros: reachable C foreign import " ++ show imp ++
             " in binding " ++ name ++
             ". Replace this dependency with NanoPrelude/Ouros primitives.")
  where
    foreignImports :: Exp -> [String]
    foreignImports (Lit (LForImp imp _)) = [imp]
    foreignImports (App f x) = foreignImports f ++ foreignImports x
    foreignImports (Lam _ body) = foreignImports body
    foreignImports (Esc _ body) = foreignImports body
    foreignImports _ = []

-- remove unused definitions
deadRemove :: (Ident, [LDef]) -> [LDef]
deadRemove (mainName, ds) =
  let
    dMap = M.fromList ds
    dfs :: Ident -> State (Int, M.Map Exp, [LDef] -> [LDef]) ()
    dfs n = do
      (i, seen, r) <- get
      case M.lookup n seen of
        Just _ -> return ()
        Nothing -> do
          let e = findIdentIn n dMap
          put (i+1, M.insert n idle seen,  r . ((n, e) :))
          -- Walk n's children
          mapM_ dfs $ freeVars e
    (_,(_, _, res)) = runState (dfs mainName) (0, M.empty, id)    
    idle = Var $ mkIdent "FUN"
  in res []

-- absorb singletons into combinators
absorbSingleton :: [LDef] -> [LDef]
absorbSingleton ds =
  let
    walk :: Exp -> Exp
    walk e =
      let
        (c, args) = spine e
        args' = map walk args
      in case c of
        Esc ar bd | ar > 0 -> let
          -- Only formal arguments can be absorbed.  Any remaining Apps in
          -- the spine are over-application of the combinator's result,
          -- not additional arguments to the combinator itself.
          -- Keep one formal argument unabsorbed so the resulting Com
          -- still has a positive arity (the reducer needs a redex).
          (candidates, rest) = splitAt (ar - 1) args'
          (bd', kept) = walk' bd 0 candidates
          absorbedCount = length candidates - length kept
          c' = Esc (ar - absorbedCount) bd'
          in foldl App c' (kept ++ rest)
        Esc ar bd -> foldl App (Esc ar bd) args'
        _ -> foldl App c args'
    walk' :: Exp -> Int -> [Exp] -> (Exp, [Exp])
    walk' e _      []       = (e, [])
    walk' e offset (ar:ars) =
      if isSingle ar then let
        absorbed = absorb e offset ar
        alligned = mapExpOnArgInt (\i -> if i > offset then i - 1 else i) absorbed
        res = walk' alligned offset ars
        in res
      else let
        (resFst, resSnd) = walk' e (offset + 1) ars
        in (resFst, ar : resSnd)
    absorb :: Exp -> Int -> Exp -> Exp
    absorb bd i sgt =
      mapExpOnArg (\(Arg a) -> if a == i then sgt else Arg a) bd
    absorb _ _ _ = undefined
  in map (\(n, ex) -> (n, walk ex)) ds

escToSc :: [LDef] -> [LDef]
escToSc defs =
  let
    toSc (App e1 e2) = App (toSc e1) (toSc e2)
    toSc (Esc a bd)
      | allArgs bd = Sc a (takePat bd) (pullout bd)
      | otherwise  = Esc a (toSc bd)
    toSc e = e
    allArgs (App e1 e2) = allArgs e1 && allArgs e2
    allArgs (Arg _) = True
    allArgs _ = False
    takePat (App e1 e2) = At (takePat e1) (takePat e2)
    takePat _ = X
  in map (\(i, e) -> (i, toSc e)) defs

isSingle :: Exp -> Bool
isSingle (App _ _) = False
isSingle _ = True

collectSingleton :: [LDef] -> M.Map Exp
collectSingleton defs =
  let
    collect [] m = m
    collect ((i, e) : ies) m =
      if isSingle e then collect ies (M.insert i e m) else collect ies m
  in collect defs M.empty

-- switch to index from Ident for function calls, turn into AExp
-- will also strip off unused defs (caused by inlineSingleton)
numberFuns :: [Exp] -> (Ident, [LDef]) -> M.Map Exp -> ([AExp], M.Map Exp)
numberFuns combs (mainName, ds) mp =
  let    
    dMap = M.fromList ds
    dfs :: Ident -> State (Int, M.Map Exp, [Exp] -> [Exp]) ()
    dfs n = do
      (i, seen, r) <- get
      case M.lookup n seen of
        Just _ -> return ()
        Nothing -> do
          let e = findIdentIn n dMap
          put (i+1, M.insert n (ref i) seen,  r . (substVar m e :))
          -- Walk n's children
          mapM_ dfs $ trackVars e
    sgls :: Exp -> State (Int, M.Map Exp, [Exp] -> [Exp]) ()
    sgls e = do mapM_ dfs $ trackVars e
    trackVars :: Exp -> [Ident]
    trackVars ae =
      case ae of
        Var i -> [i]
        App f a -> trackVars f ++ trackVars a
        Esc _ (Cbp p) -> trackVars $ combs !! p -- WARNING: infinity?
        _ -> []
    fullAct = do
      dfs mainName
      mapM_ (\(_, e) -> sgls e) (toList mp)
    (_,(_, m, res)) = runState fullAct (0, mp, id)
    ref i = Var $ mkIdent $ "FUN" ++ show i
  in (map (expToAExp False) $ res [], m)

type SCRecord = [(Exp, Int)]

scK = Sc 2 X [0] -- False
scA = Sc 2 X [1] -- True
scPair = Sc 3 (At (At X X) X) [2, 0, 1] -- \xyk. kxy

defaultRecord :: SCRecord
defaultRecord = [(scK, 0), (scA, 1), (scPair, 2)]

defaultCombs :: [Exp] -> [Exp]
defaultCombs = ([Arg 0,
                 Arg 1,
                 App (App (Arg 2) (Arg 0)) (Arg 1)] ++)

matchSc :: Exp -> Exp -> Bool
matchSc (Sc _ p1 is1) (Sc _ p2 is2) =
  p1 == p2 && is1 == is2
matchSc _ _ = False

-- extract combinators into comb table, switch to comb ptrs
extractCombs :: [LDef] -> ([LDef], [Exp])
extractCombs ds =
  let
    extract :: [LDef] -> State (SCRecord, [Exp] -> [Exp], Int) [LDef]
    extract [] = return []
    extract (df:dfs) = do
      let (n, e) = df
      e' <- extr e
      r <- extract dfs
      return ((n, e') : r)

    extr :: Exp -> State (SCRecord, [Exp] -> [Exp], Int) Exp
    extr (App e1 e2) = do
      e1' <- extr e1
      e2' <- extr e2
      return (App e1' e2')
    extr sc@(Sc a _ _) = do
      (scs, es, len) <- get
      case find (\(e, _) -> matchSc sc e) scs of
        Nothing -> do
          put ((sc, len) : scs, es . (getBody (scToEsc sc) :), len + 1)
          return (Esc a (Cbp len))
        Just (_, i) -> do
          return (Esc a (Cbp i))
    extr (Esc a e) = do
      ------ try to fully expand ------
      e' <- extr e
      ------ try to fully expand ------
      (scs, es, len) <- get
      put (scs, es . (e' :), len + 1)
      return (Esc a (Cbp len))
    extr e = return e

    getBody (Esc _ e) = e

    (ds', (_, comb, _)) = runState (extract ds) (defaultRecord, defaultCombs, 3)
  in (ds', comb [])

dashArgs :: [AExp] -> [AExp]
dashArgs =
  let
    matchArg :: Atom -> Int -> Bool
    matchArg (Apt p _) p' = p == p'
    matchArg _ _ = False
    countArg :: AExp -> Int -> Int
    countArg aexp p = sum (map (length . filter (`matchArg` p)) aexp)
    onArg :: AExp -> Atom -> Atom
    onArg aexp a@(Apt p _) = if countArg aexp p > 1 then Apt p False else a
    onArg _ a = a
    walk :: AExp -> AExp
    walk aexp =
      map (map (onArg aexp)) aexp
  in map walk


l = mkIdent "l"
h = mkIdent "h"
ieft = mkIdent "enumFromTo"
eft = lams [l, h] (apps ltlh [Lit (LPrim "[]"), mkL])
ltlh = apps (Lit (LPrim "<=")) [Var l, Var h]
pl1  = apps (Lit (LPrim "+")) [Var l, Lit (LInt 1)]
recEnum = apps (Var ieft) [pl1, Var h]
mkL = apps (Lit (LPrim ":")) [Var l, recEnum]

eftRes = codeGen (ieft, [(ieft, compileEsc eft)])  
