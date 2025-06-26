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
import MicroHs.Abstract
import MicroHs.GenRom(getPatNum,inlineSingle,finalEtaApply,freeText)

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
  (("#[rustfmt::skip]\npub static " ++ name ++": LazyLock<Program> = LazyLock::new(|| {\n") ++) . r .
  ("\n});" ++)

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
comb :: Int -> Pat -> [Int] -> (String -> String)
comb art p is =
  atomIndent ("COM(" ++ show art ++ "," ++ show (getPatNum p) ++ ","
              ++ listPrint isExt ++ "), //" ++ show p ++ "\n")
  where
    holes = 6 -- FIXME: better be configured
    isExt = is ++ replicate (holes - length is) 0

primConvert :: String -> (String, String)
primConvert "==" = ("EQ", "false")
primConvert "/=" = ("EQ", "true")
primConvert "<=" = ("LE", "false")
primConvert ">" = ("LE", "true")
primConvert "<" = ("LT", "false")
primConvert ">=" = ("LT", "true")
primConvert "+" = ("ADD", "false")
primConvert "-" = ("SUB", "fasle")
primConvert "*" = ("MUL", "false")

ptr :: Int -> (String -> String)
ptr n =
  atomIndent ("PTR(" ++ show n ++ "),\n")

int :: Int -> (String -> String)
int n =
  atomIndent ("INT(" ++ show n ++ "),\n")

prim :: String -> (String -> String)
prim op =
  let (code, rev) = primConvert op
  in atomIndent ("PRM(" ++ code ++ "," ++ rev ++ "),\n")

y :: String -> String
y = atomIndent "Y,\n"

err :: Int -> (String -> String)
err n =
  atomIndent ("ERR(" ++ show n ++ "),\n")

listPrint :: [Int] -> String
listPrint [] = "[]"
listPrint xs = "[" ++ inner ++ "]"
  where
    inner = concat $ zipWith (\x y -> show x ++ y) xs (replicate (length xs - 1) "," ++ [""])

genRomRs :: String -> (Ident, [LDef]) -> String
genRomRs progName (mainName, ldefs) =
  let
    -- ds = killDead (mainName, inlineSingle ldefs)
    ds = finalEtaApply $ inlineSingle ldefs
    dMap = M.fromList ds
    -- state: 1. fun counter; 2. app counter; 3. comb counter; 4. function map; 5. resulting string
    dfs :: Ident -> State (Int, Int, Int, M.Map Exp, String -> String) ()
    dfs n = do
      (i, ptr, combs, seen, r) <- get
      case M.lookup n seen of
        Just _ -> return ()
        Nothing -> do
          let e = findIdentIn n dMap
          put (i, ptr + 1, combs, M.insert n (ref ptr) seen, r)
          -- print this function
          buildFunc (substv e) (showIdent n)
          (i', ptr', combs', seen', r') <- get
          put (i + 1, ptr', combs', seen', r')
          -- Walk n's children
          mapM_ dfs $ freeVars e

    buildFunc :: Exp -> String -> State (Int, Int, Int, M.Map Exp, String -> String) ()
    buildFunc e name =
      let
        -- state: 1. ptr counter; 2. comb counter; 3. current spine; 4. apps
        build :: Exp -> State (Int, Int, String -> String, [String -> String]) ()
        build e = do
          (i, combs, s, as) <- get
          case e of
            App f (App a1 a2) -> do
              put (i, combs, freeText "", as)
              build (App a1 a2)
              (i', combs', s', as') <- get
              put (i' + 1, combs', ptr i' . s, as' ++ [app i' s'])
              build f
            App f a -> do
              let combs' =
                    case a of
                      Sc _ _ _ -> combs + 1
                      _ -> combs
              put(i, combs', atom a . s, as)
              build f
            _ -> do
              let combs' =
                    case e of
                      Sc _ _ _ -> combs + 1
                      _ -> combs
              put(i, combs', atom e . s, as)
      in do
      (i, ptr, combs, seen, r) <- get
      let (_, (ptr', combs', spn, aps)) = runState (build e) (ptr, combs, freeText "", [])
      put (i, ptr', combs', seen, r . indentation 2 (" // FUN" ++ show i ++ name  ++ "\n") . app (ptr - 1) spn . foldr (.) (freeText "") aps)
           
          
    (_, (funCount, appCount, combCount, defs, res)) = runState (dfs mainName) (0, 0, 0, M.empty, freeText "")
    ref i = Var $ mkIdent $ "PTR" ++ show i
    findIdentIn n m = fromMaybe (errorMessage (getSLoc n) $ "No definition found for: " ++ showIdent n) $
                      M.lookup n m
    findIdent n = findIdentIn n defs
    substv aexp =
      case aexp of
        Var n -> findIdent n
        App f a -> App (substv f) (substv a)
        e -> e
  in header ++
     "// Functions in this file: " ++ show funCount ++ "\n"
     ++ "// Apps in this file: " ++ show appCount ++ "\n"
     ++ "// Combinators in this file: " ++ show combCount ++ "\n"
     ++ lazyLockProg progName (vecS res) ""

atom :: Exp -> (String -> String)
atom ae =
  case ae of
    Var i -> if "PTR" `isPrefixOf` showIdent i then ptr $ read (drop 3 (showIdent i))
               else error "Strange Var exists."
    Lit (LInt i) -> int i
    Lit (LPrim "Y") -> y
    Lit (LPrim ('e':'r':'r':'o':'r':rest)) -> err $ read rest
    Lit (LPrim op) -> if "error" `isPrefixOf` op then err $ read (drop 5 op)
                        else prim op
    Lit _ -> error "Strange Lit exists."
    Sc a p is -> comb a p is
    _ -> error "Not an Atom."
