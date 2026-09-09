module MicroHs.GenRomRs(genRomRs) where
import Prelude(); import MHSPrelude
import Data.List
import MicroHs.State
import MicroHs.CodeGen
import MicroHs.GenRom

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

vecS :: String -> (String -> String) -> (String -> String)
vecS n r =
  let
    indent = indentation 1
  in indent (n ++ ": vec![\n") . r . indent "],\n"

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
  atomIndent ("Com(" ++ show art ++ "," ++ show p ++ "),\n")

opConvert :: String -> (String, String)
opConvert "==" = ("EQ", "false")
opConvert "/=" = ("EQ", "true")
opConvert "<=" = ("LE", "false")
opConvert ">" = ("LE", "true")
opConvert "<" = ("LT", "false")
opConvert ">=" = ("LT", "true")
opConvert "+" = ("Add", "false")
opConvert "-" = ("Sub", "false")
opConvert "*" = ("Mul", "false")

ptr :: Int -> Bool -> (String -> String)
ptr n oc = if oc
  then atomIndent ("Ptr(" ++ show n ++ ", true, true),\n")
  else atomIndent ("Ptr(" ++ show n ++ ", false, false),\n")

arg :: Int -> Bool -> (String -> String)
arg n True = atomIndent ("Arg(" ++ show n ++ ", true),\n")
arg n False = atomIndent ("Arg(" ++ show n ++ ", false),\n")

int :: Int -> (String -> String)
int n = atomIndent ("Int(" ++ show n ++ "),\n")

prim :: String -> (String -> String)
prim op =
  let (code, rev) = opConvert op
  in atomIndent ("Prm(" ++ code ++ "," ++ rev ++ "),\n")

y :: String -> String
y = atomIndent "Y,\n"

seqStr :: String -> String
seqStr = atomIndent "Seq,\n"

err :: Int -> (String -> String)
err n =
  atomIndent ("Err(" ++ show n ++ "),\n")

genRomRs :: String -> ([AExp], [AExp]) -> String
genRomRs progName (heap, cmb) =
  let          
    (heap', cmb') = serialise heap cmb
    heapStr = putAExpList "heap_img" heap'
    cmbStr = putAExpList "comb_img" cmb'
  in header
     -- ++ "// Functions in this file: " ++ show funCount ++ "\n"
     -- ++ "// Apps in this file: " ++ show appCount ++ "\n"
     ++ "// Combinators in this file: " ++ show (length cmb) ++ "\n"
     ++ lazyLockProg progName (heapStr . cmbStr) ""

putAExpList :: String -> [AExp] -> (String -> String)
putAExpList n aes = let
  ((), (_, _, r)) = runState (mapM_ putAExp aes) (0, 0, id)
  in vecS n r

-- state: 1. aexp counter; 2. app counter; 3. result string
putAExp :: AExp -> State (Int, Int, String -> String) ()
putAExp ae = do
  (c1, c2, r) <- get
  let comment = indentation 2 $ "// AExp" ++ show c1 ++ "\n"
  put (c1 + 1, c2, r . comment)
  mapM_ putApp ae

putApp :: App -> State (Int, Int, String -> String) ()
putApp ap = do
  (c1, c2, r) <- get
  put (c1, c2, id)
  mapM_ putAtom ap
  (c1', c2', r') <- get
  put (c1', c2' + 1, r . app c2' r')

putAtom :: Atom -> State (Int, Int, String -> String) ()
putAtom atm =
  let
    atom (Prm "Y") = y
    atom (Prm "seq") = seqStr
    atom (Prm op) = if "error" `isPrefixOf` op
                      then err $ read (drop 5 op)
                      else prim op
    atom (Int i) = int i
    atom (Com a p) = comb a p
    atom (Fun p) = ptr p False
    atom (Ptr p hc) = ptr p hc
    atom (Apt p unq) = arg p unq
  in do
  (c1, c2, r) <- get  
  put (c1, c2, r . atom atm)
