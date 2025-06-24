module InboundIdxPlain where

import Prelude()
import NanoPrelude
  
inboundIdx :: Int -> Int -> [Int] -> [Int]
inboundIdx lower upper ns = [i | (n, i) <- zip ns [0..]
                               , n >= lower
                               , n <= upper]

main :: Int
main = sum (inboundIdx (1::Int) (500::Int) [(1::Int)..50])
