use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 33
// Apps in this file: 119
// Combinators in this file: 228
#[rustfmt::skip]
 pub static countdown: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Countdown.main
        vec![ // 0 
            PTR(5),
            PTR(4),
        ], 
        vec![ // 1 
            COM(4,2,[3, 0, 1]), //XXX
            INT(10),
            COM(2,0,[0]), //X
        ], 
        vec![ // 2 
            COM(4,2,[3, 0, 1]), //XXX
            INT(4),
            PTR(1),
        ], 
        vec![ // 3 
            COM(4,2,[3, 0, 1]), //XXX
            INT(3),
            PTR(2),
        ], 
        vec![ // 4 
            PTR(8),
            PTR(3),
            INT(70),
        ], 
         // FUN1NanoPrelude.length
        vec![ // 5 
            Y,
            PTR(7),
            INT(0),
        ], 
        vec![ // 6 
            COM(6,49,[0, 1, 4, 5, 2, 3]), //XX(X(XXX))
            COM(4,6,[3, 2, 0, 1]), //XX(XX)
            COM(2,0,[0]), //X
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 7 
            COM(3,4,[0, 1, 2, 2]), //XXXX
            PTR(6),
        ], 
         // FUN2Countdown.solutions
        vec![ // 8 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(9),
            PTR(105),
        ], 
        vec![ // 9 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(10),
            PTR(16),
        ], 
         // FUN3Data.List_Type.concatMap
        vec![ // 10 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(12),
        ], 
        vec![ // 11 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(13),
        ], 
        vec![ // 12 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(11),
        ], 
         // FUN4Data.List_Type.++
        vec![ // 13 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            Y,
            PTR(15),
        ], 
        vec![ // 14 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 15 
            COM(4,6,[3, 1, 0, 2]), //XX(XX)
            PTR(14),
        ], 
         // FUN5Countdown.solns
        vec![ // 16 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(17),
            PTR(21),
        ], 
         // FUN6Countdown.preImage
        vec![ // 17 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(20),
        ], 
        vec![ // 18 
            COM(5,9,[4, 0, 1, 2, 3]), //XXXXX
            PRM(EQ,false),
        ], 
        vec![ // 19 
            COM(6,34,[0, 1, 3, 4, 5, 2]), //X(XX(XX))X
            COM(4,7,[0, 1, 3, 2]), //X(XXX)
            PTR(18),
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 20 
            COM(6,47,[5, 0, 1, 2, 3, 4]), //XX(X(XX)X)
            COM(2,0,[0]), //X
            COM(4,56,[2, 0, 1, 3, 1, 3]), //X(XXX(XX))
            PTR(19),
        ], 
         // FUN7Countdown.results
        vec![ // 21 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[0]), //X
            PTR(25),
        ], 
        vec![ // 22 
            COM(5,33,[0, 1, 2, 4, 4, 3]), //X(X(XX)X)X
            COM(4,2,[3, 0, 1]), //XXX
            COM(3,2,[2, 0, 1]), //XXX
            PTR(104),
            COM(2,0,[0]), //X
        ], 
        vec![ // 23 
            PTR(10),
            PTR(28),
        ], 
        vec![ // 24 
            COM(5,50,[0, 4, 1, 2, 3, 4]), //XX(X(X(XX)))
            PTR(26),
            PTR(23),
            PTR(93),
        ], 
        vec![ // 25 
            COM(5,41,[0, 1, 2, 4, 3, 4]), //X(X(XX))(XX)
            COM(3,2,[0, 2, 1]), //XXX
            PTR(24),
            COM(4,2,[3, 0, 1]), //XXX
            PTR(22),
        ], 
         // FUN8NanoPrelude.null
        vec![ // 26 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[1]), //X
            PTR(27),
        ], 
        vec![ // 27 
            COM(3,0,[0]), //X
            COM(2,0,[0]), //X
        ], 
         // FUN9Countdown.combinedResults
        vec![ // 28 
            COM(2,1,[1, 0]), //XX
            PTR(30),
        ], 
        vec![ // 29 
            PTR(31),
            PTR(35),
        ], 
        vec![ // 30 
            COM(4,13,[0, 1, 2, 3, 2]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(29),
            PTR(21),
        ], 
         // FUN10Countdown.concatProdWith
        vec![ // 31 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(34),
        ], 
        vec![ // 32 
            COM(6,61,[0, 1, 2, 3, 5, 4]), //X(X(X(XX)X))
            COM(4,7,[0, 1, 3, 2]), //X(XXX)
            PTR(13),
            PTR(10),
        ], 
        vec![ // 33 
            COM(5,23,[0, 1, 3, 4, 2, 3]), //XXXXXX
            PTR(32),
        ], 
        vec![ // 34 
            COM(6,44,[0, 5, 1, 2, 3, 4]), //X(XX)(XXX)
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(2,0,[0]), //X
            PTR(33),
        ], 
         // FUN11Countdown.combine
        vec![ // 35 
            COM(3,3,[1, 0, 2]), //X(XX)
            PTR(42),
        ], 
        vec![ // 36 
            COM(4,2,[3, 0, 1]), //XXX
            PTR(92),
            COM(2,0,[0]), //X
        ], 
        vec![ // 37 
            COM(4,2,[3, 0, 1]), //XXX
            PTR(91),
            PTR(36),
        ], 
        vec![ // 38 
            COM(4,2,[3, 0, 1]), //XXX
            PTR(90),
            PTR(37),
        ], 
        vec![ // 39 
            COM(4,2,[3, 0, 1]), //XXX
            PTR(89),
            PTR(38),
        ], 
        vec![ // 40 
            COM(5,11,[0, 1, 2, 4, 3]), //XX(XX)X
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(10),
        ], 
        vec![ // 41 
            COM(5,19,[1, 0, 2, 4, 3]), //X(X(XX)X)
            PTR(40),
        ], 
        vec![ // 42 
            COM(5,11,[0, 3, 1, 4, 2]), //XX(XX)X
            PTR(41),
            PTR(43),
            PTR(39),
        ], 
         // FUN12Countdown.combi
        vec![ // 43 
            COM(5,30,[0, 4, 1, 3, 4, 2]), //XX(XXX)X
            PTR(49),
            PTR(45),
            COM(2,0,[0]), //X
        ], 
        vec![ // 44 
            COM(5,18,[0, 1, 4, 2, 3]), //X(XXXX)
            COM(3,2,[2, 0, 1]), //XXX
            PTR(73),
        ], 
        vec![ // 45 
            COM(6,26,[0, 1, 3, 5, 2, 4]), //X(XXX)XX
            COM(5,46,[0, 4, 1, 4, 2, 3]), //XX(XXXX)
            PTR(44),
            PTR(74),
        ], 
        vec![ // 46 
            COM(5,9,[0, 4, 2, 3, 1]), //XXXXX
            PTR(50),
            COM(2,0,[0]), //X
        ], 
        vec![ // 47 
            COM(5,38,[0, 1, 4, 2, 3, 4]), //X(XX)X(XX)
            COM(5,47,[0, 4, 1, 2, 4, 3]), //XX(X(XX)X)
        ], 
        vec![ // 48 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(47),
            PTR(46),
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 49 
            COM(6,25,[0, 1, 2, 4, 5, 3]), //XX(XX)XX
            PTR(48),
        ], 
         // FUN13Countdown.valid
        vec![ // 50 
            COM(4,6,[0, 2, 1, 3]), //XX(XX)
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(59),
        ], 
        vec![ // 51 
            COM(6,26,[0, 5, 1, 2, 3, 4]), //X(XXX)XX
            COM(4,15,[0, 3, 1, 3, 2]), //X(XX)(XX)
            PRM(LT,false),
            INT(1),
        ], 
        vec![ // 52 
            COM(6,25,[0, 1, 2, 5, 3, 4]), //XX(XX)XX
            COM(6,32,[0, 1, 5, 2, 3, 4]), //X(XXXX)X
            PTR(51),
            PTR(61),
            PRM(EQ,false),
            INT(0),
        ], 
        vec![ // 53 
            COM(4,4,[3, 0, 1, 2]), //XXXX
            PRM(EQ,false),
            INT(3),
            COM(2,0,[0]), //X
        ], 
        vec![ // 54 
            COM(5,40,[0, 4, 1, 2, 3, 4]), //X(XXX)(XX)
            COM(5,41,[0, 1, 4, 2, 4, 3]), //X(X(XX))(XX)
            PRM(LT,false),
            INT(3),
            PTR(53),
        ], 
        vec![ // 55 
            COM(6,31,[0, 1, 2, 3, 5, 4]), //XX(X(XX))X
            COM(4,4,[0, 3, 1, 2]), //XXXX
            PTR(54),
            PTR(60),
        ], 
        vec![ // 56 
            COM(5,40,[0, 4, 1, 2, 3, 4]), //X(XXX)(XX)
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            PRM(LT,false),
            INT(2),
        ], 
        vec![ // 57 
            COM(5,41,[0, 1, 2, 4, 3, 4]), //X(X(XX))(XX)
            COM(3,6,[0, 2, 1, 2]), //XX(XX)
            PTR(56),
        ], 
        vec![ // 58 
            COM(5,19,[0, 1, 4, 2, 3]), //X(X(XX)X)
            PTR(57),
            PTR(55),
            PRM(LE,false),
            COM(2,0,[1]), //X
        ], 
        vec![ // 59 
            COM(4,16,[0, 3, 1, 3, 2]), //XX(XXX)
            PTR(58),
            PTR(52),
            COM(2,0,[1]), //X
        ], 
         // FUN14Data.Bool.not
        vec![ // 60 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[1]), //X
            COM(2,0,[0]), //X
        ], 
         // FUN15NanoPrelude.mod
        vec![ // 61 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(3,2,[0, 2, 1]), //XXX
            PTR(62),
            COM(2,0,[1]), //X
        ], 
         // FUN16NanoPrelude.divMod
        vec![ // 62 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(72),
        ], 
        vec![ // 63 
            COM(5,12,[0, 4, 1, 2, 3]), //X(XXX)X
            COM(3,2,[2, 0, 1]), //XXX
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 64 
            COM(5,53,[0, 1, 1, 4, 2, 3]), //X(XX(XX)X)
            COM(5,49,[0, 4, 1, 4, 2, 3]), //XX(X(XXX))
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            PRM(LE,false),
            COM(3,2,[2, 1, 0]), //XXX
        ], 
        vec![ // 65 
            COM(4,9,[0, 3, 1, 2, 3]), //XXXXX
            PTR(64),
            PTR(63),
            PRM(SUB,fasle),
        ], 
        vec![ // 66 
            COM(5,11,[0, 1, 2, 4, 3]), //XX(XX)X
            COM(4,16,[0, 1, 3, 2, 3]), //XX(XXX)
            COM(3,2,[0, 2, 1]), //XXX
            PTR(65),
            PRM(ADD,false),
        ], 
        vec![ // 67 
            COM(4,6,[0, 2, 1, 3]), //XX(XX)
            COM(3,2,[0, 2, 1]), //XXX
            PTR(66),
        ], 
        vec![ // 68 
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,2,[2, 0, 1]), //XXX
            INT(1),
        ], 
        vec![ // 69 
            COM(5,16,[0, 1, 2, 3, 4]), //XX(XXX)
            COM(5,37,[4, 0, 3, 1, 2, 4]), //XXXX(XX)
            PRM(LE,false),
            COM(3,2,[2, 0, 1]), //XXX
            INT(0),
        ], 
        vec![ // 70 
            COM(4,31,[0, 3, 1, 3, 2, 3]), //XX(X(XX))X
            PTR(69),
            PTR(68),
            PRM(SUB,fasle),
        ], 
        vec![ // 71 
            COM(5,40,[0, 1, 2, 4, 3, 4]), //X(XXX)(XX)
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            COM(5,37,[4, 0, 1, 2, 3, 4]), //XXXX(XX)
            PRM(LE,false),
            PTR(70),
        ], 
        vec![ // 72 
            COM(6,25,[0, 1, 2, 5, 3, 4]), //XX(XX)XX
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,16,[0, 2, 2, 1, 2]), //XX(XXX)
            PTR(71),
            PTR(67),
            PRM(ADD,false),
        ], 
         // FUN17Countdown.App
        vec![ // 73 
            COM(6,46,[5, 0, 1, 2, 3, 4]), //XX(XXXX)
            INT(5),
            COM(4,4,[3, 0, 1, 2]), //XXXX
        ], 
         // FUN18Countdown.apply
        vec![ // 74 
            COM(5,48,[0, 3, 1, 4, 2, 4]), //XX(XX(XX))
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(82),
            PTR(76),
        ], 
        vec![ // 75 
            COM(6,26,[0, 5, 1, 2, 3, 4]), //X(XXX)XX
            COM(4,15,[0, 3, 1, 3, 2]), //X(XX)(XX)
            PRM(LT,false),
            INT(1),
        ], 
        vec![ // 76 
            COM(5,39,[0, 1, 2, 4, 4, 3]), //XX(XX)(XX)
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            PTR(75),
            PTR(88),
            PRM(ADD,false),
        ], 
        vec![ // 77 
            COM(4,4,[3, 0, 1, 2]), //XXXX
            PRM(EQ,false),
            INT(3),
            INT(0),
        ], 
        vec![ // 78 
            COM(5,40,[0, 4, 1, 2, 3, 4]), //X(XXX)(XX)
            COM(5,41,[0, 1, 4, 2, 4, 3]), //X(X(XX))(XX)
            PRM(LT,false),
            INT(3),
            PTR(77),
        ], 
        vec![ // 79 
            COM(5,39,[0, 1, 2, 4, 3, 4]), //XX(XX)(XX)
            COM(4,4,[0, 3, 1, 2]), //XXXX
            PTR(78),
        ], 
        vec![ // 80 
            COM(5,40,[0, 4, 1, 2, 3, 4]), //X(XXX)(XX)
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            PRM(LT,false),
            INT(2),
        ], 
        vec![ // 81 
            COM(5,41,[0, 1, 2, 4, 3, 4]), //X(X(XX))(XX)
            COM(3,6,[0, 2, 1, 2]), //XX(XX)
            PTR(80),
        ], 
        vec![ // 82 
            COM(5,57,[0, 1, 4, 2, 3, 4]), //X(X(XX)(XX))
            PTR(81),
            PTR(79),
            PRM(SUB,fasle),
            PTR(83),
        ], 
         // FUN19Countdown.mul
        vec![ // 83 
            COM(4,35,[0, 1, 2, 3, 3, 3]), //X(X(XXX))X
            PTR(87),
            PTR(86),
            PTR(85),
        ], 
        vec![ // 84 
            COM(5,49,[0, 1, 2, 4, 3, 4]), //XX(X(XXX))
            COM(6,26,[0, 1, 5, 2, 3, 4]), //X(XXX)XX
            COM(5,51,[0, 4, 1, 2, 3, 2]), //X(XXXXX)
            PTR(83),
            PRM(ADD,false),
        ], 
        vec![ // 85 
            COM(6,51,[0, 1, 5, 2, 3, 4]), //X(XXXXX)
            COM(3,2,[0, 2, 1]), //XXX
            PTR(84),
            PRM(ADD,false),
            PRM(EQ,false),
            INT(0),
        ], 
        vec![ // 86 
            COM(4,4,[0, 3, 1, 2]), //XXXX
            PTR(62),
            INT(2),
        ], 
        vec![ // 87 
            COM(5,28,[4, 0, 1, 2, 4, 3]), //XXX(XX)X
            PRM(EQ,false),
            INT(1),
        ], 
         // FUN20NanoPrelude.div
        vec![ // 88 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(3,2,[0, 2, 1]), //XXX
            PTR(62),
            COM(2,0,[0]), //X
        ], 
         // FUN21Countdown.Add
        vec![ // 89 
            COM(3,2,[2, 0, 1]), //XXX
            INT(0),
            COM(1,0,[0]), //X
        ], 
         // FUN22Countdown.Sub
        vec![ // 90 
            COM(3,2,[2, 0, 1]), //XXX
            INT(3),
            COM(1,0,[0]), //X
        ], 
         // FUN23Countdown.Mul
        vec![ // 91 
            COM(3,2,[2, 0, 1]), //XXX
            INT(2),
            COM(1,0,[0]), //X
        ], 
         // FUN24Countdown.Div
        vec![ // 92 
            COM(3,2,[2, 0, 1]), //XXX
            INT(1),
            COM(1,0,[0]), //X
        ], 
         // FUN25Countdown.split
        vec![ // 93 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[0]), //X
            PTR(97),
        ], 
        vec![ // 94 
            COM(6,61,[0, 1, 2, 3, 5, 4]), //X(X(X(XX)X))
            PTR(98),
            PTR(101),
            COM(3,2,[2, 0, 1]), //XXX
            COM(4,2,[3, 0, 1]), //XXX
            COM(1,0,[0]), //X
        ], 
        vec![ // 95 
            COM(6,49,[0, 1, 2, 3, 4, 5]), //XX(X(XXX))
            COM(5,45,[0, 1, 4, 2, 3, 4]), //X(XX)(X(XX))
            COM(4,2,[3, 0, 1]), //XXX
            COM(3,2,[2, 0, 1]), //XXX
            COM(4,2,[3, 1, 0]), //XXX
            COM(2,0,[0]), //X
        ], 
        vec![ // 96 
            COM(4,11,[0, 3, 1, 3, 2]), //XX(XX)X
            PTR(95),
            PTR(94),
            PTR(93),
        ], 
        vec![ // 97 
            COM(5,11,[0, 1, 2, 4, 3]), //XX(XX)X
            COM(4,11,[0, 3, 1, 3, 2]), //XX(XX)X
            PTR(26),
            PTR(96),
            COM(2,0,[0]), //X
        ], 
         // FUN26NanoPrelude.map
        vec![ // 98 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(100),
        ], 
        vec![ // 99 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 100 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(99),
        ], 
         // FUN27Countdown.cross
        vec![ // 101 
            COM(3,3,[1, 0, 2]), //X(XX)
            PTR(103),
        ], 
        vec![ // 102 
            COM(5,11,[0, 1, 2, 4, 3]), //XX(XX)X
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,2,[2, 0, 1]), //XXX
        ], 
        vec![ // 103 
            COM(4,7,[1, 0, 2, 3]), //X(XXX)
            PTR(102),
        ], 
         // FUN28Countdown.Val
        vec![ // 104 
            COM(4,6,[3, 0, 1, 2]), //XX(XX)
            INT(4),
            COM(2,1,[1, 0]), //XX
        ], 
         // FUN29Countdown.choices
        vec![ // 105 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(106),
            PTR(115),
        ], 
        vec![ // 106 
            PTR(10),
            PTR(107),
        ], 
         // FUN30Countdown.perms
        vec![ // 107 
            COM(3,2,[2, 0, 1]), //XXX
            PTR(109),
            PTR(108),
        ], 
        vec![ // 108 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(10),
            PTR(110),
            PTR(107),
        ], 
        vec![ // 109 
            COM(4,2,[3, 0, 1]), //XXX
            COM(2,0,[0]), //X
            COM(2,0,[0]), //X
        ], 
         // FUN31Countdown.interleave
        vec![ // 110 
            COM(4,7,[0, 1, 3, 2]), //X(XXX)
            Y,
            PTR(114),
            PTR(111),
        ], 
        vec![ // 111 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(98),
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 112 
            COM(6,34,[0, 1, 2, 3, 5, 4]), //X(XX(XX))X
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            COM(5,41,[0, 1, 2, 4, 3, 4]), //X(X(XX))(XX)
            COM(4,2,[3, 0, 1]), //XXX
            COM(4,2,[3, 0, 1]), //XXX
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 113 
            COM(5,12,[0, 1, 2, 4, 3]), //X(XXX)X
            COM(4,2,[3, 0, 1]), //XXX
            COM(4,2,[3, 1, 0]), //XXX
            COM(2,0,[0]), //X
            COM(2,0,[0]), //X
        ], 
        vec![ // 114 
            COM(5,39,[0, 1, 2, 4, 3, 4]), //XX(XX)(XX)
            COM(5,17,[0, 1, 2, 3, 4]), //XX(X(XX))
            COM(3,2,[2, 0, 1]), //XXX
            PTR(113),
            PTR(112),
        ], 
         // FUN32Countdown.subs
        vec![ // 115 
            COM(3,2,[2, 0, 1]), //XXX
            PTR(118),
            PTR(117),
        ], 
        vec![ // 116 
            COM(6,59,[0, 1, 2, 3, 4, 5]), //X(XX(X(XX)))
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(3,6,[0, 2, 1, 2]), //XX(XX)
            PTR(13),
            PTR(98),
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 117 
            COM(3,2,[0, 2, 1]), //XXX
            PTR(116),
            PTR(115),
        ], 
        vec![ // 118 
            COM(4,2,[3, 0, 1]), //XXX
            COM(2,0,[0]), //X
            COM(2,0,[0]), //X
        ], 
    ]
});