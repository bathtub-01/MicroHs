module MicroHs.AbstractESC(
  compileEsc,
  scToEsc
  ) where
import Prelude(); import MHSPrelude
import MicroHs.Ident
import MicroHs.Exp
import MicroHs.Expr(Lit(..))

-- compileOpt: remove all lambdas
-- print program imgs (heap + reducer); translate remaining SCs into ESCs

isPrim :: String -> Exp -> Bool
isPrim s ae =
  case ae of
    Lit (LPrim ss) -> s == ss
    _       -> False

scI = Sc 1 X [0]
scK = Sc 2 X [0]
scB = Sc 3 (At X (At X X)) [0, 1, 2]
scS = Sc 3 (At (At X X) (At X X)) [0, 2, 1, 2]
scC = Sc 3 (At (At X X) X) [0, 2, 1]
scA = Sc 2 X [1]
scO = Sc 4 (At (At X X) X) [3, 0, 1]
scP = Sc 3 (At (At X X) X) [2, 0, 1]
scC' = Sc 4 (At (At X (At X X)) X) [0, 1, 3, 2]
scC'B = Sc 4 (At (At X X) (At X X)) [0, 2, 1, 3]
scU = Sc 2 (At X X) [1, 0]
scZ = Sc 3 (At X X) [0, 1]
scS' = Sc 4 (At (At X (At X X)) (At X X)) [0, 1, 3, 2, 3]
scR = Sc 3 (At (At X X) X) [1, 2, 0]
scK2 = Sc 3 X [0]
scB' = Sc 4 (At (At X X) (At X X)) [0, 1, 2, 3]
scK3 = Sc 4 X [0]
scK4 = Sc 5 X [0]

scToEsc :: Exp -> Exp
scToEsc (Sc a p is) = Esc a body
  where
    args = map Arg [0..]
    body = fromPat p is args
scToEsc _ = undefined

removeSKI :: Exp -> Exp
removeSKI (App f a) = App (removeSKI f) (removeSKI a)
removeSKI (Lam x a) = Lam x (removeSKI a)
removeSKI ae
  | isPrim "I" ae = scI
  | isPrim "K" ae = scK
  | isPrim "B" ae = scB
  | isPrim "S" ae = scS
  | isPrim "C" ae = scC
  | isPrim "A" ae = scA
  | isPrim "O" ae = scO
  | isPrim "P" ae = scP
  | isPrim "C'" ae = scC'
  | isPrim "C'B" ae = scC'B
  | isPrim "U" ae = scU
  | isPrim "Z" ae = scZ
  | isPrim "S'" ae = scS'
  | isPrim "R" ae = scR
  | isPrim "K2" ae = scK2
  | isPrim "B'" ae = scB'
  | isPrim "K3" ae = scK3
  | isPrim "K4" ae = scK4
  | otherwise = ae    

-- compilation pipeline:
-- 1. removeSKI: convert SKI combinators into SCs
-- 2. compileExpEsc: remove lambdas into ESCs
compileEsc :: Exp -> Exp
compileEsc = compileExpEsc . removeSKI

compileExpEsc :: Exp -> Exp
compileExpEsc ae =
  case ae of
    App f a -> App (compileExpEsc f) (compileExpEsc a)
    Lam x f -> abstractEsc [] x f
    _       -> ae

abstractEsc :: [Ident] -> Ident -> Exp -> Exp
abstractEsc ids x ae =
  case ae of
    Var y | x == y -> scToEsc scI
    Var y | y `elem` ids -> App (scToEsc scK) ae
    App f a -> combineEsc (abstractEsc ids x f) (abstractEsc ids x a)
    Lam y e -> abstractEsc ids x $ argReorder x . etaRewrite $ abstractEsc (x : ids) y e
    -- Esc ar body | ar < 7 -> Esc (ar + 1) (mapExpOnArg (\(Arg i) -> Arg (i + 1)) body)
    _ -> Esc 1 ae

combineEsc :: Exp -> Exp -> Exp
combineEsc a1 a2 =
  let
    (c1, args1) = spine a1
    (c2, args2) = spine a2
  in case (c1, c2) of
    (Esc ar1 bd1, Esc ar2 bd2) -> standardCombine c1 args1 c2 args2









standardCombine :: Exp -> [Exp] -> Exp -> [Exp] -> Exp
standardCombine (Esc ar1 bd1) args1 (Esc ar2 bd2) args2 =
  let
    c = Esc (ar1 + ar2 - 1) (App (mapExpOnArg redirect1 bd1) (mapExpOnArg redirect2 bd2))
    redirect1 a@(Arg i) = if i == ar1 - 1 then Arg (ar1 + ar2 - 2) else a
    redirect2 (Arg i) = Arg (i + length args1)
    in foldl App c (args1 ++ args2)

argReorder :: Ident -> Exp -> Exp
argReorder x = id

etaRewrite :: Exp -> Exp
etaRewrite = id

mapExpOnArg :: (Exp -> Exp) -> Exp -> Exp
mapExpOnArg f = mapExp (onArg f)

onArg :: (Exp -> Exp) -> Exp -> Exp
onArg f a@(Arg _) = f a
onArg _ e       = e

mapExp :: (Exp -> Exp) -> Exp -> Exp
mapExp f (App e1 e2) = App (mapExp f e1) (mapExp f e2)
mapExp f e           = f e
