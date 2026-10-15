module Ouros.IO where

import Prelude()
import Data.Int
import Ouros.Monad

type World = Int

newtype IO a = IO (World -> (a, World))

runRW :: IO a -> World -> (a, World)
runRW (IO f) = f

instance Monad IO where
  return a = IO (\w -> (a, w))

  (IO m) >>= k = IO (\w ->
    case m w of
      (a, w') -> runRW (k a) w')
                  
-- primitive runtime IO functions (strict on arguments --- they also need to be unboxed at runtime..)

writeAXIReg :: Int -> Int -> IO ()
writeAXIReg !addr !word = IO (\w -> ((), w))

readAXIReg :: Int -> IO Int
readAXIReg !addr = IO(\w -> (42::Int, w))
