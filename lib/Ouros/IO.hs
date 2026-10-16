module Ouros.IO where

import Prelude()
import Data.Int
import Ouros.Monad
import Ouros.Char_String

type World = Int

newtype IO a = IO (World -> (a, World))

runIO :: IO a -> World -> (a, World)
runIO (IO f) = f

instance Monad IO where
  return a = IO (\w -> (a, w))

  (IO m) >>= k = IO (\w ->
    case m w of
      (a, w') -> runIO (k a) w')
                  
-- primitive IO functions (Ouros enforces strictness and unboxing of arguments at runtime)

-- `writeAXIReg 3 42` will be compiled to [IO true, INT 3, INT 42]; `IO` is an Atom tag in Ouros
writeAXIReg :: Int -> Int -> IO ()
writeAXIReg = primitive "IO.Write"

-- `readAXIReg 3` will be compiled to [IO false, INT 3]
readAXIReg :: Int -> IO Int
readAXIReg = primitive "IO.Read"

-- more drivers to be added...

-- UART (32-bit AXI4-Lite byte addresses):
--   0x00: RX_DATA -- blocks until a byte is available, then consumes it.
--   0x04: TX_DATA -- transmits the low eight bits of the value.
-- No polling is needed for this simulator's blocking RX / ready TX.

uartGetChar :: IO Char
uartGetChar =
  readAXIReg 0 >>= \byte ->
  return (Char byte)

uartPutChar :: Char -> IO ()
uartPutChar (Char byte) = writeAXIReg 4 byte
