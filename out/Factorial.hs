module Factorial where

import Prelude()
import NanoPrelude

fact :: Int -> Int
fact 0 = 1
fact n = n * fact (n - 1)

main :: Int
main = fact 5
