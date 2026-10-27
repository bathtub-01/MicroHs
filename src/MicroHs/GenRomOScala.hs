module MicroHs.GenRomOScala(genRomOScala, genRomScala) where
import Prelude(); import MHSPrelude
import Data.List
import MicroHs.CodeGen
import MicroHs.GenRom

-- Generate Ouros program images for Chisel/Scala.
--
-- A JVM method has a 65535-byte code limit.  A single gigantic Seq(...) of
-- appBuilder calls eventually exceeds that limit, so construct each image
-- through several small, separately compiled methods instead.
--
-- Chunking happens only *after* serialise has resolved the heap/combinator
-- addresses.  Neither App order nor any pointer or combinator index changes.

header :: String
header = "\
 \package benchmarks\n\
 \import common.Helper._\n\
 \import common.Atom\n\
 \import chisel3.Vec\n\
 \ \n"

-- 32 Apps per method keeps even eight-Atom Apps comfortably below the
-- JVM method size limit.  Both heap and combinator ROMs use this size.
appsPerChunk :: Int
appsPerChunk = 32

-- spine application
app :: Int -> (String -> String) -> (String -> String)
app offset r = ("appBuilder( // " ++) . (show offset ++) . ("\n" ++) . r . ("),\n" ++)

-- atoms: the encoding and builder names are unchanged.
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

seqStr :: String -> String
seqStr = ("seqBuilder(),\n" ++)

io :: Bool -> String -> String
io True = ("ioBuilder(true),\n" ++)
io _    = ("ioBuilder(false),\n" ++)

err :: Int -> (String -> String)
err code = ("errorBuilder(" ++) . (show code ++) . ("),\n" ++)

putAtom :: Atom -> (String -> String)
putAtom (Prm "Y") = y
putAtom (Prm "seq") = seqStr
putAtom (Prm "IO.Write") = io True
putAtom (Prm "IO.Read") = io False
putAtom (Prm op)
  | "error" `isPrefixOf` op = err $ read (drop 5 op)
  | otherwise = prim op
putAtom (Int i) = int i
putAtom (Com a p) = comb a p
putAtom (Fun p) = ptr p False
putAtom (Ptr p hc) = ptr p hc
putAtom (Apt p unq) = arg p unq

-- The optional AExp number is used only for comments.  Every input App
-- appears exactly once in the flattened sequence, even across chunks.
flattenAExps :: [AExp] -> [(Maybe Int, App)]
flattenAExps aes = concatMap flattenOne (zip [0..] aes)
  where
    flattenOne :: (Int, AExp) -> [(Maybe Int, App)]
    flattenOne (_, []) = []
    flattenOne (i, a : as) = (Just i, a) : map (\ap -> (Nothing, ap)) as

chunksOf :: Int -> [a] -> [[a]]
chunksOf _ [] = []
chunksOf n xs = let (h, t) = splitAt n xs in h : chunksOf n t

-- Render an App with its global (ROM) offset, not its chunk-local offset.
renderApp :: (Int, (Maybe Int, App)) -> String
renderApp (index, (label, atoms)) =
  (case label of
     Just n -> "// AExp" ++ show n ++ "\n"
     Nothing -> "") ++
  app index (foldr (.) id (map putAtom atoms)) ""

-- Using fixed-width names is cosmetic; indices above 999 still work.
chunkName :: String -> Int -> String
chunkName prefix i = prefix ++ "Chunk" ++ pad3 (show i)
  where
    pad3 str
      | length str >= 3 = str
      | otherwise = pad3 ('0' : str)

-- Produce the typed top-level image and its small helper methods.
-- Empty images are legal and require no helpers.
putAExpList :: String -> String -> [AExp] -> (String, String)
putAExpList imageName prefix aes =
  let
    numberedApps = zip [0..] (flattenAExps aes)
    groups = zip [0..] (chunksOf appsPerChunk numberedApps)
    names = map (chunkName prefix . fst) groups
    initializer =
      if null groups
        then "val " ++ imageName ++ ": Seq[Vec[Atom]] = Seq.empty[Vec[Atom]]\n"
        else "val " ++ imageName ++ ": Seq[Vec[Atom]] = Seq.concat(\n" ++
             intercalate ",\n" (map ("  " ++) names) ++ "\n)\n"
    emitChunk (i, aps) =
      "  private def " ++ chunkName prefix i ++ ": Seq[Vec[Atom]] = Seq(\n" ++
      concatMap renderApp aps ++
      "  )\n"
    definitions = concatMap emitChunk groups
  in (initializer, definitions)

genRomOScala :: String -> ([AExp], [AExp]) -> String
genRomOScala progName (heap, cmb) =
  let
    -- Resolve AExp-relative pointers exactly once, before flattening/chunking.
    (heap', cmb') = serialise heap cmb
    (heapInit, heapDefs) = putAExpList "heap_img" "heap" heap'
    (combInit, combDefs) = putAExpList "comb_img" "comb" cmb'
  in header ++
     "object " ++ progName ++ " extends Benchmark {\n" ++
     "override def toString() = \"" ++ progName ++ "\"\n" ++
     "val combinatorCount = " ++ show (length cmb) ++ "\n" ++
     heapInit ++ combInit ++
     "\n  // Each helper constructs at most " ++ show appsPerChunk ++ " Apps.\n" ++
     heapDefs ++ combDefs ++
     "}\n"

-- Alternative entry point for builds using the GenRomScala module name.
genRomScala :: String -> ([AExp], [AExp]) -> String
genRomScala = genRomOScala
