import Data.Maybe
import System.Environment(getArgs)
import Parse
import Exp hiding (compile)
import Compile

main :: IO ()
main = do
  [mn] <- getArgs
  ds <- compile mn
  let ds' = [ (n, compileOpt e) | (n, e) <- ds ]
      defs :: [(Ident, Exp)]
      defs = [ (n, ref i) | ((n, _), i) <- zip ds' [0..] ]
      find n = fromMaybe (error $ "undefined: " ++ show n) $ lookup n defs
      emain = find $ mn ++ ".main"
      res = foldr def emain (zip ds' [0..])
      def :: ((Ident, Exp), Int) -> Exp -> Exp
      def ((_, e), i) r = App2 CT (Lbl i (subst e)) r
      subst (Var n) = find n
      subst (App f a) = App (subst f) (subst a)
      subst e = e
      ref i = Var $ "_" ++ show (i::Int)
--  mapM_ (\ (i, e) -> putStrLn $  i ++ " = " ++ toString e) ds
  mapM_ (\ (i, e) -> putStrLn $  i ++ " = " ++ toString e) ds'
  putStrLn $ toStringP res
  writeFile "out.comb" $ toStringP res