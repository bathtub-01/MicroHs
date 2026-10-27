-- Interactive 100x80 Conway's Game of Life for Ouros / NanoPrelude.
--
-- UART register contract:
--   0x00 RX_DATA: blocking read/consume one byte (Ouros.IO)
--   0x04 TX_DATA: blocking/flow-controlled transmit (Ouros.IO)
--   0x08 STATUS: bit 0 = RX byte available (NEW, nonblocking read)
--
-- Only running mode polls STATUS. Paused mode uses uartGetChar.
-- Terminal must support ANSI CSI cursor movement / inverse video.
module GameOfLife (runGame) where

import Prelude()
import NanoPrelude

-- Four 25-bit chunks represent one 100-cell row. All chunk values fit
-- comfortably in a signed 29-bit Ouros Int.
type Row = (Int, Int, Int, Int)
type Board = [Row]

data KeyMode = KeyNormal | KeyEscape | KeyCSI

data GameState = GameState Board Int Int Bool Int KeyMode

width :: Int
width = 100

height :: Int
height = 80

-- Adjust to taste: running-mode UART status polls per generation.
-- This is a pacing heuristic, not a real-time clock.
framePolls :: Int
framePolls = 3000

emptyRow :: Row
emptyRow = (0::Int, 0::Int, 0::Int, 0::Int)

emptyBoard :: Board
emptyBoard = replicate height emptyRow

-- Pure board operations ------------------------------------------------------

power2 :: Int -> Int
power2 n = go n 1
  where
    go k a = if k == 0 then a else go (k - 1) (a + a)

bitOf :: Int -> Int -> Int
bitOf word i = mod (div word (power2 i)) 2

rowCell :: Row -> Int -> Int
rowCell _ x | x < 0 || x >= width = 0
rowCell (a,b,c,d) x =
  if x < 25 then bitOf a x
  else if x < 50 then bitOf b (x - 25)
  else if x < 75 then bitOf c (x - 50)
  else bitOf d (x - 75)

boardCell :: Board -> Int -> Int -> Int
boardCell _ x y | x < 0 || x >= width || y < 0 || y >= height = 0
boardCell [] _ _ = 0
boardCell (r:_) x 0 = rowCell r x
boardCell (_:rs) x y = boardCell rs x (y - 1)

changeWord :: Int -> Int -> Int -> Int
changeWord word i live =
  let old = bitOf word i
      mask = power2 i
  in if old == live then word
     else if live == 1 then word + mask else word - mask

changeRow :: Row -> Int -> Int -> Row
changeRow row x live =
  case row of
    (a,b,c,d) ->
      if x < 25 then (changeWord a x live, b, c, d)
      else if x < 50 then (a, changeWord b (x-25) live, c, d)
      else if x < 75 then (a, b, changeWord c (x-50) live, d)
      else (a, b, c, changeWord d (x-75) live)

changeCell :: Board -> Int -> Int -> Int -> Board
changeCell [] _ _ _ = []
changeCell (r:rs) x y live =
  if y == 0 then changeRow r x live : rs
  else r : changeCell rs x (y-1) live

-- Bit streams are built lazily and immediately consumed by pack25.
-- Avoids repeated indexing/division when calculating neighbors.
wordBits :: Int -> Int -> [Int] -> [Int]
wordBits 0 _ rest = rest
wordBits n word rest =
  mod word 2 : wordBits (n-1) (div word 2) rest

rowBits :: Row -> [Int]
rowBits (a,b,c,d) =
  wordBits 25 a (wordBits 25 b (wordBits 25 c (wordBits 25 d [])))

firstOrZero :: [Int] -> Int
firstOrZero [] = 0
firstOrZero (v:_) = v

-- Sliding three-row neighborhood. Cells outside the board are dead.
-- The input lists all have exactly 100 entries.
nextBits :: Int -> Int -> Int -> [Int] -> [Int] -> [Int] -> [Int]
nextBits _ _ _ [] _ _ = []
nextBits _ _ _ _ [] _ = []
nextBits _ _ _ _ _ [] = []
nextBits la lb lc (a:as) (b:bs) (c:cs) =
  let ra = firstOrZero as
      rb = firstOrZero bs
      rc = firstOrZero cs
      neighbours = la + a + ra + lb + rb + lc + c + rc
      live = if neighbours == 3 || (b == 1 && neighbours == 2)
               then 1::Int else 0::Int
  in live : nextBits a b c as bs cs

pack25 :: Int -> Int -> Int -> [Int] -> (Int, [Int])
pack25 0 _ total bits = (total, bits)
pack25 _ _ total [] = (total, [])
pack25 n power total (b:bs) =
  pack25 (n-1) (power+power) (total + b*power) bs

