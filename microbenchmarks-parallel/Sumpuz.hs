module Sumpuz where

import Prelude()
import NanoPrelude

-- rng: <1L>
rng = map snd

-- img: <1L><L>
img :: [(Int, Int)] -> Int -> Int
img lds l = fromJust (lookup l lds)

-- del: <ML><1L>
del :: Int -> [Int] -> [Int]
del x [] = []
del x (y : ys) =
  if x == y
    then ys
    else y : del x ys

-- diff: <L><1L>
diff = foldl (flip del)

-- con: <L><L>
con x xs = x : xs

-- bindings: <L><1L><SL>
bindings :: Int -> [Int] -> [(Int, Int)] -> [[(Int, Int)]]
bindings l ds lds =
  case lookup l lds of
    Nothing -> map (flip con lds) (zip (repeat l) (diff ds (rng lds)))
    Just d -> if elem d ds then [lds] else []

-- ofAll: <LC(S,L)><1L>
ofAll f [] = []
ofAll f (x : xs) = (f x) ++ (ofAll f xs)

-- ifNull: <1L><ML><ML>
ifNull [] t e = t
ifNull (x : xs) t e = e

-- solutions: <1L><L><1L><ML>
solutions :: [Int] -> [Int] -> [Int] -> (Int, [(Int, Int)]) -> [[(Int, Int)]]
solutions [] yys [] clds =
  if fst clds == 0
    then [snd clds]
    else []
solutions [] yys [z] clds =
  if fst clds == 1
    then bindings z [1] (snd clds)
    else []
solutions (x : xs) yys (z : zs) clds =
  ofAll (solns (fst clds) x (head yys) z (zs `seq` solutions xs (tail yys) zs)) $
  ofAll (bindings (head yys) (enumFromTo (ifNull (tail yys) (1::Int) (0::Int)) (9::Int)))
        (bindings x (enumFromTo (ifNull xs (1::Int) (0::Int)) (9::Int)) (snd clds))        
solutions _ yys _ clds = [] -- ad-hoc fix

-- solns: <ML><L><L><L><LC(S,L)><SL>
solns :: Int -> Int -> Int -> Int -> ((Int, [(Int, Int)]) -> [[(Int, Int)]])
           -> [(Int, Int)] -> [[(Int, Int)]]
solns c x y z f s =
  let qr = divMod10 (img s x + img s y + c)
  in ofAll (curry f (fst qr))
           (bindings z [snd qr] s)

-- divMod10: <1!P(L)>
divMod10 n =
  if n <= (9::Int)
    then (0::Int, n)
    else case divMod10 (n - 10::Int) of
           (q, r) -> (q + 1::Int, r)

-- isSingleton: <1L>
isSingleton :: [[(Int, Int)]] -> Bool
isSingleton []           = False
isSingleton [x]          = True
isSingleton (x : y : ys) = False

-- valid: <SL><SL><L>
valid :: [Int] -> [Int] -> [Int] -> Bool
valid x y z =
  (length x == length y) &&
  (length x == length z) &&
  (isSingleton $ (z `seq` solutions x y z (0::Int, [])))

-- sumMap: <LC(S,L)><1L>
sumMap :: ([Int] -> Int) -> [[Int]] -> Int
sumMap f xs = sumMapAcc f xs 0

-- sumMapAcc: <LC(S,L)><1L><1!P(L)>
sumMapAcc :: ([Int] -> Int) -> [[Int]] -> Int -> Int
sumMapAcc f [] acc = acc
sumMapAcc f (x : xs) acc = sumMapAcc f xs (f x + acc)

-- count: <1L><L><L>
count xs ys zs = sumMap (fx ys zs) xs

-- fx: <1L><L><L>
fx ys zs x = sumMap (fy x zs) ys

-- fy: <L><1L><L>
fy x zs y = sumMap (fz x y) zs

-- fz: <SL><SL><L>
fz x y z = if valid x y z then 1::Int else 0::Int

-- Small main
main :: Int
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
