module Countdown where

import Prelude()
import NanoPrelude

data Op a =
    Add
  | Div
  | Mul
  | Sub
  | Val a
  | App (Op a) (Op a) (Op a)

-- valid: <1L><ML><ML>
valid :: Op Int -> Int -> Int -> Bool
valid Add x y  =  True
valid Sub x y  =  not (x <= y)
valid Mul x y  =  True
valid Div x y  =  mod x y == 0
valid _ _ _ = False

-- apply: <1L><ML><ML>
apply :: Op Int -> Int -> Int -> Int
apply Add x y  =  x + y
apply Sub x y  =  x - y
apply Mul x y  =  mul x y
apply Div x y  =  div x y
apply _ _ _ = 0

-- subs: <1L>
subs []       = [[]]
subs (x : xs) = let yss = subs xs
    in yss ++ map ((:) x) yss

-- interleave: <L><1L>
interleave x []       =  [[x]]
interleave x (y : ys) =
  (x : y : ys) : map ((:) y) (interleave x ys)

-- perms: <1L>
perms []       =  [[]]
perms (x : xs) =  concatMap (interleave x) (perms xs)

-- choices: <1L>
choices xs  =  concatMap perms (subs xs)

-- split: <1L>
split [] = []
split (x : xs) =
  if null xs
    then []
    else ([x], xs) : map (cross (((:) x), id)) (split xs)

-- results: <1L>
results []       =  []
results (n : ns) =
  if null ns
    then [(Val n, n)]
    else concatMap combinedResults (split (n : ns))

-- combinedResults: <1!P(1L,ML)>
combinedResults (ls, rs)  = concatProdWith combine (results ls) (results rs)

-- concatProdWith: <LC(L,C(S,L))><1L><L>
concatProdWith :: ((Op Int, Int) -> (Op Int, Int) -> [(Op Int, Int)]) 
  -> [(Op Int, Int)] -> [(Op Int, Int)] -> [(Op Int, Int)]
concatProdWith f []       ys = []
concatProdWith f (x : xs) ys = concatMap (f x) ys ++ concatProdWith f xs ys

-- combine: <1!P(L,L)><1!P(L,L)>
combine :: (Op Int, Int) -> (Op Int, Int) -> [(Op Int, Int)]
combine (l, x) (r, y) =
  let ops = [Add, Sub, Mul, Div]
  in concatMap (combi l x r y) ops

-- combi: <L><ML><L><ML><1L>
combi :: Op Int -> Int -> Op Int -> Int -> Op Int -> [(Op Int, Int)]
combi l x r y o =
  if valid o x y
    then [(App o l r, apply o x y)]
    else []

-- solutions: <1L><L>
solutions ns n = concatMap (solns n) (choices ns)

-- solns: <ML><1L>
solns n ns = let ems = results ns
             in preImage n (results ns)

-- preImage: <ML><1L>
preImage:: Int -> [(Op Int, Int)] -> [Op Int]
preImage n []             = []
preImage n ((e, m) : ems) =
  if m == n
    then e : preImage n ems
    else preImage n ems

-- mul: <1!P(L)><1!P(1L)>
mul :: Int -> Int -> Int
mul x n =
  if n == 1
    then x
    else case divMod n 2 of
           (d, m) -> mul (x + x) d + (if m == 0 then 0 else x)

-- cross: <1!P(MC(1,L),MC(1,L))><1!P(L,L)>
cross (f, g) (x, y) = (f x, g y)

-- Small main
main =
  let givens = [3::Int,4,10]
      target = 70::Int
  in length (solutions givens target)

{-
-- Large main
main =
  let givens = [1,3,7,10,25]
      target = 765
  in length (solutions givens target)
-}