packRow :: [Int] -> Row
packRow bits =
  case pack25 25 1 0 bits of
    (a, rest1) -> case pack25 25 1 0 rest1 of
      (b, rest2) -> case pack25 25 1 0 rest2 of
        (c, rest3) -> case pack25 25 1 0 rest3 of
          (d, _) -> (a,b,c,d)

evolveRow :: Row -> Row -> Row -> Row
evolveRow above here below =
  packRow (nextBits 0 0 0 (rowBits above) (rowBits here) (rowBits below))

evolve :: Board -> Board
evolve board = go emptyRow board
  where
    go _ [] = []
    go above (here:rest) =
      let below = case rest of
                    [] -> emptyRow
                    r:_ -> r
      in evolveRow above here below : go here rest

sameRow :: Row -> Row -> Bool
sameRow (a,b,c,d) (w,x,y,z) =
  a == w && b == x && c == y && d == z

-- Tiny preset to make the game interesting immediately.
seedGlider :: Board -> Int -> Int -> Board
seedGlider board x y =
  let px = if x > width - 3 then width - 3 else x
      py = if y > height - 3 then height - 3 else y
      s1 = changeCell board (px+1) py 1
      s2 = changeCell s1 (px+2) (py+1) 1
      s3 = changeCell s2 px (py+2) 1
      s4 = changeCell s3 (px+1) (py+2) 1
  in changeCell s4 (px+2) (py+2) 1

-- ANSI UART presentation ----------------------------------------------------

putString :: String -> IO ()
putString [] = return ()
putString (c:cs) = uartPutChar c >> putString cs

putsAt :: Int -> Int -> String -> IO ()
putsAt row col s = at row col >> putString s

number :: Int -> String
number n = go n []
  where
    go x acc =
      let (q,r) = divMod x 10
          acc' = Char (48+r) : acc
      in if q == 0 then acc' else go q acc'

at :: Int -> Int -> IO ()
at row col =
  putString "\ESC[" >> putString (number row) >>
  putString ";" >> putString (number col) >> putString "H"

cellChar :: Int -> Char
cellChar n = if n == 1 then Char 111 else Char 45

-- Render exactly 25 cells without allocating 25 Chars in memory.
putWord :: Int -> Int -> IO ()
putWord 0 _ = return ()
putWord n word =
  uartPutChar (cellChar (mod word 2)) >>
  putWord (n-1) (div word 2)

putRow :: Row -> IO ()
putRow (a,b,c,d) =
  putWord 25 a >> putWord 25 b >> putWord 25 c >> putWord 25 d

drawRow :: Int -> Row -> IO ()
drawRow y r = at (y+2) 1 >> putRow r

drawBoard :: Int -> Board -> IO ()
drawBoard _ [] = return ()
drawBoard y (r:rs) = drawRow y r >> drawBoard (y+1) rs

-- Avoid transmitting unchanged rows. Initial draw is always complete.
drawChanges :: Int -> Board -> Board -> IO ()
drawChanges _ [] _ = return ()
drawChanges _ _ [] = return ()
drawChanges y (a:as) (b:bs) =
  (if sameRow a b then return () else drawRow y b) >>
  drawChanges (y+1) as bs

-- Inverse-video rendition is our selection marker. Hide physical cursor.
paintSelection :: Board -> Int -> Int -> Bool -> IO ()
paintSelection board x y selected =
  at (y+2) (x+1) >>
  (if selected then putString "\ESC[7m" else return ()) >>
  uartPutChar (cellChar (boardCell board x y)) >>
  (if selected then putString "\ESC[0m" else return ())

drawStatus :: Bool -> Int -> IO ()
drawStatus running generation =
  at 82 1 >> putString "\ESC[2K" >>
  putString (if running then "RUNNING" else "PAUSED ") >>
  putString "  Generation: " >> putString (number generation) >>
  putString "  arrows/hjkl move  o/- edit  SPACE run/pause  g glider  c clear  q quit"

enterScreen :: IO ()
enterScreen =
  putString "\ESC[?1049h\ESC[?25l\ESC[2J\ESC[H" >>
  putsAt 1 1 "Ouros / Conway's Game of Life (100 x 80)"

leaveScreen :: IO ()
leaveScreen = putString "\ESC[0m\ESC[?25h\ESC[?1049l"

-- Device-specific requirement: STATUS bit 0 reports RX_NOT_EMPTY.
-- Never read RX_DATA unless status says a byte is present.
uartTryGetChar :: IO (Maybe Char)
uartTryGetChar =
  readAXIReg 8 >>= \status ->
  if mod status 2 == 0
    then return Nothing
    else readAXIReg 0 >>= \byte -> return (Just (Char byte))

