module Ouros.IO where

import Prelude()
import Data.Int
import Ouros.Monad

type World = Int

newtype IO a = IO (World -> (a, World))

runIO :: IO a -> World -> (a, World)
runIO (IO f) = f

instance Monad IO where
  return a = IO (\w -> (a, w))

  (IO m) >>= k = IO (\w ->
    case m w of
      (a, w') -> runIO (k a) w')
                  
-- primitive runtime IO functions (strict on arguments --- they also need to be unboxed at runtime..)

-- `writeAXIReg 3 42` will be compiled to [IO true, INT 3, INT 42]; `IO` is an Atom tag in Ouros
writeAXIReg :: Int -> Int -> IO ()
writeAXIReg = primitive "IO.Write"

-- `readAXIReg 3` will be compiled to [IO false, INT 3]
readAXIReg :: Int -> IO Int
readAXIReg = primitive "IO.Read"
