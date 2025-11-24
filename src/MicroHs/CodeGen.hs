module MicroHs.CodeGen where
import Prelude(); import MHSPrelude
import Data.List
import qualified MicroHs.IdentMap as M
import Data.Maybe
import MicroHs.Desugar(LDef)
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

-- Takes the result of abstraction, generate heap image and comb image
codeGen :: (Ident, [LDef]) -> ([LDef], [AExp], [AExp])
codeGen (mainName, ds) =
  let
    removed = deadRemove (mainName, ds)
    absorbed = absorbSingleton removed
    (heap, comb) = extractCombs $ escToSc absorbed
    singletons = collectSingleton heap
    (heap', varMap) = numberFuns comb (mainName, heap) singletons
    comb' = map (expToAExp True . substVar varMap) comb
  in (removed, heap', dashArgs comb')
  -- in (escToSc removed, heap', map (substVar varMap) comb)

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
        Esc ar bd -> let
          (bd', args'') = walk' bd 0 args'
          diff = length args' - length args''
          c' = Esc (ar - diff) bd'
          in foldl App c' args''
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

defaultRecord :: SCRecord
defaultRecord = [(scK, 0), (scA, 1)]

defaultCombs :: [Exp] -> [Exp]
defaultCombs = ([Arg 0, Arg 1] ++)

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

    (ds', (_, comb, _)) = runState (extract ds) (defaultRecord, defaultCombs, 2)
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
