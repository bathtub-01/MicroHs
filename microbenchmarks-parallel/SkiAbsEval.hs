module SkiAbsEval where

import Prelude()
import NanoPrelude
  
data Expr = 
  Idx Int |
  Lam Int Expr |
  App Expr Expr |
  S | K | I | B | C | A | Y |
  Int Int | Add | Eq

mkS :: Expr -> Expr -> Expr
mkS e1 e2 = e2 `seq` case (e1, e2) of
  (App K e1', App K e2') -> App K (App e1' e2')
  (App K e1', e2') -> App (App B e1') e2'
  (e1', App K e2') -> App (App C e1') e2'
  (e1', e2') -> App (App S e1') e2'

compile :: Expr -> Expr
compile (Lam i body) = abstract i (compile body)
compile (App e1 e2) = (App (compile e1) (compile e2))
compile e = e

abstract :: Int -> Expr -> Expr
abstract i e = case e of
  App e1 e2 -> mkS (abstract i e1) (abstract i e2)
  Idx i' | i == i' -> I
  e -> App K e

peel :: Expr -> [Expr]
peel e = let go (App e1 e2) acc = go e1 (e2 : acc)
             go e' acc = e' : acc
         in go e [] 

suture :: [Expr] -> Expr
suture (e : es) = foldl App e es
suture _ = I -- to comfort our compiler

eval :: Expr -> Expr
eval = suture . step . peel

step :: [Expr] -> [Expr]
step (S : e1 : e2 : e3 : stk) = 
  let e2' = eval e2
      e3' = eval e3
  in e2' `seq` e3' `seq` step (peel e1 ++ e3' : App e2' e3' : stk)
step (K : e1 : e2 : stk) = step (peel e1 ++ stk)
step (I : e1 : stk) = step (peel e1 ++ stk)
step (B : e1 : e2 : e3 : stk) = 
  let e2' = eval e2
      e3' = eval e3 
  in e2' `seq` e3' `seq` step (peel e1 ++ App e2' e3' : stk)
step (C : e1 : e2 : e3 : stk) = 
  let e2' = eval e2
      e3' = eval e3 
  in e2' `seq` e3' `seq` step (peel e1 ++ e3' : e2' : stk)
step (A : e1 : e2 : stk) = step (peel e2 ++ stk)
step (Y : e1 : stk) = peel e1 ++ (App Y e1) : stk
step (Add : e1 : e2 : stk) = 
  let i1 = peek (eval e1)
      i2 = peek (eval e2)
  in step (Int (i1 + i2) : stk)
step (Eq : e1 : e2 : stk) = 
  let i1 = peek (eval e1)
      i2 = peek (eval e2)
  in step ((if i1 == i2 then K else A) : stk)
step es = es

peek :: Expr -> Int
peek (Int i) = i
peek _ = 0

drive :: Expr -> Int
drive (Int i) = i
drive e = drive (eval e)

{-
  sumRange :: Int -> Int -> Int
  sumRange x y = if x == y then x else x + (sumRange (x + 1) y)
-}
sumRange = 
  App Y (
    Lam 0 (Lam 1 (Lam 2 (
      App (App (App (App Eq (Idx 1)) (Idx 2)) (Idx 1))
        (App (App Add (Idx 1))
        (App (App (Idx 0) (App (App Add (Idx 1)) (Int 1))) (Idx 2))
        )
      ))
    )
  )

main :: Int
main = drive (App (App (compile sumRange) (Int 1)) (Int 5))
