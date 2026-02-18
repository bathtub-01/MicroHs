module MicroHs.GenRomOScala(genRomOScala) where
import Prelude(); import MHSPrelude
import Data.List
import MicroHs.State
import MicroHs.CodeGen
import MicroHs.GenRom

-- generate Chisel ROM file

header :: String
header = "\
 \package benchmarks\n\
 \import common.Helper._\n\
 \import common.Atom\n\
 \import chisel3.Vec\n\
 \ \n"

object :: String -> (String -> String) -> (String -> String)
object name r =
  (("object " ++ name ++ " extends Benchmark {\n") ++) . r . ("}" ++)

defToString :: String -> (String -> String)
defToString name =
  ((("override def toString() = \"" ++ name) ++ "\" \n") ++)

freeText :: String -> (String -> String)
freeText t = (t ++)

val :: String -> (String -> String) -> (String -> String)
val name r =
  (("val " ++ name ++ " = ") ++) . r 

img :: String -> (String -> String) -> (String -> String)
img name r = val name (("Seq(\n" ++) . r . (")\n" ++)) 

-- spine application
app :: Int -> (String -> String) -> (String -> String)
app offset r = ("appBuilder( // " ++) . (show offset ++) . ("\n" ++) . r . ("),\n" ++)

-- atoms
comb :: Int -> Int -> (String -> String)
comb art p = ("comBuilder(" ++) .
                ((show art ++ ",") ++) .
                ((show p ++ "),\n") ++)

ptr :: Int -> Bool -> (String -> String)
ptr n True = ("ptrBuilder(" ++) . (show n ++) . (", true, true),\n" ++)
ptr n False = ("ptrBuilder(" ++) . (show n ++) . (", false, false),\n" ++)

int :: Int -> (String -> String)
int n = ("intBuilder(" ++) . (show n ++) . ("),\n" ++)

prim :: String -> (String -> String)
prim op = ("prmBuilder(\"" ++) . (op ++) . ("\"),\n" ++)

arg :: Int -> Bool -> (String -> String)
arg n True = (("argBuilder(" ++ show n ++ ", true),\n") ++)
arg n False = (("argBuilder(" ++ show n ++ ", false),\n") ++)

y :: (String -> String)
y = ("yBuilder(),\n" ++)

err :: Int -> (String -> String)
err code = ("errorBuilder(" ++) . (show code ++) . ("),\n" ++)

genRomOScala :: String -> ([AExp], [AExp]) -> String
genRomOScala progName (heap, cmb) =
  let
    (heap', cmb') = serialise heap cmb
    heapStr = putAExpList "heap_img" heap'
    cmbStr = putAExpList "comb_img" cmb'
  in header
     ++ object progName (defToString progName .
                         val "combinatorCount" (freeText (show (length cmb) ++ "\n")) .
                         heapStr . cmbStr) ""

putAExpList :: String -> [AExp] -> (String -> String)
putAExpList n aes = let
  ((), (_, _, r)) = runState (mapM_ putAExp aes) (0, 0, id)
  in img n r

-- state: 1. aexp counter; 2. app counter; 3. result string
putAExp :: AExp -> State (Int, Int, String -> String) ()
putAExp ae = do
  (c1, c2, r) <- get
  let comment = (("// AExp" ++ show c1 ++ "\n") ++)
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
    -- atom (Prm "seq") = seqStr
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
