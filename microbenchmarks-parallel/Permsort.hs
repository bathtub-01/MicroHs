module Permsort where

import Prelude()
import NanoPrelude

-- place: <L><1L>
place x [] = [[x]]
place x (y : ys) = (x : y : ys) : map ((:) y) (place x ys)

-- perm: <1L>
perm [] = [[]]
perm (x : xs) = concatMap (place x) (perm xs)

-- ord: <1L>
ord :: [Int] -> Bool
ord [] = True
ord [x] = True
ord (x : y : ys) = (x <= y) && (ord (y : ys))

-- permSort: <1L>
permSort xs = head (filter ord (perm xs))

-- Small main
main = head (permSort [10,6,7,9,6,12,12])

-- Large main
-- main = head (permSort [10,9,8,7,6,5,4,3,2,1])
