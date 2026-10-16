module Uart where

import Prelude()
import NanoPrelude

main :: IO()
main = echo

echo :: IO ()
echo = do
  c <- uartGetChar
  uartPutChar c
  echo
