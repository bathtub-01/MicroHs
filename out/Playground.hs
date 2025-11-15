module Playground where

import Prelude()
import NanoPrelude

isEven :: Int -> Bool
isEven n =
  let
    isEven' 0 = True
    isEven' i = isOdd' (i - 1)
    isOdd' 0 = False
    isOdd' i = isEven' (i - 1)
  in
    isEven' n
    
    
isEven' 0 = True
isEven' i = isOdd' (i - 1)

isOdd' 0 = False
isOdd' i = isEven' (i - 1)

data TF = T | F

tfAnd :: TF -> TF -> TF
tfAnd T b = b
tfAnd F _ = F

sumPair :: Int -> Int -> Int
sumPair !a !b = a + b

-- sequential thing
lastEle :: [a] -> a
lastEle [] = primitive "error0"
lastEle [n] = n
lastEle (n : ns) = lastEle ns

sumOrMul :: Bool -> Int -> Int -> Int
-- sumOrMul !True !a !b = a + b
-- sumOrMul !False !a !b = a * b
-- sumOrMul cond a b = let body = if cond then a + b else a - b
--                    in seq cond (seq a (seq b body))
sumOrMul cond !a !b = if cond then a + b else a - b


main = 
  let
    a = (1::Int) : b
    b = (2::Int) : c
    c = (3::Int) : a
  in a

--main = let a = sum [1 .. 50]
--           b = a * 2
--           c = sum (map (+ b) [1 .. 10])
--       in a + (b + c)

--let a = sum (map (+ 1) [1 ..6 ])
--	in a + a
	
	
	
