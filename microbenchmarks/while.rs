use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 31
// Apps in this file: 136
// Combinators in this file: 199
#[rustfmt::skip]
 pub static while: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0While.main
        vec![ // 0 
            PTR(55),
            PTR(54),
            INT(5),
            COM(1,0,[0]), //X
        ], 
        vec![ // 1 
            COM(3,2,[2, 0, 1]), //XXX
            INT(5),
            INT(0),
        ], 
        vec![ // 2 
            COM(4,2,[3, 0, 1]), //XXX
            PTR(1),
            COM(2,0,[0]), //X
        ], 
        vec![ // 3 
            COM(3,2,[2, 0, 1]), //XXX
            INT(4),
            INT(0),
        ], 
        vec![ // 4 
            COM(4,2,[3, 0, 1]), //XXX
            PTR(3),
            PTR(2),
        ], 
        vec![ // 5 
            COM(3,2,[2, 0, 1]), //XXX
            INT(3),
            INT(17),
        ], 
        vec![ // 6 
            COM(4,2,[3, 0, 1]), //XXX
            PTR(5),
            PTR(4),
        ], 
        vec![ // 7 
            COM(3,2,[2, 0, 1]), //XXX
            INT(2),
            INT(0),
        ], 
        vec![ // 8 
            COM(4,2,[3, 0, 1]), //XXX
            PTR(7),
            PTR(6),
        ], 
        vec![ // 9 
            COM(3,2,[2, 0, 1]), //XXX
            INT(1),
            INT(0),
        ], 
        vec![ // 10 
            COM(4,2,[3, 0, 1]), //XXX
            PTR(9),
            PTR(8),
        ], 
        vec![ // 11 
            COM(3,2,[2, 0, 1]), //XXX
            INT(0),
            INT(0),
        ], 
        vec![ // 12 
            COM(4,2,[3, 0, 1]), //XXX
            PTR(11),
            PTR(10),
        ], 
        vec![ // 13 
            PTR(132),
            INT(1),
        ], 
        vec![ // 14 
            COM(5,1,[4, 0]), //XX
            INT(4),
        ], 
        vec![ // 15 
            PTR(134),
            PTR(14),
            PTR(13),
        ], 
        vec![ // 16 
            PTR(128),
            INT(4),
            PTR(15),
        ], 
        vec![ // 17 
            PTR(132),
            INT(1),
        ], 
        vec![ // 18 
            COM(5,1,[4, 0]), //XX
            INT(5),
        ], 
        vec![ // 19 
            PTR(135),
            PTR(18),
            PTR(17),
        ], 
        vec![ // 20 
            PTR(128),
            INT(5),
            PTR(19),
        ], 
        vec![ // 21 
            PTR(132),
            INT(0),
        ], 
        vec![ // 22 
            COM(5,1,[4, 0]), //XX
            INT(0),
        ], 
        vec![ // 23 
            PTR(131),
            PTR(22),
            PTR(21),
        ], 
        vec![ // 24 
            PTR(127),
            PTR(23),
            PTR(20),
            COM(5,0,[3]), //X
        ], 
        vec![ // 25 
            PTR(132),
            INT(1),
        ], 
        vec![ // 26 
            COM(5,1,[4, 0]), //XX
            INT(2),
        ], 
        vec![ // 27 
            PTR(135),
            PTR(26),
            PTR(25),
        ], 
        vec![ // 28 
            PTR(128),
            INT(2),
            PTR(27),
        ], 
        vec![ // 29 
            COM(5,1,[4, 0]), //XX
            INT(1),
        ], 
        vec![ // 30 
            COM(5,1,[4, 0]), //XX
            INT(0),
        ], 
        vec![ // 31 
            PTR(134),
            PTR(30),
            PTR(29),
        ], 
        vec![ // 32 
            PTR(128),
            INT(0),
            PTR(31),
        ], 
        vec![ // 33 
            PTR(93),
            PTR(32),
            PTR(28),
        ], 
        vec![ // 34 
            COM(5,1,[4, 0]), //XX
            INT(0),
        ], 
        vec![ // 35 
            COM(5,1,[4, 0]), //XX
            INT(1),
        ], 
        vec![ // 36 
            PTR(133),
            PTR(35),
            PTR(34),
        ], 
        vec![ // 37 
            COM(7,2,[6, 0, 1]), //XXX
            PTR(36),
            PTR(33),
        ], 
        vec![ // 38 
            COM(5,1,[4, 0]), //XX
            INT(4),
        ], 
        vec![ // 39 
            PTR(128),
            INT(1),
            PTR(38),
        ], 
        vec![ // 40 
            COM(5,1,[4, 0]), //XX
            INT(3),
        ], 
        vec![ // 41 
            PTR(128),
            INT(0),
            PTR(40),
        ], 
        vec![ // 42 
            PTR(93),
            PTR(41),
            PTR(39),
        ], 
        vec![ // 43 
            PTR(93),
            PTR(42),
            PTR(37),
        ], 
        vec![ // 44 
            PTR(93),
            PTR(43),
            PTR(24),
        ], 
        vec![ // 45 
            PTR(93),
            PTR(44),
            PTR(16),
        ], 
        vec![ // 46 
            PTR(132),
            INT(0),
        ], 
        vec![ // 47 
            COM(5,1,[4, 0]), //XX
            INT(4),
        ], 
        vec![ // 48 
            PTR(131),
            PTR(47),
            PTR(46),
        ], 
        vec![ // 49 
            PTR(130),
            PTR(48),
        ], 
        vec![ // 50 
            COM(7,2,[6, 0, 1]), //XXX
            PTR(49),
            PTR(45),
        ], 
        vec![ // 51 
            COM(5,1,[4, 0]), //XX
            INT(3),
        ], 
        vec![ // 52 
            PTR(128),
            INT(4),
            PTR(51),
        ], 
        vec![ // 53 
            PTR(93),
            PTR(52),
            PTR(50),
        ], 
        vec![ // 54 
            PTR(58),
            PTR(53),
            PTR(12),
        ], 
         // FUN1While.value
        vec![ // 55 
            COM(6,25,[0, 1, 5, 2, 3, 4]), //XX(XX)XX
            COM(5,16,[0, 1, 2, 4, 3]), //XX(XXX)
            COM(3,3,[0, 1, 2]), //X(XX)
            ERR(42),
            PTR(57),
            COM(1,0,[0]), //X
        ], 
        vec![ // 56 
            COM(6,32,[0, 5, 1, 2, 3, 4]), //X(XXXX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            PRM(EQ,false),
        ], 
        vec![ // 57 
            COM(5,52,[0, 1, 2, 4, 3, 4]), //X(X(XX)XX)
            COM(5,19,[3, 0, 1, 2, 4]), //X(X(XX)X)
            COM(5,32,[0, 1, 4, 2, 3, 3]), //X(XXXX)X
            PTR(56),
            PTR(55),
        ], 
         // FUN2While.ssos
        vec![ // 58 
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(59),
            COM(4,2,[3, 0, 1]), //XXX
        ], 
         // FUN3While.run
        vec![ // 59 
            COM(3,2,[2, 0, 1]), //XXX
            COM(1,0,[0]), //X
            PTR(60),
        ], 
        vec![ // 60 
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(59),
            PTR(61),
        ], 
         // FUN4While.sosstm
        vec![ // 61 
            COM(5,10,[0, 1, 4, 2, 3]), //X(XX)XX
            COM(4,39,[0, 3, 1, 3, 2, 3]), //XX(XX)(XX)
            PTR(69),
            PTR(92),
            PTR(63),
        ], 
        vec![ // 62 
            COM(5,38,[0, 1, 4, 2, 3, 4]), //X(XX)X(XX)
            COM(5,34,[0, 1, 4, 2, 4, 3]), //X(XX(XX))X
            PTR(127),
            PTR(93),
            COM(7,2,[6, 0, 1]), //XXX
        ], 
        vec![ // 63 
            COM(6,30,[0, 1, 2, 5, 3, 4]), //XX(XXX)X
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(4,2,[3, 0, 1]), //XXX
            PTR(62),
            COM(5,0,[3]), //X
        ], 
        vec![ // 64 
            COM(6,30,[0, 1, 2, 5, 4, 3]), //XX(XXX)X
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(94),
        ], 
        vec![ // 65 
            COM(4,7,[0, 1, 3, 2]), //X(XXX)
            COM(4,2,[3, 0, 1]), //XXX
            PTR(93),
        ], 
        vec![ // 66 
            COM(6,26,[0, 1, 5, 4, 2, 3]), //X(XXX)XX
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            PTR(61),
            COM(4,2,[3, 0, 1]), //XXX
            PTR(65),
        ], 
        vec![ // 67 
            COM(5,40,[0, 1, 2, 4, 3, 4]), //X(XXX)(XX)
            COM(4,7,[0, 1, 3, 2]), //X(XXX)
            COM(4,4,[0, 3, 1, 2]), //XXXX
            PTR(70),
            PTR(82),
        ], 
        vec![ // 68 
            COM(5,40,[3, 0, 4, 1, 2, 4]), //X(XXX)(XX)
            PTR(67),
            PTR(92),
            PTR(66),
        ], 
        vec![ // 69 
            COM(5,10,[0, 1, 4, 2, 3]), //X(XX)XX
            COM(4,47,[0, 3, 1, 2, 3, 3]), //XX(X(XX)X)
            PTR(68),
            PTR(64),
            PTR(125),
        ], 
         // FUN5While.aval
        vec![ // 70 
            COM(4,14,[0, 2, 3, 1, 3]), //XXX(XX)
            PTR(77),
            PTR(55),
        ], 
        vec![ // 71 
            COM(5,35,[0, 1, 2, 4, 3, 2]), //X(X(XXX))X
            COM(5,12,[0, 1, 4, 2, 3]), //X(XXX)X
            PTR(78),
            PTR(70),
        ], 
        vec![ // 72 
            COM(3,5,[0, 1, 2, 2]), //X(XX)X
            COM(4,4,[0, 3, 1, 2]), //XXXX
            PTR(71),
        ], 
        vec![ // 73 
            COM(5,35,[0, 1, 2, 4, 3, 2]), //X(X(XXX))X
            COM(5,12,[0, 1, 4, 2, 3]), //X(XXX)X
            PTR(78),
            PTR(70),
        ], 
        vec![ // 74 
            COM(5,47,[0, 3, 1, 2, 4, 4]), //XX(X(XX)X)
            COM(4,13,[0, 1, 2, 3, 3]), //X(X(XX))X
            COM(4,4,[0, 3, 1, 2]), //XXXX
            PTR(73),
        ], 
        vec![ // 75 
            COM(5,40,[0, 1, 4, 2, 3, 4]), //X(XXX)(XX)
            COM(4,17,[0, 3, 1, 2, 3]), //XX(X(XX))
        ], 
        vec![ // 76 
            COM(5,10,[0, 1, 4, 2, 3]), //X(XX)XX
            PTR(75),
            PTR(74),
            PTR(79),
            PTR(72),
        ], 
        vec![ // 77 
            COM(6,32,[0, 1, 4, 5, 2, 3]), //X(XXXX)X
            COM(4,16,[0, 3, 1, 2, 3]), //XX(XXX)
            PTR(76),
            PTR(81),
            COM(3,2,[0, 2, 1]), //XXX
        ], 
         // FUN6While.seqq
        vec![ // 78 
            COM(4,6,[0, 2, 1, 3]), //XX(XX)
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(3,3,[0, 1, 2]), //X(XX)
        ], 
         // FUN7While.add
        vec![ // 79 
            COM(5,11,[0, 1, 4, 2, 3]), //XX(XX)X
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(80),
            PRM(ADD,false),
        ], 
         // FUN8While.int
        vec![ // 80 
            COM(5,26,[0, 4, 1, 2, 3, 4]), //X(XXX)XX
            COM(4,15,[0, 3, 2, 3, 1]), //X(XX)(XX)
            PRM(EQ,false),
            INT(0),
            INT(0),
        ], 
         // FUN9While.sub
        vec![ // 81 
            COM(5,11,[0, 1, 4, 2, 3]), //XX(XX)X
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(80),
            PRM(SUB,fasle),
        ], 
         // FUN10While.update
        vec![ // 82 
            COM(4,6,[0, 2, 1, 3]), //XX(XX)
            PTR(90),
            PTR(89),
        ], 
        vec![ // 83 
            COM(5,9,[0, 4, 1, 2, 3]), //XXXXX
            PTR(82),
        ], 
        vec![ // 84 
            COM(5,12,[0, 4, 1, 2, 3]), //X(XXX)X
            COM(6,33,[0, 1, 2, 5, 3, 4]), //X(X(XX)X)X
            PRM(EQ,false),
        ], 
        vec![ // 85 
            COM(6,35,[0, 1, 2, 5, 3, 4]), //X(X(XXX))X
            COM(5,25,[0, 4, 1, 4, 2, 3]), //XX(XX)XX
        ], 
        vec![ // 86 
            COM(4,10,[0, 1, 3, 2, 3]), //X(XX)XX
            PTR(85),
            PTR(84),
            PTR(82),
        ], 
        vec![ // 87 
            COM(6,31,[0, 1, 2, 3, 5, 4]), //XX(X(XX))X
            COM(5,47,[0, 1, 2, 3, 4, 4]), //XX(X(XX)X)
            COM(5,56,[3, 0, 4, 2, 1, 4]), //X(XXX(XX))
        ], 
        vec![ // 88 
            COM(5,38,[0, 1, 4, 2, 3, 4]), //X(XX)X(XX)
            PTR(87),
            PTR(86),
            PTR(91),
            PTR(83),
        ], 
        vec![ // 89 
            COM(4,42,[0, 2, 3, 1, 3, 2]), //XXX(XXX)
            PTR(88),
            PTR(91),
        ], 
        vec![ // 90 
            COM(5,41,[0, 2, 4, 1, 3, 4]), //X(X(XX))(XX)
            COM(3,7,[0, 1, 2, 2]), //X(XXX)
            COM(2,0,[0]), //X
        ], 
         // FUN11While.upd
        vec![ // 91 
            COM(6,54,[2, 0, 1, 3, 4, 5]), //X(X(XXX)X)
            COM(4,2,[3, 0, 1]), //XXX
            COM(3,2,[2, 0, 1]), //XXX
        ], 
         // FUN12While.Final
        vec![ // 92 
            COM(3,3,[0, 2, 1]), //X(XX)
            COM(2,0,[0]), //X
        ], 
         // FUN13While.Comp
        vec![ // 93 
            COM(6,58,[0, 1, 0, 5, 2, 3]), //X(XX(XXX))
            COM(2,0,[0]), //X
            COM(3,1,[0, 1]), //XX
        ], 
         // FUN14While.bval
        vec![ // 94 
            COM(6,24,[0, 1, 5, 2, 3, 4]), //X(XX)XXX
            COM(5,29,[0, 1, 4, 2, 4, 3]), //X(XX)(XX)X
            COM(5,59,[0, 1, 4, 2, 3, 4]), //X(XX(X(XX)))
            PTR(117),
            PTR(96),
            PTR(123),
        ], 
        vec![ // 95 
            COM(5,35,[0, 1, 2, 4, 3, 2]), //X(X(XXX))X
            COM(5,12,[0, 1, 4, 2, 3]), //X(XXX)X
            PTR(78),
            PTR(94),
        ], 
        vec![ // 96 
            COM(4,9,[0, 1, 3, 1, 2]), //XXXXX
            PTR(95),
        ], 
        vec![ // 97 
            COM(5,35,[0, 1, 2, 4, 3, 2]), //X(X(XXX))X
            COM(5,12,[0, 1, 4, 2, 3]), //X(XXX)X
            PTR(78),
            PTR(70),
        ], 
        vec![ // 98 
            COM(4,9,[0, 1, 3, 1, 2]), //XXXXX
            PTR(97),
        ], 
        vec![ // 99 
            COM(4,6,[0, 2, 1, 3]), //XX(XX)
            PTR(98),
            PTR(122),
        ], 
        vec![ // 100 
            COM(3,2,[2, 0, 1]), //XXX
            PRM(LT,false),
            INT(2),
        ], 
        vec![ // 101 
            COM(5,40,[0, 4, 1, 2, 3, 4]), //X(XXX)(XX)
            COM(5,57,[0, 1, 4, 2, 4, 3]), //X(X(XX)(XX))
            PRM(LT,false),
            INT(1),
            PTR(100),
        ], 
        vec![ // 102 
            COM(5,39,[0, 1, 4, 2, 3, 4]), //XX(XX)(XX)
            COM(4,4,[0, 3, 1, 2]), //XXXX
            PTR(101),
            COM(2,0,[0]), //X
        ], 
        vec![ // 103 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(102),
            PTR(99),
        ], 
        vec![ // 104 
            COM(5,35,[0, 1, 2, 4, 3, 2]), //X(X(XXX))X
            COM(5,12,[0, 1, 4, 2, 3]), //X(XXX)X
            PTR(78),
            PTR(70),
        ], 
        vec![ // 105 
            COM(4,9,[0, 1, 3, 1, 2]), //XXXXX
            PTR(104),
        ], 
        vec![ // 106 
            COM(4,4,[0, 3, 1, 2]), //XXXX
            PTR(94),
        ], 
        vec![ // 107 
            COM(4,6,[0, 2, 1, 3]), //XX(XX)
            PTR(106),
            PTR(118),
        ], 
        vec![ // 108 
            COM(3,2,[2, 0, 1]), //XXX
            PRM(LT,false),
            INT(5),
        ], 
        vec![ // 109 
            COM(5,40,[0, 4, 1, 2, 3, 4]), //X(XXX)(XX)
            COM(5,57,[0, 1, 4, 2, 4, 3]), //X(X(XX)(XX))
            PRM(LT,false),
            INT(4),
            PTR(108),
        ], 
        vec![ // 110 
            COM(5,39,[0, 1, 4, 2, 3, 4]), //XX(XX)(XX)
            COM(4,4,[0, 3, 1, 2]), //XXXX
            PTR(109),
            COM(2,0,[1]), //X
        ], 
        vec![ // 111 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(110),
            PTR(107),
        ], 
        vec![ // 112 
            COM(5,40,[0, 4, 1, 2, 3, 4]), //X(XXX)(XX)
            COM(4,20,[0, 1, 3, 3, 2]), //X(XX(XX))
            PRM(LT,false),
            INT(3),
        ], 
        vec![ // 113 
            COM(6,49,[0, 1, 2, 3, 4, 5]), //XX(X(XXX))
            COM(5,40,[0, 1, 4, 2, 3, 4]), //X(XXX)(XX)
            COM(4,48,[0, 3, 1, 3, 3, 2]), //XX(XX(XX))
            PTR(112),
            PTR(111),
        ], 
        vec![ // 114 
            COM(5,43,[0, 1, 4, 2, 3, 4]), //XXX(X(XX))
            PTR(113),
        ], 
        vec![ // 115 
            COM(4,11,[0, 3, 1, 3, 2]), //XX(XX)X
            PTR(114),
            PTR(105),
            PTR(121),
        ], 
        vec![ // 116 
            COM(5,56,[0, 1, 2, 4, 3, 4]), //X(XXX(XX))
            COM(3,2,[0, 2, 1]), //XXX
            PTR(115),
        ], 
        vec![ // 117 
            COM(3,6,[0, 2, 1, 2]), //XX(XX)
            PTR(116),
            PTR(103),
        ], 
         // FUN15While.notk
        vec![ // 118 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(119),
            PTR(120),
        ], 
         // FUN16While.bool
        vec![ // 119 
            COM(4,15,[2, 3, 0, 3, 1]), //X(XX)(XX)
            COM(2,0,[0]), //X
            COM(2,0,[1]), //X
        ], 
         // FUN17Data.Bool.not
        vec![ // 120 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[1]), //X
            COM(2,0,[0]), //X
        ], 
         // FUN18While.leq
        vec![ // 121 
            COM(5,11,[0, 1, 4, 2, 3]), //XX(XX)X
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(119),
            PRM(LE,false),
        ], 
         // FUN19While.eq
        vec![ // 122 
            COM(5,11,[0, 1, 4, 2, 3]), //XX(XX)X
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(119),
            PRM(EQ,false),
        ], 
         // FUN20While.andk
        vec![ // 123 
            COM(5,11,[0, 1, 2, 4, 3]), //XX(XX)X
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(119),
            PTR(124),
        ], 
         // FUN21Data.Bool.&&
        vec![ // 124 
            COM(2,1,[1, 0]), //XX
            COM(2,0,[0]), //X
        ], 
         // FUN22While.cond
        vec![ // 125 
            COM(4,16,[0, 2, 1, 3, 2]), //XX(XXX)
            PTR(126),
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 126 
            COM(5,12,[0, 1, 4, 2, 3]), //X(XXX)X
            COM(3,2,[2, 0, 1]), //XXX
            COM(4,2,[3, 0, 1]), //XXX
        ], 
         // FUN23While.If
        vec![ // 127 
            COM(7,23,[0, 1, 2, 3, 4, 5]), //XXXXXX
            COM(7,46,[0, 1, 6, 2, 3, 4]), //XX(XXXX)
            COM(3,1,[0, 1]), //XX
            COM(2,0,[0]), //X
        ], 
         // FUN24While.Ass
        vec![ // 128 
            COM(6,46,[0, 1, 2, 3, 4, 5]), //XX(XXXX)
            COM(3,1,[0, 1]), //XX
            COM(2,0,[0]), //X
            PTR(129),
        ], 
        vec![ // 129 
            COM(5,16,[0, 1, 4, 2, 3]), //XX(XXX)
            COM(3,1,[0, 1]), //XX
            COM(2,0,[0]), //X
        ], 
         // FUN25While.Neg
        vec![ // 130 
            COM(4,6,[3, 0, 1, 2]), //XX(XX)
            INT(4),
            COM(2,1,[1, 0]), //XX
        ], 
         // FUN26While.Eq
        vec![ // 131 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            INT(1),
            COM(3,2,[2, 0, 1]), //XXX
        ], 
         // FUN27While.N
        vec![ // 132 
            COM(5,6,[0, 1, 4, 2]), //XX(XX)
            COM(3,1,[0, 1]), //XX
            COM(2,0,[0]), //X
        ], 
         // FUN28While.Le
        vec![ // 133 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            INT(3),
            COM(3,2,[2, 0, 1]), //XXX
        ], 
         // FUN29While.Sub
        vec![ // 134 
            COM(6,7,[0, 5, 1, 2]), //X(XXX)
            COM(2,0,[0]), //X
        ], 
         // FUN30While.Add
        vec![ // 135 
            COM(5,58,[0, 1, 0, 4, 2, 3]), //X(XX(XXX))
            COM(2,0,[0]), //X
            COM(3,1,[0, 1]), //XX
        ], 
    ]
});