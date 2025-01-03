module Sumpuz where

import Prelude()
import NanoPrelude

rng = map snd

img lds l = fromJust (lookup l lds)

del x [] = []
del x (y : ys) =
  if x == y
    then ys
    else y : del x ys

diff = foldl (flip del)

con x xs = x : xs

bindings l ds lds =
  case lookup l lds of
    Nothing -> map (flip con lds) (zip (repeat l) (diff ds (rng lds)))
    Just d -> if elem d ds then [lds] else []

ofAll f [] = []
ofAll f (x : xs) = (f x) ++ (ofAll f xs)

ifNull [] t e = t
ifNull (x : xs) t e = e

solutions [] yys [] clds =
  if fst clds == 0
    then [snd clds]
    else []

solutions [] yys [z] clds =
  if fst clds == 1
    then bindings z [1] (snd clds)
    else []

solutions (x : xs) yys (z : zs) clds =
  ofAll (solns (fst clds) x (head yys) z (solutions xs (tail yys) zs)) $
  ofAll (bindings (head yys) (enumFromTo (ifNull (tail yys) (1::Int) (0::Int)) (9::Int)))
        (bindings x (enumFromTo (ifNull xs (1::Int) (0::Int)) (9::Int)) (snd clds))
        
solutions _ yys _ clds = [] -- ad-hoc fix

solns c x y z f s =
  let qr = divMod10 (img s x + img s y + c)
  in ofAll (curry f (fst qr))
           (bindings z [snd qr] s)

divMod10 n =
  if n <= (9::Int)
    then (0::Int, n)
    else case divMod10 (n - 10::Int) of
           (q, r) -> (q + 1::Int, r)

isSingleton []           = False
isSingleton [x]          = True
isSingleton (x : y : ys) = False

valid x y z =
  (length x == length y) &&
  (length x == length z) &&
  (isSingleton $ solutions x y z (0::Int, []))

sumMap f xs = sumMapAcc f xs 0

sumMapAcc f [] acc = acc
sumMapAcc f (x : xs) acc = sumMapAcc f xs (f x + acc)

count xs ys zs = sumMap (fx ys zs) xs

fx ys zs x = sumMap (fy x zs) ys

fy x zs y = sumMap (fz x y) zs

fz x y z = if valid x y z then 1::Int else 0::Int

-- Small main
main =
  let words = [[0::Int, 1, 2, 1]-- "BANA"
              ,[2, 1, 2]-- "NAN"
              ]
  in count words words words

{-
-- Large main
wordsOne =
  ["ANANAB"
  ,"ELPPA"
  ,"YRREHC"
  ,"HCAEP"
  ,"TOCIRPA"
  ,"EVILO"
  ]

wordsTwo =
  ["NOMEL"
  ,"AVAUG"
  ,"ODACAVA"
  ,"AYAPAP"
  ,"IHCTIL"
  ,"NOLEM"
  ]

main = let words = wordsOne ++ wordsTwo
       in count words words words
-}
