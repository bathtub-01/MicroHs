module Clausify where

import Prelude()
import NanoPrelude

data Expr a
  = Con (Expr a) (Expr a)
  | Dis (Expr a) (Expr a)
  | Neg (Expr a)
  | Sym a

insert x [] = x : []
insert x (y : ys) =
  if x <= y
    then x : y : ys
    else y : insert x ys

clauses ps = map (clause ([], [])) ps

clause (c, a) (Dis p q)     = clause (clause (c, a) p) q
clause (c, a) (Sym s)       = (insert s c, a)
clause (c, a) (Neg (Sym s)) = (c, insert s a)
clause _ _ = ([], [])

contains eq [] y = False
contains eq (x : xs) y = eq x y || contains eq xs y

disin (Sym s) = Sym s
disin (Neg p) = Neg p
disin (Con p q) = Con (disin p) (disin q)
disin (Dis p q) = din (disin p) (disin q)

din (Con p q) r = Con (din p r) (din q r)
din (Dis p q) r = din2 (Dis p q) r
din (Neg p) r = din2 (Neg p) r
din (Sym s) r = din2 (Sym s) r

din2 p (Con q r) = Con (din p q) (din p r)
din2 p (Dis q r) = Dis p (Dis q r)
din2 p (Neg q) = Dis p (Neg q)
din2 p (Sym s) = Dis p (Sym s)

inter eq xs ys = filter (contains eq xs) ys

negin (Neg (Con p q)) = Dis (negin (Neg p)) (negin (Neg q))
negin (Neg (Dis p q)) = Con (negin (Neg p)) (negin (Neg q))
negin (Neg (Neg p))   = negin p
negin (Neg (Sym s))   = Neg (Sym s)
negin (Dis p q)       = Dis (negin p) (negin q)
negin (Con p q)       = Con (negin p) (negin q)
negin (Sym s)         = Sym s

nonTaut cs = filter notTaut cs

eqList f [] [] = True
eqList f [] (y : ys) = False
eqList f (x : xs) [] = False
eqList f (x : xs) (y : ys) = f x y && eqList f xs ys

eq a b = a == b

eqClause (a, b) (c, d) = eqList eq a c && eqList eq b d

notTaut (c, a) = null (inter eq c a)

clausify p = uniq    $
             nonTaut $
             clauses $
             split   $
             disin   $
             negin p

split p = spl [] p

spl a (Con p q) = spl (spl a p) q
spl a (Dis p q) = Dis p q : a
spl a (Neg p) = Neg p : a
spl a (Sym s) = Sym s : a

comp f g x = f (g x)

union eq xs ys = xs ++ filter (comp not (contains eq xs)) ys

singleton x = x : []

uniq xs = foldr (comp (union eqClause) singleton) [] xs

display [] = 0::Int
display (c : cs) = emitClause c + display cs

emitClause (c, a) = sum c + sum a

eqv a b = Con (Dis (Neg a) b) (Dis (Neg b) a)

-- Small main
main = --let p = eqv (eqv a $ eqv a a)
         --          (eqv a $ eqv a a)
           --a = Sym (0::Int)
       --in display $ clausify $ foldr Con a $ replicate 2 p -- TODO: strange, check why the compiler does not apply this
       display $ clausify $ foldr Con (Sym (0::Int)) $ replicate 2 (eqv (eqv (Sym (0::Int)) $ eqv (Sym (0::Int)) (Sym (0::Int))) (eqv (Sym (0::Int)) $ eqv (Sym (0::Int)) (Sym (0::Int))))

{-
-- Large main
main = let p = eqv (eqv a $ eqv a a) $
                    eqv (eqv a $ eqv a a)
                        (eqv a $ eqv a a)
           a = Sym 0
       in display $ clausify $ foldr Con a $ replicate 80 p
-}
