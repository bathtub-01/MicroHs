module MicroHs.CodeGen (
  codeGen,
  deadRemove) where
import Prelude(); import MHSPrelude
import Data.List
import qualified MicroHs.IdentMap as M
import Data.Maybe
import MicroHs.Desugar(LDef)
import MicroHs.Exp
import MicroHs.Expr(Lit(..), showLit, errorMessage, HasLoc(..))
import MicroHs.Ident(Ident(..), showIdent, mkIdent)
import MicroHs.State
import MicroHs.Abstract
import MicroHs.AbstractESC(scToEsc)
import Data.Char (isDigit)


type Arity = Int
type Ptr = Int

-- This IR is closer to machine's program representation
data Atom
  = Prm String -- Y, seq, operators and other `LPrim`
  | Int Int
  | Com Arity Ptr
  | Fun Ptr
  | Ptr Ptr
  | Apt Ptr
  deriving(Show)
  
type App = [Atom]
type AExp = [App]

expToAtom :: Exp -> Atom
expToAtom ae =
  case ae of
    Var f -> if "FUN" `isPrefixOf` i then Fun $ read (drop 3 i)
               else error "Strange Var exists." where i = showIdent f
    Lit (LInt i) -> Int i
    Lit (LPrim s) -> Prm s
    Sc a _ _ -> Com a 42
    Esc a (Cbp p) -> Com a p
    Arg i -> Apt i
    _ -> error $ "Not an Atom: " ++ show ae

expToAExp :: Exp -> AExp
expToAExp ae =
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
          put (i' + 1, Ptr i' : spn, acc' . (spn' :))
          expToAExp' f
        App f a -> do
          put (i, expToAtom a : spn, acc)
          expToAExp' f
        _ -> do
          put (i, expToAtom e : spn, acc)
    (_, (_, rspn, racc)) = runState (expToAExp' ae) (0, [], id)
  in rspn : racc []

isCombPlcHolder :: Ident -> Bool
isCombPlcHolder n = all isDigit $ showIdent n

substVar :: M.Map Exp -> Exp -> Exp
substVar m e =
  let
    findIdent n = findIdentIn n m
    substv aexp =
      case aexp of
        Var n -> findIdent n
        App f a -> App (substv f) (substv a)
        _ -> aexp
  in substv e

findIdentIn :: Ident -> M.Map Exp -> Exp
findIdentIn n m = fromMaybe
  (errorMessage (getSLoc n) $ "No definition found for: " ++ showIdent n) $
  M.lookup n m

-- Takes the result of abstraction, generate heap image and comb image
-- codeGen :: (Ident, [LDef]) -> ([AExp], [AExp])
codeGen (mainName, ds) =
  let
    removed = deadRemove (mainName, ds)
    (heap, comb, scs) = extractCombs removed
    singletons = collectSingleton heap
    (heap', varMap) = numberFuns (mainName, heap) singletons
    comb' = map (expToAExp . substVar varMap) comb
  in (heap', comb')

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

collectSingleton :: [LDef] -> M.Map Exp
collectSingleton defs =
  let
    isSingle (App _ _) = False
    isSingle _ = True
    collect [] m = m
    collect ((i, e) : ies) m =
      if isSingle e then collect ies (M.insert i e m) else collect ies m
  in collect defs M.empty

-- switch to index from Ident for function calls, turn into AExp
-- will also strip off unused defs (caused by inlineSingleton)
numberFuns :: (Ident, [LDef]) -> M.Map Exp -> ([AExp], M.Map Exp)
numberFuns (mainName, ds) mp =
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
          mapM_ dfs $ freeVars e
    (_,(_, m, res)) = runState (dfs mainName) (0, mp, id)
    ref i = Var $ mkIdent $ "FUN" ++ show i
  in (map expToAExp $ res [], m)

type SCRecord = [(Exp, Int)]

scK = Sc 2 X [0] -- False
scA = Sc 2 X [1] -- True

defaultRecord :: SCRecord
defaultRecord = [(scK, 0), (scA, 1)]

defaultCombs :: [Exp] -> [Exp]
defaultCombs = ([Arg 0, Arg 1] ++)

matchSc :: Exp -> Exp -> Bool
matchSc (Sc a1 p1 is1) (Sc a2 p2 is2) =
  a1 == a2 && p1 == p2 && is1 == is2
matchSc _ _ = False

-- extract combinators into comb table, switch to comb ptrs
extractCombs :: [LDef] -> ([LDef], [Exp], SCRecord)
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
          put (scs, es . (getBody (scToEsc sc) :), len + 1)
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

    (ds', (scs', comb, _)) = runState (extract ds) (defaultRecord, defaultCombs, 2)
  in (ds', comb [], scs')

-- fully expand comb table
-- expandCombs :: [Exp] -> [Exp]
-- expandCombs combs =
--   let
--     expand :: Exp -> State(SCRecord, [Exp] -> [Exp], Int) Exp
--     expand = undefined
--     expand' :: Exp -> State(SCRecord, [Exp] -> [Exp], Int) (Exp, [Exp])
--     expand' (App e1 e2) = do
--       (ee1, dirty1) <- expand' e1
--       (ee2, dirty2) <- expand' e2
--       return (App ee1 ee2, dirty1 ++ dirty2)
--     expand' sc@(Sc a _ _) = do
--       (scs, es, len) <- get
--       case find (\(e, _) -> matchSc sc e) scs of
--         Nothing -> do
--           put (scs, es . (getBody (scToEsc sc) :), len + 1)
--           return (Esc a (Cbp len), [])
--     -- Exp -> (Exp, [Exp])
--     -- Exp -> [Exp]
--   in undefined




















  
