module MicroHs.AbstractESC(
  compileEsc,
  scToEsc
  ) where
import Prelude(); import MHSPrelude
import MicroHs.Ident
import MicroHs.Exp
import MicroHs.Expr(Lit(..))
import MicroHs.State
import Data.List

-- compileOpt: remove all lambdas
-- print program imgs (heap + reducer); translate remaining SCs into ESCs

isPrim :: String -> Exp -> Bool
isPrim s ae =
  case ae of
    Lit (LPrim ss) -> s == ss
    _       -> False

-- example = Lam (mkIdent "x") (Lam (mkIdent "y") (Lam (mkIdent "z") (App (App (App (App (Var (mkIdent "x")) (Lit (LPrim "*"))) (Var (mkIdent "x"))) (Lit (LPrim "+"))) (App (App (Var (mkIdent "y")) (Lit (LPrim "*"))) (Var (mkIdent "z"))))))

-- example1 = Lam (mkIdent "z") (App (App (App (App (Var (mkIdent "x")) (Lit (LPrim "*"))) (Var (mkIdent "x"))) (Lit (LPrim "+"))) (App (App (Var (mkIdent "y")) (Lit (LPrim "*"))) (Var (mkIdent "z"))))

-- example2 = Lam (mkIdent "y") (Lam (mkIdent "z") (App (App (App (App (Var (mkIdent "x")) (Lit (LPrim "*"))) (Var (mkIdent "x"))) (Lit (LPrim "+"))) (App (App (Var (mkIdent "y")) (Lit (LPrim "*"))) (Var (mkIdent "z")))))

-- exampleBig = Lam (mkIdent "a") (App (Var (mkIdent "a")) (App (Lam (mkIdent "y") (Lam (mkIdent "z") (App (App (App (App (Var (mkIdent "x")) (Lit (LPrim "*"))) (Var (mkIdent "a"))) (Lit (LPrim "+"))) (App (App (Var (mkIdent "y")) (Lit (LPrim "*"))) (Var (mkIdent "z")))))) (Var (mkIdent "a"))))

cons = lams
  [mkIdent "x", mkIdent "y", mkIdent "z", mkIdent "f"]
  (apps (Var (mkIdent "f")) [Var (mkIdent "x"), Var (mkIdent "y")])

cons' = lams
  [mkIdent "f"]
  (apps (Var (mkIdent "f")) [Var (mkIdent "x"), Var (mkIdent "y")])

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
compileEsc = etaRewrite . compileExpEsc . removeSKI

compileExpEsc :: Exp -> Exp
compileExpEsc ae =
  case ae of
    App f a -> App (compileExpEsc f) (compileExpEsc a)
    Lam x f -> abstractEsc [] x f
    _       -> ae

escI = scToEsc scI
escK = scToEsc scK
escS = scToEsc scS
escC = scToEsc scC
escB = scToEsc scB

abstractEsc :: [Ident] -> Ident -> Exp -> Exp
abstractEsc ids x ae =
  case ae of
    Var y | x == y -> escI
    Var y | y `elem` ids -> App escK ae
    App f a -> combineEsc (abstractEsc ids x f) (abstractEsc ids x a)
    Lam y e -> abstractEsc ids x $ argReorder x . etaRewrite $ abstractEsc (x : ids) y e
    Esc ar body-> Esc (ar + 1) (mapExpOnArg (\(Arg i) -> Arg (i + 1)) body)
    _ -> Esc 1 ae

