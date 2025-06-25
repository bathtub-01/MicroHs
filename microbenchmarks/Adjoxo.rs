use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 23
// Apps in this file: 106
// Combinators in this file: 175
#[rustfmt::skip]
 pub static Adjoxo: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Adjoxo.main
        vec![ // 0 
            PTR(5),
            PTR(4),
            PTR(2),
        ], 
        vec![ // 1 
            COM(4,2,[3, 0, 1]), //XXX
            INT(5),
            COM(2,0,[0]), //X
        ], 
        vec![ // 2 
            COM(4,2,[3, 0, 1]), //XXX
            INT(2),
            PTR(1),
        ], 
        vec![ // 3 
            COM(4,2,[3, 0, 1]), //XXX
            INT(4),
            COM(2,0,[0]), //X
        ], 
        vec![ // 4 
            COM(4,2,[3, 0, 1]), //XXX
            INT(1),
            PTR(3),
        ], 
         // FUN1Adjoxo.adjudicate
        vec![ // 5 
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            COM(3,6,[0, 2, 1, 2]), //XX(XX)
            PTR(13),
            PTR(6),
        ], 
        vec![ // 6 
            COM(5,12,[0, 1, 4, 3, 2]), //X(XXX)X
            PTR(70),
            PTR(73),
            COM(2,0,[1]), //X
        ], 
        vec![ // 7 
            PTR(70),
            COM(3,0,[2]), //X
            COM(2,0,[1]), //X
        ], 
        vec![ // 8 
            PTR(70),
            COM(3,0,[2]), //X
            COM(2,0,[0]), //X
        ], 
        vec![ // 9 
            COM(6,24,[0, 1, 5, 2, 3, 4]), //X(XX)XXX
            COM(6,54,[0, 1, 2, 5, 4, 3]), //X(X(XXX)X)
            PTR(19),
            PTR(70),
            PTR(73),
            COM(2,0,[1]), //X
        ], 
        vec![ // 10 
            COM(5,30,[0, 1, 2, 4, 4, 3]), //XX(XXX)X
            COM(5,30,[0, 4, 1, 4, 2, 3]), //XX(XXX)X
            PTR(19),
            PTR(9),
            PTR(8),
        ], 
        vec![ // 11 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            PTR(14),
            PTR(16),
            PTR(16),
        ], 
        vec![ // 12 
            COM(5,58,[0, 1, 4, 2, 4, 3]), //X(XX(XXX))
            COM(5,47,[0, 4, 1, 2, 4, 3]), //XX(X(XX)X)
            PTR(11),
            PTR(10),
            PTR(7),
        ], 
        vec![ // 13 
            COM(5,28,[0, 4, 1, 2, 4, 3]), //XXX(XX)X
            PTR(12),
            PTR(70),
            PTR(73),
            COM(2,0,[0]), //X
        ], 
         // FUN2Adjoxo.cmp
        vec![ // 14 
            COM(5,12,[0, 1, 4, 2, 3]), //X(XXX)X
            COM(3,2,[0, 2, 1]), //XXX
            PTR(15),
            COM(3,0,[1]), //X
            COM(3,0,[0]), //X
        ], 
        vec![ // 15 
            COM(5,29,[0, 4, 1, 4, 2, 3]), //X(XX)(XX)X
            COM(5,46,[0, 4, 1, 4, 2, 3]), //XX(XXXX)
            PRM(EQ,false),
            PRM(LE,false),
            COM(3,0,[2]), //X
        ], 
         // FUN3NanoPrelude.length
        vec![ // 16 
            Y,
            PTR(18),
            INT(0),
        ], 
        vec![ // 17 
            COM(6,49,[0, 1, 4, 5, 2, 3]), //XX(X(XXX))
            COM(4,6,[3, 2, 0, 1]), //XX(XX)
            COM(2,0,[0]), //X
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 18 
            COM(3,4,[0, 1, 2, 2]), //XXXX
            PTR(17),
        ], 
         // FUN4Adjoxo.hasLine
        vec![ // 19 
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            PTR(58),
            PTR(57),
            PTR(53),
        ], 
        vec![ // 20 
            COM(4,2,[3, 0, 1]), //XXX
            INT(7),
            COM(2,0,[0]), //X
        ], 
        vec![ // 21 
            COM(4,2,[3, 0, 1]), //XXX
            INT(5),
            PTR(20),
        ], 
        vec![ // 22 
            COM(4,2,[3, 0, 1]), //XXX
            INT(3),
            PTR(21),
        ], 
        vec![ // 23 
            PTR(59),
            PTR(22),
        ], 
        vec![ // 24 
            COM(4,2,[3, 0, 1]), //XXX
            INT(9),
            COM(2,0,[0]), //X
        ], 
        vec![ // 25 
            COM(4,2,[3, 0, 1]), //XXX
            INT(5),
            PTR(24),
        ], 
        vec![ // 26 
            COM(4,2,[3, 0, 1]), //XXX
            INT(1),
            PTR(25),
        ], 
        vec![ // 27 
            PTR(59),
            PTR(26),
        ], 
        vec![ // 28 
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            PTR(58),
            PTR(27),
            PTR(23),
        ], 
        vec![ // 29 
            COM(4,2,[3, 0, 1]), //XXX
            INT(9),
            COM(2,0,[0]), //X
        ], 
        vec![ // 30 
            COM(4,2,[3, 0, 1]), //XXX
            INT(6),
            PTR(29),
        ], 
        vec![ // 31 
            COM(4,2,[3, 0, 1]), //XXX
            INT(3),
            PTR(30),
        ], 
        vec![ // 32 
            PTR(59),
            PTR(31),
        ], 
        vec![ // 33 
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            PTR(58),
            PTR(32),
            PTR(28),
        ], 
        vec![ // 34 
            COM(4,2,[3, 0, 1]), //XXX
            INT(8),
            COM(2,0,[0]), //X
        ], 
        vec![ // 35 
            COM(4,2,[3, 0, 1]), //XXX
            INT(5),
            PTR(34),
        ], 
        vec![ // 36 
            COM(4,2,[3, 0, 1]), //XXX
            INT(2),
            PTR(35),
        ], 
        vec![ // 37 
            PTR(59),
            PTR(36),
        ], 
        vec![ // 38 
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            PTR(58),
            PTR(37),
            PTR(33),
        ], 
        vec![ // 39 
            COM(4,2,[3, 0, 1]), //XXX
            INT(7),
            COM(2,0,[0]), //X
        ], 
        vec![ // 40 
            COM(4,2,[3, 0, 1]), //XXX
            INT(4),
            PTR(39),
        ], 
        vec![ // 41 
            COM(4,2,[3, 0, 1]), //XXX
            INT(1),
            PTR(40),
        ], 
        vec![ // 42 
            PTR(59),
            PTR(41),
        ], 
        vec![ // 43 
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            PTR(58),
            PTR(42),
            PTR(38),
        ], 
        vec![ // 44 
            COM(4,2,[3, 0, 1]), //XXX
            INT(9),
            COM(2,0,[0]), //X
        ], 
        vec![ // 45 
            COM(4,2,[3, 0, 1]), //XXX
            INT(8),
            PTR(44),
        ], 
        vec![ // 46 
            COM(4,2,[3, 0, 1]), //XXX
            INT(7),
            PTR(45),
        ], 
        vec![ // 47 
            PTR(59),
            PTR(46),
        ], 
        vec![ // 48 
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            PTR(58),
            PTR(47),
            PTR(43),
        ], 
        vec![ // 49 
            COM(4,2,[3, 0, 1]), //XXX
            INT(6),
            COM(2,0,[0]), //X
        ], 
        vec![ // 50 
            COM(4,2,[3, 0, 1]), //XXX
            INT(5),
            PTR(49),
        ], 
        vec![ // 51 
            COM(4,2,[3, 0, 1]), //XXX
            INT(4),
            PTR(50),
        ], 
        vec![ // 52 
            PTR(59),
            PTR(51),
        ], 
        vec![ // 53 
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            PTR(58),
            PTR(52),
            PTR(48),
        ], 
        vec![ // 54 
            COM(4,2,[3, 0, 1]), //XXX
            INT(3),
            COM(2,0,[0]), //X
        ], 
        vec![ // 55 
            COM(4,2,[3, 0, 1]), //XXX
            INT(2),
            PTR(54),
        ], 
        vec![ // 56 
            COM(4,2,[3, 0, 1]), //XXX
            INT(1),
            PTR(55),
        ], 
        vec![ // 57 
            PTR(59),
            PTR(56),
        ], 
         // FUN5Data.Bool.||
        vec![ // 58 
            COM(3,2,[1, 2, 0]), //XXX
            COM(2,0,[1]), //X
        ], 
         // FUN6Adjoxo.subset
        vec![ // 59 
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(60),
            PTR(62),
        ], 
         // FUN7NanoPrelude.null
        vec![ // 60 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[1]), //X
            PTR(61),
        ], 
        vec![ // 61 
            COM(3,0,[0]), //X
            COM(2,0,[0]), //X
        ], 
         // FUN8Adjoxo.diff
        vec![ // 62 
            COM(4,5,[0, 3, 1, 2]), //X(XX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(2,0,[0]), //X
            PTR(69),
        ], 
        vec![ // 63 
            COM(6,34,[0, 1, 2, 3, 5, 4]), //X(XX(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
        ], 
        vec![ // 64 
            COM(4,10,[0, 1, 3, 2, 1]), //X(XX)XX
            PTR(63),
            COM(4,2,[3, 0, 1]), //XXX
            PTR(62),
        ], 
        vec![ // 65 
            COM(5,34,[0, 1, 0, 2, 4, 3]), //X(XX(XX))X
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            COM(5,38,[0, 1, 4, 2, 3, 4]), //X(XX)X(XX)
            PTR(14),
            PTR(62),
        ], 
        vec![ // 66 
            COM(5,39,[0, 1, 2, 4, 3, 4]), //XX(XX)(XX)
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(3,6,[0, 2, 1, 2]), //XX(XX)
        ], 
        vec![ // 67 
            COM(4,20,[0, 1, 3, 2, 3]), //X(XX(XX))
            PTR(66),
            PTR(65),
            PTR(64),
        ], 
        vec![ // 68 
            COM(5,48,[0, 4, 1, 2, 3, 4]), //XX(XX(XX))
            PTR(67),
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(62),
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 69 
            COM(5,39,[0, 3, 1, 4, 2, 4]), //XX(XX)(XX)
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            COM(4,2,[3, 0, 1]), //XXX
            PTR(68),
        ], 
         // FUN9Adjoxo.report
        vec![ // 70 
            COM(5,10,[0, 4, 1, 2, 3]), //X(XX)XX
            COM(4,41,[0, 1, 2, 3, 1, 3]), //X(X(XX))(XX)
            INT(3),
            PTR(71),
            PTR(72),
        ], 
         // FUN10Adjoxo.side
        vec![ // 71 
            COM(3,2,[2, 0, 1]), //XXX
            INT(0),
            INT(88),
        ], 
         // FUN11Adjoxo.opp
        vec![ // 72 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[1]), //X
            COM(2,0,[0]), //X
        ], 
         // FUN12Adjoxo.analysis
        vec![ // 73 
            COM(5,11,[0, 1, 2, 4, 3]), //XX(XX)X
            COM(4,11,[0, 3, 1, 3, 2]), //XX(XX)X
            PTR(19),
            PTR(79),
            COM(3,0,[1]), //X
        ], 
        vec![ // 74 
            PTR(98),
            INT(1),
            INT(9),
        ], 
        vec![ // 75 
            PTR(62),
            PTR(74),
        ], 
        vec![ // 76 
            PTR(82),
            PTR(86),
        ], 
        vec![ // 77 
            COM(5,57,[0, 1, 2, 4, 3, 4]), //X(X(XX)(XX))
            PTR(76),
            PTR(88),
        ], 
        vec![ // 78 
            COM(5,45,[0, 1, 4, 2, 3, 4]), //X(XX)(X(XX))
            PTR(77),
            PTR(91),
            PTR(62),
            PTR(75),
        ], 
        vec![ // 79 
            COM(5,29,[0, 1, 4, 2, 4, 3]), //X(XX)(XX)X
            COM(4,11,[0, 3, 1, 3, 2]), //XX(XX)X
            PTR(80),
            PTR(78),
            COM(3,0,[0]), //X
        ], 
         // FUN13Adjoxo.gridFull
        vec![ // 80 
            COM(3,2,[0, 2, 1]), //XXX
            PTR(81),
            INT(9),
        ], 
        vec![ // 81 
            COM(6,26,[0, 1, 5, 2, 3, 4]), //X(XXX)XX
            COM(5,10,[0, 1, 4, 2, 3]), //X(XX)XX
            PTR(16),
            PRM(ADD,false),
            PTR(16),
            PRM(EQ,false),
        ], 
         // FUN14NanoPrelude.foldr1
        vec![ // 82 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(85),
        ], 
        vec![ // 83 
            COM(5,17,[0, 1, 2, 3, 4]), //XX(X(XX))
            COM(3,1,[0, 1]), //XX
            COM(2,0,[0]), //X
        ], 
        vec![ // 84 
            COM(5,33,[0, 1, 2, 4, 3, 4]), //X(X(XX)X)X
            COM(3,6,[2, 1, 0, 2]), //XX(XX)
            PTR(83),
        ], 
        vec![ // 85 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            ERR(0),
            PTR(84),
        ], 
         // FUN15Adjoxo.bestOf
        vec![ // 86 
            COM(4,4,[0, 2, 3, 1]), //XXXX
            PTR(87),
            COM(3,0,[2]), //X
        ], 
        vec![ // 87 
            COM(4,32,[2, 3, 0, 0, 1, 3]), //X(XXXX)X
            COM(3,0,[0]), //X
            COM(3,0,[2]), //X
        ], 
         // FUN16NanoPrelude.map
        vec![ // 88 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(90),
        ], 
        vec![ // 89 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 90 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(89),
        ], 
         // FUN17Adjoxo.moveval
        vec![ // 91 
            COM(6,25,[0, 1, 2, 5, 3, 4]), //XX(XX)XX
            COM(5,21,[0, 1, 2, 4, 3]), //X(X(XXX))
            PTR(92),
            PTR(73),
            PTR(93),
        ], 
         // FUN18Adjoxo.inverse
        vec![ // 92 
            COM(4,4,[3, 0, 1, 2]), //XXXX
            COM(3,0,[0]), //X
            COM(3,0,[2]), //X
            COM(3,0,[1]), //X
        ], 
         // FUN19Adjoxo.insert
        vec![ // 93 
            COM(4,20,[0, 1, 3, 2, 3]), //X(XX(XX))
            Y,
            PTR(97),
            PTR(96),
        ], 
        vec![ // 94 
            COM(5,11,[0, 1, 4, 2, 3]), //XX(XX)X
            COM(5,29,[0, 1, 4, 2, 4, 3]), //X(XX)(XX)X
            COM(5,41,[0, 1, 2, 4, 3, 4]), //X(X(XX))(XX)
            PRM(LE,false),
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 95 
            COM(5,11,[0, 1, 2, 4, 3]), //XX(XX)X
            COM(6,24,[0, 1, 5, 2, 3, 4]), //X(XX)XXX
            COM(5,48,[0, 4, 1, 2, 3, 4]), //XX(XX(XX))
            PTR(94),
            COM(3,3,[0, 1, 2]), //X(XX)
        ], 
        vec![ // 96 
            COM(4,11,[0, 3, 1, 3, 2]), //XX(XX)X
            PTR(95),
            COM(4,2,[3, 0, 1]), //XXX
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 97 
            COM(5,16,[0, 1, 2, 4, 3]), //XX(XXX)
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,2,[2, 0, 1]), //XXX
            COM(4,2,[3, 0, 1]), //XXX
            COM(2,0,[0]), //X
        ], 
         // FUN20NanoPrelude.enumFromTo
        vec![ // 98 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(100),
            PTR(105),
        ], 
        vec![ // 99 
            COM(3,2,[2, 0, 1]), //XXX
            PRM(LE,false),
        ], 
        vec![ // 100 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(101),
            PTR(99),
        ], 
         // FUN21NanoPrelude.takeWhile
        vec![ // 101 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(104),
        ], 
        vec![ // 102 
            COM(5,40,[0, 3, 4, 1, 2, 4]), //X(XXX)(XX)
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            COM(2,0,[0]), //X
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 103 
            COM(4,4,[0, 1, 3, 2]), //XXXX
            PTR(102),
        ], 
        vec![ // 104 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(103),
        ], 
         // FUN22NanoPrelude.enumFrom
        vec![ // 105 
            COM(5,49,[0, 4, 1, 4, 2, 3]), //XX(X(XXX))
            COM(4,2,[3, 0, 1]), //XXX
            PTR(105),
            PRM(ADD,false),
            INT(1),
        ], 
    ]
});