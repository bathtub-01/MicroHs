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

main :: TF
main = tfAnd T F

--let a = sum (map (+ 1) [1 ..6 ])
--	in a + a
	
	
	
