module MicroHs.GenRomRs where
import Prelude(); import MHSPrelude
import Data.List
import qualified MicroHs.IdentMap as M
import Data.Maybe
import MicroHs.Desugar(LDef)
import MicroHs.Exp
import MicroHs.Expr(Lit(..), showLit, errorMessage, HasLoc(..))
import MicroHs.Ident(Ident(..), showIdent, mkIdent)
import MicroHs.State
import MicroHs.GenRom(getPatNum,inlineSingle,finalEtaApply,freeText)
import MicroHs.CodeGen

-- generate ROM file for the Rust simulator

header :: String
header = "\
  \use crate::hardware::ouros::program::{AluOp, Atom, Program};\n\
  \use std::sync::LazyLock;\n\
  \use AluOp::*;\n\
  \use Atom::*; \n\
  \ \n"

lazyLockProg :: String -> (String -> String) -> (String -> String)
lazyLockProg name r =
  (("#[rustfmt::skip]\npub static " ++ name ++": LazyLock<Program> = LazyLock::new(|| { Program {\n") ++) . r .
  ("\n}});" ++)

indentation :: Int -> String -> (String -> String)
indentation level s = ((replicate (level * 4) ' ' ++ s) ++)

vecS :: (String -> String) -> (String -> String)
vecS r =
  let
    indent = indentation 1
  in indent "vec![\n" . r . indent "]"

-- application
app :: Int -> (String -> String) -> (String -> String)
app offset r =
  let
    indent = indentation 2
  in indent ("vec![ // " ++ show offset ++ " \n") .
     r .
     indent "], \n"

atomIndent :: String -> String -> String
atomIndent = indentation 3

-- atoms
comb :: Int -> Int -> (String -> String)
comb art p =
  atomIndent ("COM(" ++ show art ++ "," ++ show p ++ "," ++ "),\n")

opConvert :: String -> (String, String)
opConvert "==" = ("EQ", "false")
opConvert "/=" = ("EQ", "true")
opConvert "<=" = ("LE", "false")
opConvert ">" = ("LE", "true")
opConvert "<" = ("LT", "false")
opConvert ">=" = ("LT", "true")
opConvert "+" = ("ADD", "false")
opConvert "-" = ("SUB", "false")
opConvert "*" = ("MUL", "false")

ptr :: Int -> Bool -> (String -> String)
ptr n oc = if oc
  then atomIndent ("PTR(" ++ show n ++ ", true, true),\n")
  else atomIndent ("PTR(" ++ show n ++ ", false, false),\n")

arg :: Int -> (String -> String)
arg n = atomIndent ("ARG(" ++ show n ++ "),\n")

int :: Int -> (String -> String)
int n = atomIndent ("INT(" ++ show n ++ "),\n")

prim :: String -> (String -> String)
prim op =
  let (code, rev) = opConvert op
  in atomIndent ("PRM(" ++ code ++ "," ++ rev ++ "),\n")

y :: String -> String
y = atomIndent "Y,\n"

seqStr :: String -> String
seqStr = atomIndent "SEQ(false),\n"

err :: Int -> (String -> String)
err n =
  atomIndent ("ERR(" ++ show n ++ "),\n")

listPrint :: [Int] -> String
listPrint [] = "[]"
listPrint xs = "[" ++ inner ++ "]"
  where
    inner = concat $ zipWith (\x y -> show x ++ y) xs (replicate (length xs - 1) "," ++ [""])

genRomRs :: String -> ([AExp], [AExp]) -> String
genRomRs progName (heap, cmb) =
  let          
    (heap', cmb') = serialise heap cmb
    heapStr = putAExpList heap'
    cmbStr = putAExpList cmb'
  in header
     -- ++ "// Functions in this file: " ++ show funCount ++ "\n"
     -- ++ "// Apps in this file: " ++ show appCount ++ "\n"
     -- ++ "// Combinators in this file: " ++ show combCount ++ "\n"
     ++ lazyLockProg progName (heapStr . cmbStr) ""

serialise :: [AExp] -> [AExp] -> ([AExp], [AExp])
serialise heap cmb =
  let
    htbl = entryTable heap
    ctbl = entryTable cmb
    walk _ (Com a p) = Com a $ ctbl !! p
    walk _ (Fun p) = Fun $ htbl !! p
    walk n (Ptr p b) | b = Ptr p b
                     | otherwise = Ptr (p + htbl !! n) b
    walk _ a = a
    mapWalk :: [AExp] -> [AExp]
    mapWalk aes = map (\(ae, i) -> map (map (walk i)) ae) (zip aes [0..])
  in (mapWalk heap, mapWalk cmb)

entryTable :: [[a]] -> [Int]
entryTable ass =
  let
    walk :: [[a]] -> Int -> [Int]
    walk [] _ = []
    walk (x:xs) ctr = ctr : walk xs (ctr + length x)
  in walk ass 0

putAExpList :: [AExp] -> (String -> String)
putAExpList aes = let
  ((), (_, _, _, r)) = runState (mapM_ putAExp aes) (0, 0, 0, id)
  in vecS r


putAExp :: AExp -> State (Int, Int, Int, String -> String) ()
putAExp ae = do
  mapM_ putApp ae
  (c1, c2, c3, r) <- get
  put (c1 + 1, c2, c3, r)

putApp :: App -> State (Int, Int, Int, String -> String) ()
putApp ap = do
  (c1, c2, c3, r) <- get
  put (c1, c2, c3, id)
  mapM_ putAtom ap
  (c1', c2', c3', r') <- get
  put (c1', c2' + 1, c3', r . vecS r')

putAtom :: Atom -> State (Int, Int, Int, String -> String) ()
putAtom atm =
  let
    atom (Prm s) = y
    atom (Int i) = int i
    atom (Com a p) = comb a p
    atom (Fun p) = ptr p False
    atom (Ptr p hc) = ptr p hc
    atom (Apt p) = arg p
  in do
  (c1, c2, c3, r) <- get  
  put (c1, c2, c3 + 1, r . atom atm)

-- atom :: Exp -> (String -> String)
-- atom ae =
--   case ae of
--     Var i -> if "PTR" `isPrefixOf` showIdent i then ptr $ read (drop 3 (showIdent i))
--                else error "Strange Var exists."
--     Lit (LInt i) -> int i
--     Lit (LPrim "Y") -> y
--     Lit (LPrim "seq") -> seqStr
--     Lit (LPrim op) -> if "error" `isPrefixOf` op then err $ read (drop 5 op)
--                         else prim op
--     Lit _ -> error "Strange Lit exists."
--     Sc a p is -> comb a p is
--     _ -> error "Not an Atom."
