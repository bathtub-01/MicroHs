-- Small backtracking parser combinators for NanoPrelude.
-- A parser either fails or returns a value and the unconsumed input.
module Parser
  ( Parser, parseAll, failure, (<|>)
  , satisfy, char, literal, spaces, spaces1, integer, word
  ) where

import Prelude()
import NanoPrelude

newtype Parser a = Parser (String -> Maybe (a, String))

runParser :: Parser a -> String -> Maybe (a, String)
runParser (Parser p) = p

instance Monad Parser where
  return x = Parser (\input -> Just (x, input))

  (Parser p) >>= k = Parser (\input ->
    case p input of
      Nothing -> Nothing
      Just (x, rest) -> runParser (k x) rest)

failure :: Parser a
failure = Parser (\_ -> Nothing)

-- Alternatives retry from the *original* input on failure.
infixr 3 <|>
(<|>) :: Parser a -> Parser a -> Parser a
(Parser p) <|> (Parser q) = Parser (\input ->
  case p input of
    Nothing -> q input
    result  -> result)

satisfy :: (Char -> Bool) -> Parser Char
satisfy predicate = Parser (\input ->
  case input of
    [] -> Nothing
    c:cs -> if predicate c then Just (c, cs) else Nothing)

char :: Char -> Parser Char
char (Char expected) = satisfy (\(Char actual) -> actual == expected)

literal :: String -> Parser String
literal [] = return []
literal (c:cs) =
  char c >> literal cs >> return (c:cs)

-- 'many' is intended for parsers that always consume on success.
many :: Parser a -> Parser [a]
many p = some p <|> return []

some :: Parser a -> Parser [a]
some p =
  p >>= \x ->
  many p >>= \xs ->
  return (x:xs)

isSpace :: Char -> Bool
isSpace (Char n) = n == 32 || n == 9

spaces :: Parser ()
spaces = many (satisfy isSpace) >> return ()

spaces1 :: Parser ()
spaces1 = some (satisfy isSpace) >> return ()

word :: Parser String
word = some (satisfy (\(Char n) -> n > 32))

digit :: Parser Int
digit =
  satisfy (\(Char n) -> n >= 48 && n <= 57) >>= \(Char n) ->
  return (n - 48)

-- Ouros INT has a signed 29-bit payload. Reject decimal overflow.
-- Negative accumulation also accepts the asymmetric minimum (-268435456).
checkedDecimal :: Bool -> [Int] -> Maybe Int
checkedDecimal negative = go 0
  where
    go acc [] = Just acc
    go acc (d:ds) =
      if negative then
        if acc < (0 - 26843545) ||
           (acc == (0 - 26843545) && d > 6)
          then Nothing
          else go (acc * 10 - d) ds
      else
        if acc > 26843545 || (acc == 26843545 && d > 5)
          then Nothing
          else go (acc * 10 + d) ds

signedDigits :: Bool -> Parser Int
signedDigits negative =
  some digit >>= \ds ->
  case checkedDecimal negative ds of
    Nothing -> failure
    Just n -> return n

integer :: Parser Int
integer =
      (char (Char 45) >> signedDigits True)
  <|> (char (Char 43) >> signedDigits False)
  <|> signedDigits False

eof :: Parser ()
eof = Parser (\input ->
  case input of
    [] -> Just ((), [])
    _  -> Nothing)

-- Complete command line only: leading/trailing whitespace is okay.
parseAll :: Parser a -> String -> Maybe a
parseAll p input =
  case runParser (spaces >> p >>= \value -> spaces >> eof >> return value) input of
    Nothing -> Nothing
    Just (value, _) -> Just value
