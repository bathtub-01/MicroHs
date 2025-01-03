module Cichelli where

import Prelude()
import NanoPrelude

-- Allow arithmetic on Char (flite encodes Char as Int directly)
import Data.Char
instance Num Char where
  a + b = chr $ ord a + ord b
  a - b = chr $ ord a - ord b
  a * b = chr $ ord a * ord b
  negate = chr . negate . ord
  abs = chr . abs . ord
  signum = chr . signum . ord
  fromInteger = chr . fromInteger

data Atype a b c =
    H a b c
data Btype a b =
    Hash a b
data Key =
    K String Char Char Int

insert x [] = x : []
insert x (y : ys) =
  if x <= y
    then x : y : ys
    else y : insert x ys

assoc x ((y, z) : yzs) =
  if x == y then z else assoc x yzs

assocm x []             = Nothing
assocm x ((y, z) : yzs) =
  if x == y then Just z else assocm x yzs

subset []       ys = True
subset (x : xs) ys =
  if elem x ys then subset xs ys else False

union xs ys = foldr ins xs ys

ins x ys = if elem x ys then ys else x : ys

histo xs = foldr histins [] xs

histins x []         = [(x, 1)]
histins x ((y,n) : yns) =
    if x == y
      then (y, n + 1) : yns
      else (y,n) : histins x yns

sorted lt = foldr (ordins lt) []

ordins lt x []       = [x]
ordins lt x (y : ys) =
  if lt x y
    then x : y : ys
    else y : ordins lt x ys

ends (K s a z n) = [a,z]

firstLetter (K s a z n) = a

lastLetter (K s a z n) = z

freqSorted ks =
  let ft = freqTabOf ks
  in (sorted (decreasingFrequencyIn ft) ks, length ft)

decreasingFrequencyIn ft (K s0 a x n0) (K s1 b y n1) =
  let freq = flip assoc ft
  in (freq a + freq x) > (freq b + freq y)

freqTabOf ks = histo (concatMap ends ks)

blocked = blockedWith []

blockedWith ds []       = []
blockedWith ds (k : ks) =
  let dsk = union ds (ends k)
      eks = endsSubset dsk
      det = filter eks ks
      rest = filter (non eks) ks
  in k : det ++ blockedWith dsk rest

non f x = if f x then False else True

endsSubset ds k = subset (ends k) ds

enKey k = K k (head k) (last k) (length k)

hashAssoc (Hash hs hf) = hf

findhash mv ks =
  case hashes mv (length ks) ks (Hash (H Nothing Nothing []) []) of
    (Hash s f : hs) -> Just f
    []       -> Nothing

hashes maxval nk []       h = [h]
hashes maxval nk (k : ks) h =
  concatMap (hashes maxval nk ks) $
  concatMap (insertKey nk k)      $
  concatMap (assignUpto maxval (lastLetter k))
            (assignUpto maxval (firstLetter k) h)
  --TODO Indent improvements here?

assignUpto maxval c h =
  case assocm c (hashAssoc h) of
    Nothing -> map (assign c h) (enumFromTo 0 maxval)
    Just v  -> [h]

insertKey nk k (Hash hs hf) =
  case hinsert nk (hash hf k) hs of
    Nothing    -> []
    Just hsNew -> [Hash hsNew hf]

assign c (Hash hs hf) v = Hash hs ((c, v) : hf)

hinsert nk h (H lo hi hs) =
    let newlo = case lo of
                  Nothing -> h
                  Just x  -> min x h
        newhi = case hi of
                  Nothing -> h
                  Just x  -> max x h
    in if elem h hs
         then Nothing
         else if (newhi + 1 - newlo) <= nk
                then Just (H (Just newlo) (Just newhi) (h : hs))
                else Nothing

hash hf (K s a z n) = n + assoc a hf + assoc z hf

cichelli ss =
  case freqSorted (map enKey ss) of
    (ks, mv) -> findhash mv (blocked ks)

emitHashFun [] = 0
emitHashFun ((c, n) : hf) = ord c + n + emitHashFun hf

-- Small main
keywordsOne =
  [ "as"
  , "case"
  , "class"
  , "data"
  , "default"
  , "instance"
  ]
keywordsTwo =
  [ "let"
  , "module"
  , "newtypeg"
  , "of"
  , "qualified"
  , "then"
  , "type"
  , "where"
  ]

main =
   case cichelli (keywordsOne ++ keywordsTwo) of
       Just hf -> emitHashFun hf
       Nothing -> 0

{-
-- Large main

keywordsOne =
  [ "as"
  , "case"
  , "class"
  , "data"
  , "default"
  , "deriving"
  , "do"
  , "else"
  ]

keywordsTwo =
  [ "hiding"
  , "if"
  , "import"
  , "in"
  , "infix"
  , "infixl"
  , "infixr"
  , "instance"
  ]

keywordsThree =
  [ "let"
  , "module"
  , "newtype"
  , "of"
  , "qualified"
  , "then"
  , "type"
  , "where"
  ]

main =
   case cichelli (keywordsOne ++ keywordsTwo ++ keywordsThree) of
       Just hf -> emitHashFun hf
       Nothing -> 0
-}
