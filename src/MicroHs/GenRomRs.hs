module MicroHs.GenRomRs where
import Prelude(); import MHSPrelude
import Data.List
import MicroHs.State
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
  atomIndent ("COM(" ++ show art ++ "," ++ show p ++ "),\n")

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

serialise :: [AExp] -> [AExp] -> ([AExp], [AExp])
serialise heap cmb =
  let
    htbl = entryTable heap
    ctbl = entryTable cmb
    walk _ (Com a p) = Com a $ ctbl !! p
    walk _ (Fun p) = Fun $ htbl !! p
    walk n (Ptr p b) | b = Ptr p b
                     | otherwise = Ptr (p + htbl !! n + 1) b
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
    atom (Apt p) = arg p
  in do
  (c1, c2, r) <- get  
  put (c1, c2, r . atom atm)
