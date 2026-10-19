module Uart where

import Prelude()
import NanoPrelude

main :: IO()
main = echo

echo :: IO ()
echo = do
  (Char i) <- uartGetChar
  uartPutChar (Char (i + 1))
  echo
