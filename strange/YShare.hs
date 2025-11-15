module YShare where

import Prelude()
import NanoPrelude

fib :: Int -> Int
fib n = if n <= 1 then 1 else fib (n-1) + fib (n-2)

inf :: Int -> [Int]
inf n = g
  where
    g = fib n : g

take :: Int -> [a] -> [a]
take 0 _ = []
take n [] = []
take n (x:xs) = x : take (n-1) xs


main = all (> 0) (take 10 (inf 10))

