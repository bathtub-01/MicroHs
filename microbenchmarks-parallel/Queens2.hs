module Queens2 where

import Prelude()
import NanoPrelude

-- one: <LC(S,L)><1L>
one :: (Int -> Bool) -> [Int] -> [Int]
one p [] = []
one p (x : xs) = if p x then [x] else one p xs

l = 0::Int
r = 1::Int
d = 2::Int

-- eq: <1!P(L)><1!P(L)>
eq :: Int -> Int -> Bool
eq x y = x == y

-- left: <1L>
left  xs = map (one (eq l)) (tail xs)

-- right: <ML>
right xs = [] : map (one (eq r)) xs

-- down: <1L>
down  xs = map (one (eq d)) xs

-- merge: <1L><ML>
merge :: [[Int]] -> [[Int]] -> [[Int]]
merge [] ys = []
merge (x : xs) [] = x : xs
merge (x : xs) (y : ys) = (x ++ y) : merge xs ys

-- next: <SL>
next :: [[Int]] -> [[Int]]
next mask = merge (merge (down mask) (left mask)) (right mask)

-- fill: <1L>
fill :: [[Int]] -> [[[Int]]]
fill [] = []
fill (x : xs) = (lrd x xs) ++ (map ((:) x) (fill xs))

-- lrd: <1L><L>
lrd :: [Int] -> [[Int]] -> [[[Int]]]
lrd [] ys = [[l,r,d] : ys]
lrd (x : xs) ys = []

-- solve: <1!P(1L)><ML>
solve :: Int -> [[Int]] -> [[[[Int]]]]
solve n mask =
  if n == 0
    then [[]]
    else concatMap (sol (n - 1)) (fill mask)

-- sol: <1!P(1L)><L>
sol :: Int -> [[Int]] -> [[[[Int]]]]
sol n row = map ((:) row) (solve n (next row))

-- nqueens: <1!P(SL)>
nqueens n = length (solve n (replicate n []))

-- Small main
main = nqueens 5

-- Large main
-- main = nqueens 11
