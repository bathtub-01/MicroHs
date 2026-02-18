module MicroHs.GenRom where
import Prelude(); import MHSPrelude
import MicroHs.CodeGen

serialise :: [AExp] -> [AExp] -> ([AExp], [AExp])
serialise heap cmb =
  let
    htbl = entryTable heap
    ctbl = entryTable cmb
    walk _ (Com a p) = Com a $ ctbl !! p
    walk _ (Fun p) = Fun $ htbl !! p
    walk n (Ptr p b) | b = Ptr p b
                     | otherwise = Ptr (p + htbl !! n + 1) b
    walk _ a = a
    mapWalk :: [AExp] -> [AExp]
    mapWalk aes = map (\(ae, i) -> map (map (walk i)) ae) (zip aes [0..])
  in (mapWalk heap, mapWalk cmb)

entryTable :: [[a]] -> [Int]
entryTable ass =
  let
    walk :: [[a]] -> Int -> [Int]
    walk [] _ = []
    walk (x:xs) ctr = ctr : walk xs (ctr + length x)
  in walk ass 0
