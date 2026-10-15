module Test where

import Prelude()
import NanoPrelude

main :: IO()
main = do
  i <- readAXIReg 13
  writeAXIReg 4 i
