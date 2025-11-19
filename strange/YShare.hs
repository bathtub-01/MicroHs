module YShare where

import Prelude()
import NanoPrelude

-- 1 1 2 3 5 8 13
fib :: Int -> Int
fib n = if n <= 1 then 1 else fib (n-1) + fib (n-2)

inf :: Int -> [Int]
inf n = g
  where
    g = fib n : g
    
inf' :: Int -> [Int]
inf' n = fib n : inf' n

take :: Int -> [a] -> [a]
take 0 _ = []
take n [] = []
take n (x:xs) = x : take (n-1) xs


main = 
  let
    a = fib 6 : b
    b = fib (head a) : c
    c = fib 8 : a
  in fib 13-- all (> 0) (take 20 a)

