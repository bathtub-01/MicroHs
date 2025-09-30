module Braun where

import Prelude()
import NanoPrelude

data Tree =
    Branch Int Tree Tree
  | Empty

-- insertTree: <L><1L>
insertTree x Empty = Branch x Empty Empty
insertTree x (Branch y t0 t1) = Branch x (insertTree y t1) t0

-- size: <1L>
size :: Tree -> Int
size Empty = 0
size (Branch x t0 t1) = size t0 + size t1 + 1

-- fromList: <1L>
fromList [] = Empty
fromList (x : xs) = insertTree x (fromList xs)

fromList' [] = Empty
fromList' (x : xs) = l `seq` r `seq` Branch x l r
  where (odds, evens) = unravel xs
        l = fromList' odds
        r = fromList' evens
  
unravel [] = ([], [])
unravel (x : xs) = (x : evens, odds)
  where (odds, evens) = unravel xs

-- toList: <1L>
toList Empty = []
toList (Branch x t0 t1) = x : ilv (toList t0) (toList t1)

-- ilv: <1L><1L>
ilv [] ys = ys
ilv (x : xs) [] = x : xs
ilv (x : xs) (y : ys) = x : y : ilv xs ys

-- equal: <1L><1L>
equal :: [Int] -> [Int] -> Bool
equal [] [] = True
equal [] (y : ys) = False
equal (x : xs) [] = False
equal (x : xs) (y : ys) =
  case (==) x y of
    False -> False
    True -> equal xs ys

-- prop: <SL>
prop xs = equal xs (toList (fromList' xs))

-- int: <1L>
int True = 1::Int
int False = 0::Int

-- Small main
main = int (all prop (replicate 2 (enumFromTo 0 255)))

-- Large main
-- main = int (all prop (replicate 6000 (enumFromTo 0 255)))
