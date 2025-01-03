module While where

import Prelude()
import NanoPrelude

type Var = Int

data Arith
  = Add Arith Arith
  | N Int
  | Sub Arith Arith
  | V Var

data Bools
  = And Bools Bools
  | Eq Arith Arith
  | FALSE
  | Le Arith Arith
  | Neg Bools
  | TRUE

data Statement
  = Ass Var Arith
  | Comp Statement Statement
  | If Bools Statement Statement
  | Skip
  | While Bools Statement

data State
  = Final [(Var, Int)]
  | Inter Statement [(Var, Int)]

value ((x, y) : s) v k =
  if x == v
    then k y
    else value s v k
value [] v k = primitive "error42" -- ad-hoc fix

update [] v k i = k []
update ((x, y) : s) v k i =
  if x == v
    then update s v (upd k v i) i
    else update s v (upd k x y) i

upd k x y s = k ((x, y) : s)

int n k = if n == (0::Int) then k (0::Int) else k n

bool False k = k False
bool True k  = k True

add  k a b = int  (a + b  ) k
sub  k a b = int  (a - b  ) k
eq   k a b = bool (a == b ) k
leq  k a b = bool (a <= b ) k
notk k a   = bool (not a  ) k
andk k a b = bool (a && b) k

seqq f g k = f (comp g k)
comp f g x = f (g x)

aval (N n) s k = k n
aval (V x) s k = value s x k
aval (Add a1 a2) s k = seqq (aval a1 s) (aval a2 s) (add k)
aval (Sub a1 a2) s k = seqq (aval a1 s) (aval a2 s) (sub k)

bval TRUE s k = k True
bval FALSE s k = k False
bval (Eq a1 a2) s k = seqq (aval a1 s) (aval a2 s) (eq k)
bval (Le a1 a2) s k = seqq (aval a1 s) (aval a2 s) (leq k)
bval (Neg b) s k = bval b s (notk k)
bval (And a1 a2) s k = seqq (bval a1 s) (bval a2 s) (andk k)

sosstm (Ass x a) s = aval a s (update s x Final)
sosstm Skip s = Final s
sosstm (Comp ss1 ss2) s =
  case sosstm ss1 s of
    Inter ss10 s0 -> Inter (Comp ss10 ss2) s0
    Final s0 -> Inter ss2 s0

sosstm (If b ss1 ss2) s = bval b s (cond s ss1 ss2)
sosstm (While b ss) s =
  Inter (If b (Comp ss (While b ss)) Skip) s

cond s ss1 ss2 c = if c then Inter ss1 s else Inter ss2 s

run (Inter ss s) = run (sosstm ss s)
run (Final s) = s

ssos ss s = run (Inter ss s)

-- Small main
main =
  let
    divide = While (Le (V 1) (V 0)) $
             Comp (Ass 0 (Sub (V 0) (V 1)))
                  (Ass 2 (Add (V 2) (N 1)))

    callDivide = Comp (Comp (Ass 0 (V 3))
                            (Ass 1 (V 4)))
                      divide

    ndivs = Comp (Ass 4 (V 3)) $
            While (Neg (Eq (V 4) (N 0))) $
            Comp (Comp callDivide $
                  If (Eq (V 0) (N 0))
                     (Ass 5 (Add (V 5) (N 1)))
                     Skip)
                 (Ass 4 (Sub (V 4) (N 1)))
                 
    sinit =  [(0::Int, 0::Int)
             ,(1::Int, 0::Int)
             ,(2::Int, 0::Int)
             ,(3::Int, 17::Int)
             ,(4::Int, 0::Int)
             ,(5::Int, 0::Int)
             ]

  in value (ssos ndivs sinit) 5 id

{-
-- Large main
main =
  let divide = While (Le (V 1) (V 0)) $
               Comp (Ass 0 (Sub (V 0) (V 1)))
                    (Ass 2 (Add (V 2) (N 1)))

      callDivide = Comp (Comp (Ass 0 (V 3))
                              (Ass 1 (V 4)))
                        divide

      ndivs = Comp (Ass 4 (V 3)) $
              While (Neg (Eq (V 4) (N 0))) $
              Comp (Comp callDivide $
                    If (Eq (V 0) (N 0))
                       (Ass 5 (Add (V 5) (N 1)))
                       Skip)
                   (Ass 4 (Sub (V 4) (N 1)))

      sinit =  [(0, 0)
               ,(1, 0)
               ,(2, 0)
               ,(3, 14000)
               ,(4, 0)
               ,(5, 0)
               ]
  in value (ssos ndivs sinit) 5 id
-}
