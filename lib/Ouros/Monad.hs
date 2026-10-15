-- Minimal Monad class for Ouros / MicroHs with NanoPrelude.
-- No Functor or Applicative superclass is required.
module Ouros.Monad (Monad(..)) where

import Prelude()

infixl 1 >>=, >>

class Monad m where
  (>>=) :: forall a b . m a -> (a -> m b) -> m b
  (>>)  :: forall a b . m a -> m b -> m b
  return :: forall a . a -> m a

  ma >> mb = ma >>= \_ -> mb