-- Event loop ----------------------------------------------------------------

-- Handle either arrow key sequences ESC [ A/B/C/D or direct h/j/k/l.
move :: GameState -> Int -> Int -> IO GameState
move (GameState board x y running gen _) dx dy =
  let nx = max 0 (min (width-1) (x+dx))
      ny = max 0 (min (height-1) (y+dy))
      st = GameState board nx ny running gen KeyNormal
  in paintSelection board x y False >>
     paintSelection board nx ny True >>
     return st

edit :: GameState -> Int -> IO GameState
edit (GameState board x y _ gen _) value =
  let updated = changeCell board x y value
  in paintSelection updated x y True >>
     drawStatus False gen >>
     return (GameState updated x y False gen KeyNormal)

applyKey :: GameState -> Int -> IO (Maybe GameState)
applyKey st byte =
  case st of
    GameState board x y running gen mode ->
      case mode of
        KeyEscape ->
          if byte == 91 then
            return (Just (GameState board x y running gen KeyCSI))
          else return (Just (GameState board x y running gen KeyNormal))
        KeyCSI ->
          if byte == 65 then move st 0 (0-1) >>= \next -> return (Just next)
          else if byte == 66 then move st 0 1 >>= \next -> return (Just next)
          else if byte == 67 then move st 1 0 >>= \next -> return (Just next)
          else if byte == 68 then move st (0-1) 0 >>= \next -> return (Just next)
          else return (Just (GameState board x y running gen KeyNormal))
        KeyNormal ->
          if byte == 27 then
            return (Just (GameState board x y running gen KeyEscape))
          else if byte == 113 then return Nothing  -- q
          else if byte == 32 then                 -- SPACE
            let running' = not running in
            drawStatus running' gen >>
            return (Just (GameState board x y running' gen KeyNormal))
          else if byte == 111 then               -- o
            edit st 1 >>= \next -> return (Just next)
          else if byte == 45 then                -- -
            edit st 0 >>= \next -> return (Just next)
          else if byte == 104 then move st (0-1) 0 >>= \next -> return (Just next) -- h
          else if byte == 106 then move st 0 1 >>= \next -> return (Just next) -- j
          else if byte == 107 then move st 0 (0-1) >>= \next -> return (Just next) -- k
          else if byte == 108 then move st 1 0 >>= \next -> return (Just next) -- l
          else if byte == 103 then               -- g
            let next = seedGlider board x y in
            drawChanges 0 board next >>
            paintSelection next x y True >> drawStatus False gen >>
            return (Just (GameState next x y False gen KeyNormal))
          else if byte == 99 then                -- c
            let next = emptyBoard in
            drawChanges 0 board next >>
            paintSelection next x y True >> drawStatus False 0 >>
            return (Just (GameState next x y False 0 KeyNormal))
          else return (Just st)

-- Paused: use blocking UART RX to avoid continuous I/O and GC traffic.
-- Running: nonblocking status polling permits autonomous generations.
gameLoop :: GameState -> Int -> Bool -> IO ()
gameLoop state countdown ignoreLF =
  case state of
    GameState board x y running gen mode ->
      (if running then uartTryGetChar else uartGetChar >>= \c -> return (Just c))
      >>= \incoming ->
      case incoming of
        Just (Char byte) ->
          -- Debug: show each received byte before the CRLF filter/key decoder.
          -- Use an otherwise-unused portion of the header row.
          putsAt 1 57 ("RX=" ++ number byte ++ "      ") >>
          if ignoreLF && byte == 10 then
            gameLoop state countdown False
          else applyKey state byte >>= \result ->
            case result of
              Nothing -> leaveScreen
              Just next -> gameLoop next framePolls False
        Nothing ->
          if running && countdown <= 0 then
            let next = evolve board in
            drawChanges 0 board next >>
            paintSelection next x y True >>
            drawStatus True (gen+1) >>
            gameLoop (GameState next x y True (gen+1) mode) framePolls False
          else gameLoop state (countdown-1) ignoreLF

-- Pass True if the game command was terminated by CR, to discard
-- a possible remaining LF without accidentally eating the next key.
runGame :: Bool -> IO ()
runGame ignoreLF =
  let initial = emptyBoard in
  enterScreen >>
  drawBoard 0 initial >>
  paintSelection initial 49 39 True >>
  drawStatus False 0 >>
  -- Debug: proves initial rendering has reached the receive loop.
  putsAt 1 57 "WAIT RX       " >>
  gameLoop (GameState initial 49 39 False 0 KeyNormal) framePolls ignoreLF
