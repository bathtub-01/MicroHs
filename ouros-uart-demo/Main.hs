-- Bare-metal UART command loop for real-world Ouros.
module Main where

import Prelude()
import NanoPrelude
import Parser

-- Command syntax ------------------------------------------------------------

command :: String -> Parser a -> Parser a
command name arguments = literal name >> arguments

commands :: Parser String
commands =
      command "hello" (return "hi!")
  <|> command "hi"    (return "hello!")
  <|> command "fib"   (
        spaces1 >> integer >>= \n ->
        if n >= 0 && n <= 40
          then return (intToString (fib n))
          else failure)
  <|> command "caesar" (
        spaces1 >> integer >>= \n ->
        spaces1 >> word >>= \s ->
        return (caesar n s))

handleCommand :: String -> String
handleCommand line =
  case parseAll commands line of
    Nothing     -> "don't know!"
    Just answer -> answer

-- Pure command implementations ----------------------------------------------

-- Use an iterative Fibonacci to keep both runtime and heap use modest.
-- F(40) fits in Ouros's signed 29-bit integer representation.
fib :: Int -> Int
fib n = go n 0 1
  where
    go remaining a b =
      if remaining == 0
        then a
        else go (remaining - 1) b (a + b)

-- NanoPrelude has no 'show', so print a nonnegative integer in decimal.
intToString :: Int -> String
intToString n = digits n []
  where
    digits x acc =
      let (q, r) = divMod x 10
          acc' = Char (48 + r) : acc
      in if q == 0 then acc' else digits q acc'

-- ASCII Caesar shift; preserve case and nonalphabetic characters.
-- NanoPrelude's 'mod' is defined for nonnegative dividends.
shiftAmount :: Int -> Int
shiftAmount n =
  if n >= 0
    then mod n 26
    else 25 - mod (0 - (n + 1)) 26

caesar :: Int -> String -> String
caesar n = map (shiftChar (shiftAmount n))

shiftChar :: Int -> Char -> Char
shiftChar k (Char c)
  | c >= 65 && c <= 90 = Char (65 + mod (c - 65 + k) 26)
  | c >= 97 && c <= 122 = Char (97 + mod (c - 97 + k) 26)
  | otherwise = Char c

-- UART console --------------------------------------------------------------

uartPutString :: String -> IO ()
uartPutString [] = return ()
uartPutString (c:cs) = uartPutChar c >> uartPutString cs

newline :: IO ()
newline = uartPutChar (Char 13) >> uartPutChar (Char 10)

putLine :: String -> IO ()
putLine str = uartPutString str >> newline

reverseChars :: String -> String
reverseChars xs = go xs []
  where
    go [] acc = acc
    go (c:cs) acc = go cs (c:acc)

-- Read one command with UART echo and backspace handling.
-- The boolean suppresses the LF of a CRLF sequence across loop iterations.
readLine :: Bool -> String -> IO (String, Bool)
readLine ignoreLF reversed =
  uartGetChar >>= \(Char byte) ->
  if ignoreLF && byte == 10 then
    readLine False reversed
  else if byte == 13 then
    newline >> return (reverseChars reversed, True)
  else if byte == 10 then
    newline >> return (reverseChars reversed, False)
  else if byte == 8 || byte == 127 then
    case reversed of
      [] -> readLine False []
      (_:rest) -> uartPutString "\b \b" >> readLine False rest
  else if byte >= 32 && byte <= 126 then
    uartPutChar (Char byte) >> readLine False (Char byte : reversed)
  else
    readLine False reversed

-- Event loop: wait for UART input, dispatch once ENTER arrives, repeat.
-- RX is blocking in the existing Ouros.IO driver; no interrupts are used.
loop :: Bool -> IO ()
loop ignoreLF =
  uartPutString "ouros> " >>
  readLine ignoreLF [] >>= \(line, endedWithCR) ->
  putLine (handleCommand line) >>
  loop endedWithCR

main :: IO ()
main = putLine "Ouros UART demo" >> loop False