combineEsc :: Exp -> Exp -> Exp
combineEsc a1 a2 =
  let
    (c1, args1) = spine a1
    (c2, args2) = spine a2
  in case (c1, c2) of
    (Esc ar1 bd1, Esc ar2 bd2) ->
      if a1IsUnary && a2IsUnary then
        standardCombine c1 args1 c2 args2
      else if a1IsUnary then
        if xNotUsed args2 is2 then
          let c = Esc (ar1 + 1) (App bd1' (Arg (ar1 - 1)))
              bd1' = mapExpOnArg (\(Arg i) -> if i == ar1 - 1 then Arg ar1 else Arg i) bd1
          in App (foldl App c args1) (etaRewrite a2Old)
        else
          let c = Esc (ar1 + 1) (App bd1' (App (Arg (ar1 - 1)) (Arg ar1)))
              bd1' = mapExpOnArg (\(Arg i) -> if i == ar1 - 1 then Arg ar1 else Arg i) bd1
          in foldl App c (args1 ++ [etaRewrite a2])
      else if a2IsUnary &&
              (length (filter (== length args1 + 1) is1) <= 1 || a2 == escI) then
        let
          
        in undefined
      else addEsc c1 args1 c2 args2
      where
        a1IsUnary = ar1 == length args1 + 1
        a2IsUnary = ar2 == length args2 + 1
        is1 = pullout bd1
        is2 = pullout bd2
        a2Old = discardAbs c2 args2

xNotUsed :: [Exp] -> [Int] -> Bool
xNotUsed args = notElem (length args)

discardAbs :: Exp -> [Exp] -> Exp
discardAbs (Esc ar body) args =
  let
    redirect (Arg i)
      | i < length args = Arg i
      | i > length args = Arg $ i - 1
      | otherwise = error "should not discard"
    c' = Esc (ar - 1) (mapExpOnArg redirect body)
    in foldl App c' args

addEsc :: Exp -> [Exp] -> Exp -> [Exp] -> Exp
addEsc c1 args1 c2 args2 =
  case (c1, c2) of
    (Esc _ bd1, Esc _ bd2) ->
      let
        is1 = pullout bd1
        is2 = pullout bd2
        a1NotUsed = notElem (length args1) is1
        a2NotUsed = notElem (length args2) is2
        a1Old = etaRewrite $ discardAbs c1 args1
        a2Old = etaRewrite $ discardAbs c2 args2
        a1Eta = etaRewrite $ fromSpine (c1, args1) 
        a2Eta = etaRewrite $ fromSpine (c2, args2)
      in if a1NotUsed && a2NotUsed
         then App escK (App a1Old a2Old)
         else if a1NotUsed
         then app2 escB a1Old a2Eta
         else if a2NotUsed
         then app2 escC a1Eta a2Old
         else app2 escS a1Eta a2Eta
    _ -> undefined

standardCombine :: Exp -> [Exp] -> Exp -> [Exp] -> Exp
standardCombine (Esc ar1 bd1) args1 (Esc ar2 bd2) args2 =
  let
    c = Esc (ar1 + ar2 - 1) (App (mapExpOnArg redirect1 bd1) (mapExpOnArg redirect2 bd2))
    redirect1 a@(Arg i) = if i == ar1 - 1 then Arg (ar1 + ar2 - 2) else a
    redirect2 (Arg i) = Arg (i + length args1)
    in foldl App c (args1 ++ args2)

argReorder :: Ident -> Exp -> Exp
argReorder x ae =
  case ae of
    App _ _ ->
      let (c, args) = spine ae in
        case c of
          Esc ar body ->
            -- 1. compress the same args (beware args might be longer than ar)
            -- 2. reorder x to the last position
            let
              is = pullout body
              edibleArgs = take ar args
              (is', args') = argCompress is edibleArgs
              ar' = ar - (length edibleArgs - length args')
              moveToEnd _ [] = []
              moveToEnd x (y:ys)
                | x == y = ys ++ [x]
                | otherwise = y : moveToEnd x ys
              args'' = moveToEnd (Var x) args' 
              xIdx = findIndex (== (Var x)) args'
              is'' = case xIdx of
                Just i ->
                  map (\i' -> if i' == i then length args'' - 1
                        else if i' > i && i' <= length args'' - 1 then i' - 1
                        else i') is'
                Nothing -> is'
              body' = refill body is''
              c' = Esc ar' body'
            in fromSpine (c', map (argReorder x) args'' ++ drop ar args)
          _ -> fromSpine (c, map (argReorder x) args)
    _ -> ae

argCompress :: [Int] -> [Exp] -> ([Int], [Exp])
argCompress is args =
  case dupPair args of
    (es, Just(i', i)) ->
      argCompress (map (\idx -> if idx == i' then i else if idx > i' then idx - 1 else idx) is) es -- fixed point
    (_, Nothing) -> (is, args)

-- find a duplicated pair and remove it
dupPair :: [Exp] -> ([Exp], Maybe (Int, Int))
dupPair args =
  case dup of
    Just (i', i) -> (removeNth i' args, dup)
    Nothing -> (args, dup)
  where
    dup = go args 0
    --                          (from, to)
    go :: [Exp] -> Int -> Maybe (Int, Int)
    go [] _ = Nothing
    go (e:es) i = case elemIndex e es of
      Just i' -> Just (i' + i + 1, i)
      Nothing -> go es (i + 1)
    removeNth n xs = take n xs ++ drop (n + 1) xs

-- pullout the list of arg pointers in this body
pullout :: Exp -> [Int]
pullout e =
  let
    pullout' :: Exp -> ([Int] -> [Int])
    pullout' (App e1 e2) = pullout' e1 . pullout' e2
    pullout' (Arg i) = (i:)
    pullout' _ = id
  in pullout' e []

refill :: Exp -> [Int] -> Exp
refill e idxs =
  let
    refill' :: Exp -> State [Int] Exp
    refill' (App e1 e2) = do
      e1' <- refill' e1
      e2' <- refill' e2
      return (App e1' e2')
    refill' (Arg _) = do
      (i:is) <- get
      put is
      return (Arg i)
    refill' e' = return e'
    (res, _) = runState (refill' e) idxs
  in res

etaRewrite :: Exp -> Exp 
etaRewrite = etaApply . etaShrink

etaShrink :: Exp -> Exp
etaShrink (App e1 e2) = App (etaShrink e1) (etaShrink e2)
etaShrink (Esc ar body) =
  let
    shrink :: Int -> Exp -> [Int] -> (Int, Exp)
    shrink a b is =
      if isOnlyLast (a - 1) is && smallTail b
      then shrink (a - 1) (stripTail b) (init is)
      else (a, b)
    isOnlyLast :: Int -> [Int] -> Bool
    isOnlyLast _ [] = False
    isOnlyLast x xs = last xs == x && count x xs == 1
      where count n = length . filter (== n)
    smallTail (App _ (Arg _)) = True
    smallTail _               = False
    stripTail (App e1 _) = e1
    idxs = pullout body
    (ra, rb) = shrink ar body idxs
  in Esc ra rb
etaShrink e = e

etaApply :: Exp -> Exp
etaApply ae =
  case ae of
    App _ _ ->
      let
        (c, args) = spine ae
      in
        case c of
          Esc ar body ->
            if ar <= length args && safeToApply 
            then
              etaApply (fromSpine (apply body (take ar etaArgs), drop ar etaArgs)) -- fixed-point recursion
            else fromSpine (c, etaArgs)
            where
              idxs = pullout body
              safeToApply = noDuplicates idxs
              etaArgs = map etaApply args
              noDuplicates [] = True
              noDuplicates (x:xs) = x `notElem` xs && noDuplicates xs
          _ -> fromSpine (c, map etaApply args)
    Esc 0 body -> body
    _ -> ae

apply :: Exp -> [Exp] -> Exp
apply bd args =
  mapExpOnArg sub bd
  where sub (Arg i) = args !! i

mapExpOnArg :: (Exp -> Exp) -> Exp -> Exp
mapExpOnArg f = mapExp (onArg f)

onArg :: (Exp -> Exp) -> Exp -> Exp
onArg f a@(Arg _) = f a
onArg _ e       = e

mapExp :: (Exp -> Exp) -> Exp -> Exp
mapExp f (App e1 e2) = App (mapExp f e1) (mapExp f e2)
mapExp f e           = f e
