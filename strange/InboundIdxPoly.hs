module PolySum where

import Prelude ()
import Primitives
import Data.Int
import Data.Bool
import Data.List_Type
import NanoPrelude(sum,enumFromTo,and,zip,tail,foldr,filter,length,enumFrom)

class Num a where
  (+) :: a -> a -> a
  (-) :: a -> a -> a
  (*) :: a -> a -> a
  -- negate :: a -> a
  
--instance Num Int where
--  (+)  = primitive "+"
--  (-)  = primitive "-"
--  (*)  = primitive "*"
  -- negate = primitive "neg"

class Eq a where
  (==) :: a -> a -> Bool
  (/=) :: a -> a -> Bool
  x /= y = if x == y then False else True

instance Eq Int where
  (==) = primitive "=="
  (/=) = primitive "/="

class Eq a => Ord a where
  (<) :: a -> a -> Bool
  (<=) :: a -> a -> Bool
  (>) :: a -> a -> Bool
  (>=) :: a -> a -> Bool
  max :: a -> a -> a
  min :: a -> a -> a

  min x y = if x <= y then x else y
  max x y = if x <= y then y else x

instance Ord Int where
  (<) = primitive "<"
  (<=) = primitive "<="
  (>) = primitive ">"
  (>=) = primitive ">="
  
--sum :: Num a => a -> [a] -> a
--sum z ns = 
--  let
--    go [] acc = acc
--    go (n:ns) acc = go ns (n+acc)
--  in go ns z
  
imax :: Int -> Int -> Int
imax = max

minimum :: Ord a => [a] -> a
minimum [] = primitive "error0"
minimum (x:ys) = foldr (\ y m -> if y < m then y else m) x ys

maximum :: Ord a => [a] -> a
maximum [] = primitive "error0"
maximum (x:ys) = foldr (\ y m -> if y > m then y else m) x ys

--isum :: [Int] -> Int
--isum = sum 0

imaximum :: [Int] -> Int
--imaximum = maximum 0
imaximum = foldr1 imax

foldr1 :: forall a . (a -> a -> a) -> [a] -> a
foldr1 f [] = primitive "error0"
foldr1 f [x] = x
foldr1 f (x:xs) = f x (foldr1 f xs)

--replicate :: Int -> a -> [a]
--replicate n x =
-- if n == 0 then []
--  else x : replicate (n-1) x

map :: forall a b . (a -> b) -> [a] -> [b]
map f =
  let
    rec [] = []
    rec (a : as) = f a : rec as
  in rec
  
quicksort :: Ord a => [a] -> [a]
quicksort [] = []  
quicksort (x:xs) =
    let smaller = [y | y <- xs, y <= x]  
        larger = [y | y <- xs, y > x]   
    in quicksort smaller ++ [x] ++ quicksort larger
    
splitAtPivot :: Ord a => a -> [a] -> ([a],[a])
splitAtPivot p ns =
  let small = [n | n <- ns, n <= p]
      large = [n | n <- ns, n > p]
  in (small, large)
  
inboundIdx :: Ord a => a -> a -> [a] -> [Int]
inboundIdx lower upper ns = [i | (n, i) <- zip ns [0..]
                               , n >= lower
                               , n <= upper]
    
isSorted :: Ord a => [a] -> Bool
isSorted xs = and [x <= y | (x, y) <- zip xs (tail xs)]

-- mini-comb won't share `minimum xs` and `maximum xs`!
nonExtremes :: Ord a => [a] -> Int
nonExtremes xs = length (filter (cond xs) xs) -- [x | x <- xs, x > minimum xs, x < maximum xs]

cond :: Ord a => [a] -> a -> Bool
cond xs x = x > minimum xs && x < maximum xs
  
main :: Int
main = sum (inboundIdx (1::Int) (500::Int) [(1::Int)..50])

