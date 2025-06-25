use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 37
// Apps in this file: 127
// Combinators in this file: 204
#[rustfmt::skip]
 pub static Clausify: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Clausify.main
        vec![ // 0 
            PTR(16),
            PTR(15),
        ], 
        vec![ // 1 
            COM(5,1,[4, 0]), //XX
            INT(0),
        ], 
        vec![ // 2 
            COM(5,1,[4, 0]), //XX
            INT(0),
        ], 
        vec![ // 3 
            PTR(124),
            PTR(2),
            PTR(1),
        ], 
        vec![ // 4 
            COM(5,1,[4, 0]), //XX
            INT(0),
        ], 
        vec![ // 5 
            PTR(124),
            PTR(4),
            PTR(3),
        ], 
        vec![ // 6 
            COM(5,1,[4, 0]), //XX
            INT(0),
        ], 
        vec![ // 7 
            COM(5,1,[4, 0]), //XX
            INT(0),
        ], 
        vec![ // 8 
            PTR(124),
            PTR(7),
            PTR(6),
        ], 
        vec![ // 9 
            COM(5,1,[4, 0]), //XX
            INT(0),
        ], 
        vec![ // 10 
            PTR(124),
            PTR(9),
            PTR(8),
        ], 
        vec![ // 11 
            PTR(124),
            PTR(10),
            PTR(5),
        ], 
        vec![ // 12 
            PTR(121),
            INT(2),
            PTR(11),
        ], 
        vec![ // 13 
            COM(5,1,[4, 0]), //XX
            INT(0),
        ], 
        vec![ // 14 
            PTR(21),
            PTR(99),
            PTR(13),
            PTR(12),
        ], 
        vec![ // 15 
            PTR(24),
            PTR(14),
        ], 
         // FUN1Clausify.display
        vec![ // 16 
            COM(3,2,[2, 0, 1]), //XXX
            INT(0),
            PTR(17),
        ], 
        vec![ // 17 
            COM(5,12,[0, 1, 4, 2, 3]), //X(XXX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(18),
            PRM(ADD,false),
            PTR(16),
        ], 
         // FUN2Clausify.emitClause
        vec![ // 18 
            COM(2,1,[1, 0]), //XX
            PTR(19),
        ], 
        vec![ // 19 
            COM(4,12,[0, 1, 3, 2, 1]), //X(XXX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(20),
            PRM(ADD,false),
        ], 
         // FUN3NanoPrelude.sum
        vec![ // 20 
            PTR(21),
            PRM(ADD,false),
            INT(0),
        ], 
         // FUN4NanoPrelude.foldr
        vec![ // 21 
            COM(4,7,[0, 1, 2, 3]), //X(XXX)
            Y,
            PTR(23),
        ], 
        vec![ // 22 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(3,3,[0, 1, 2]), //X(XX)
        ], 
        vec![ // 23 
            COM(5,16,[4, 2, 0, 1, 3]), //XX(XXX)
            PTR(22),
        ], 
         // FUN5Clausify.clausify
        vec![ // 24 
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            PTR(26),
            PTR(58),
            PTR(25),
        ], 
        vec![ // 25 
            COM(5,22,[0, 1, 2, 3, 4]), //X(X(X(XX)))
            PTR(65),
            PTR(86),
            PTR(96),
            PTR(112),
        ], 
         // FUN6Clausify.uniq
        vec![ // 26 
            PTR(21),
            PTR(28),
            COM(2,0,[0]), //X
        ], 
        vec![ // 27 
            PTR(29),
            PTR(44),
        ], 
        vec![ // 28 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(27),
            PTR(57),
        ], 
         // FUN7Clausify.union
        vec![ // 29 
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            PTR(32),
            PTR(31),
            PTR(40),
        ], 
        vec![ // 30 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(39),
        ], 
        vec![ // 31 
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            PTR(36),
            PTR(30),
        ], 
        vec![ // 32 
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(33),
        ], 
         // FUN8Data.List_Type.++
        vec![ // 33 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            Y,
            PTR(35),
        ], 
        vec![ // 34 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 35 
            COM(4,6,[3, 1, 0, 2]), //XX(XX)
            PTR(34),
        ], 
         // FUN9NanoPrelude.filter
        vec![ // 36 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(38),
        ], 
        vec![ // 37 
            COM(5,38,[0, 2, 4, 3, 1, 4]), //X(XX)X(XX)
            COM(4,45,[0, 1, 3, 2, 1, 3]), //X(XX)(X(XX))
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 38 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(37),
        ], 
         // FUN10Data.Bool.not
        vec![ // 39 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[1]), //X
            COM(2,0,[0]), //X
        ], 
         // FUN11Clausify.contains
        vec![ // 40 
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            Y,
            PTR(42),
            PTR(41),
        ], 
        vec![ // 41 
            COM(6,35,[0, 1, 2, 5, 4, 3]), //X(X(XXX))X
            COM(4,7,[0, 1, 3, 2]), //X(XXX)
            PTR(43),
        ], 
        vec![ // 42 
            COM(6,45,[0, 5, 1, 2, 3, 4]), //X(XX)(X(XX))
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(2,0,[0]), //X
            COM(3,4,[0, 1, 2, 1]), //XXXX
        ], 
         // FUN12Data.Bool.||
        vec![ // 43 
            COM(3,2,[1, 2, 0]), //XXX
            COM(2,0,[1]), //X
        ], 
         // FUN13Clausify.eqClause
        vec![ // 44 
            COM(3,3,[1, 0, 2]), //X(XX)
            PTR(47),
        ], 
        vec![ // 45 
            PTR(49),
            PTR(56),
        ], 
        vec![ // 46 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(48),
        ], 
        vec![ // 47 
            COM(5,31,[0, 3, 1, 2, 4, 2]), //XX(X(XX))X
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            PTR(46),
            PTR(45),
        ], 
         // FUN14Data.Bool.&&
        vec![ // 48 
            COM(2,1,[1, 0]), //XX
            COM(2,0,[0]), //X
        ], 
         // FUN15Clausify.eqList
        vec![ // 49 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(55),
        ], 
        vec![ // 50 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(48),
        ], 
        vec![ // 51 
            COM(6,25,[0, 1, 5, 2, 3, 4]), //XX(XX)XX
            COM(6,31,[0, 1, 2, 3, 5, 4]), //XX(X(XX))X
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            COM(2,0,[0]), //X
            PTR(50),
        ], 
        vec![ // 52 
            COM(4,4,[0, 1, 3, 2]), //XXXX
            PTR(51),
        ], 
        vec![ // 53 
            COM(3,0,[0]), //X
            COM(2,0,[0]), //X
        ], 
        vec![ // 54 
            COM(5,40,[3, 4, 0, 1, 2, 4]), //X(XXX)(XX)
            COM(2,0,[1]), //X
            PTR(53),
        ], 
        vec![ // 55 
            COM(4,7,[0, 1, 2, 3]), //X(XXX)
            PTR(54),
            PTR(52),
        ], 
         // FUN16Clausify.eq
        vec![ // 56 
            COM(2,1,[1, 0]), //XX
            PRM(EQ,false),
        ], 
         // FUN17Clausify.singleton
        vec![ // 57 
            COM(3,2,[0, 2, 1]), //XXX
            COM(4,2,[3, 0, 1]), //XXX
            COM(2,0,[0]), //X
        ], 
         // FUN18Clausify.nonTaut
        vec![ // 58 
            PTR(36),
            PTR(59),
        ], 
         // FUN19Clausify.notTaut
        vec![ // 59 
            COM(2,1,[1, 0]), //XX
            PTR(61),
        ], 
        vec![ // 60 
            PTR(64),
            PTR(56),
        ], 
        vec![ // 61 
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(62),
            PTR(60),
        ], 
         // FUN20NanoPrelude.null
        vec![ // 62 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[1]), //X
            PTR(63),
        ], 
        vec![ // 63 
            COM(3,0,[0]), //X
            COM(2,0,[0]), //X
        ], 
         // FUN21Clausify.inter
        vec![ // 64 
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(36),
            PTR(40),
        ], 
         // FUN22Clausify.clauses
        vec![ // 65 
            PTR(68),
            PTR(67),
        ], 
        vec![ // 66 
            COM(2,2,[1, 0, 0]), //XXX
            COM(2,0,[0]), //X
        ], 
        vec![ // 67 
            PTR(71),
            PTR(66),
        ], 
         // FUN23NanoPrelude.map
        vec![ // 68 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(70),
        ], 
        vec![ // 69 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 70 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(69),
        ], 
         // FUN24Clausify.clause
        vec![ // 71 
            COM(6,54,[4, 0, 1, 5, 3, 2]), //X(X(XXX)X)
            COM(4,42,[0, 2, 3, 1, 2, 3]), //XXX(XXX)
            PTR(80),
            PTR(73),
            PTR(72),
        ], 
        vec![ // 72 
            COM(2,2,[1, 0, 0]), //XXX
            COM(2,0,[0]), //X
        ], 
        vec![ // 73 
            COM(5,12,[0, 1, 4, 2, 3]), //X(XXX)X
            COM(3,2,[2, 0, 1]), //XXX
            PTR(81),
        ], 
        vec![ // 74 
            COM(5,16,[0, 2, 1, 4, 3]), //XX(XXX)
            COM(3,2,[2, 0, 1]), //XXX
            PTR(81),
        ], 
        vec![ // 75 
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            COM(5,37,[4, 0, 0, 1, 2, 3]), //XXXX(XX)
            COM(3,0,[0]), //X
            COM(2,0,[0]), //X
        ], 
        vec![ // 76 
            COM(5,49,[0, 1, 1, 2, 3, 4]), //XX(X(XXX))
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(71),
            COM(3,2,[2, 0, 1]), //XXX
        ], 
        vec![ // 77 
            COM(4,6,[0, 2, 1, 3]), //XX(XX)
            COM(5,39,[0, 1, 2, 4, 3, 4]), //XX(XX)(XX)
            COM(3,0,[0]), //X
        ], 
        vec![ // 78 
            COM(5,38,[0, 1, 4, 2, 3, 4]), //X(XX)X(XX)
            COM(5,45,[0, 1, 4, 2, 3, 4]), //X(XX)(X(XX))
        ], 
        vec![ // 79 
            COM(5,10,[0, 1, 4, 2, 3]), //X(XX)XX
            PTR(78),
            PTR(77),
            PTR(76),
            PTR(75),
        ], 
        vec![ // 80 
            COM(4,4,[0, 2, 3, 1]), //XXXX
            PTR(79),
            PTR(74),
        ], 
         // FUN25Clausify.insert
        vec![ // 81 
            COM(4,20,[0, 1, 3, 2, 3]), //X(XX(XX))
            Y,
            PTR(85),
            PTR(84),
        ], 
        vec![ // 82 
            COM(5,11,[0, 1, 4, 2, 3]), //XX(XX)X
            COM(5,29,[0, 1, 4, 2, 4, 3]), //X(XX)(XX)X
            COM(5,41,[0, 1, 2, 4, 3, 4]), //X(X(XX))(XX)
            PRM(LE,false),
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 83 
            COM(5,11,[0, 1, 2, 4, 3]), //XX(XX)X
            COM(6,24,[0, 1, 5, 2, 3, 4]), //X(XX)XXX
            COM(5,48,[0, 4, 1, 2, 3, 4]), //XX(XX(XX))
            PTR(82),
            COM(3,3,[0, 1, 2]), //X(XX)
        ], 
        vec![ // 84 
            COM(4,11,[0, 3, 1, 3, 2]), //XX(XX)X
            PTR(83),
            COM(4,2,[3, 0, 1]), //XXX
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 85 
            COM(5,16,[0, 1, 2, 4, 3]), //XX(XXX)
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,2,[2, 0, 1]), //XXX
            COM(4,2,[3, 0, 1]), //XXX
            COM(2,0,[0]), //X
        ], 
         // FUN26Clausify.split
        vec![ // 86 
            PTR(87),
            COM(2,0,[0]), //X
        ], 
         // FUN27Clausify.spl
        vec![ // 87 
            COM(4,20,[0, 1, 3, 2, 3]), //X(XX(XX))
            Y,
            PTR(93),
            PTR(88),
        ], 
        vec![ // 88 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(4,2,[3, 0, 1]), //XXX
            COM(5,1,[4, 0]), //XX
        ], 
        vec![ // 89 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(4,2,[3, 0, 1]), //XXX
            PTR(95),
        ], 
        vec![ // 90 
            COM(5,11,[0, 1, 2, 4, 3]), //XX(XX)X
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(4,2,[3, 0, 1]), //XXX
            PTR(94),
        ], 
        vec![ // 91 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(87),
        ], 
        vec![ // 92 
            COM(6,24,[5, 0, 4, 1, 2, 3]), //X(XX)XXX
            PTR(91),
        ], 
        vec![ // 93 
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            PTR(92),
            PTR(90),
            PTR(89),
        ], 
         // FUN28Clausify.Dis
        vec![ // 94 
            COM(6,16,[0, 1, 5, 2, 3]), //XX(XXX)
            COM(3,1,[0, 1]), //XX
            COM(2,0,[0]), //X
        ], 
         // FUN29Clausify.Neg
        vec![ // 95 
            COM(5,3,[0, 4, 1]), //X(XX)
            COM(2,0,[0]), //X
        ], 
         // FUN30Clausify.disin
        vec![ // 96 
            COM(5,9,[4, 0, 1, 2, 3]), //XXXXX
            PTR(98),
            PTR(97),
            PTR(95),
            COM(5,1,[4, 0]), //XX
        ], 
        vec![ // 97 
            COM(4,13,[0, 1, 2, 3, 2]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(100),
            PTR(96),
        ], 
        vec![ // 98 
            COM(4,13,[0, 1, 2, 3, 2]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(99),
            PTR(96),
        ], 
         // FUN31Clausify.Con
        vec![ // 99 
            COM(5,58,[0, 1, 0, 4, 2, 3]), //X(XX(XXX))
            COM(2,0,[0]), //X
            COM(3,1,[0, 1]), //XX
        ], 
         // FUN32Clausify.din
        vec![ // 100 
            COM(5,10,[0, 1, 4, 2, 3]), //X(XX)XX
            COM(4,39,[0, 3, 1, 3, 2, 3]), //XX(XX)(XX)
            PTR(106),
            PTR(102),
            PTR(101),
        ], 
        vec![ // 101 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(107),
            COM(5,1,[4, 0]), //XX
        ], 
        vec![ // 102 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(107),
            PTR(95),
        ], 
        vec![ // 103 
            COM(5,11,[0, 1, 2, 4, 3]), //XX(XX)X
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(107),
            PTR(94),
        ], 
        vec![ // 104 
            COM(5,35,[0, 1, 2, 4, 3, 2]), //X(X(XXX))X
            COM(4,7,[0, 1, 3, 2]), //X(XXX)
            PTR(99),
            PTR(100),
        ], 
        vec![ // 105 
            COM(3,4,[0, 1, 2, 1]), //XXXX
            PTR(104),
        ], 
        vec![ // 106 
            COM(4,15,[2, 0, 3, 1, 3]), //X(XX)(XX)
            PTR(105),
            PTR(103),
        ], 
         // FUN33Clausify.din2
        vec![ // 107 
            COM(5,47,[0, 4, 1, 2, 4, 3]), //XX(X(XX)X)
            PTR(111),
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(94),
            COM(5,1,[4, 0]), //XX
        ], 
        vec![ // 108 
            COM(5,11,[0, 1, 2, 4, 3]), //XX(XX)X
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(94),
            PTR(94),
        ], 
        vec![ // 109 
            COM(4,13,[0, 1, 2, 3, 2]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(99),
        ], 
        vec![ // 110 
            COM(5,41,[0, 1, 2, 4, 3, 4]), //X(X(XX))(XX)
            COM(5,9,[4, 0, 1, 2, 3]), //XXXXX
            PTR(109),
            PTR(100),
            PTR(108),
        ], 
        vec![ // 111 
            COM(5,47,[0, 4, 1, 2, 4, 3]), //XX(X(XX)X)
            PTR(110),
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(94),
            PTR(95),
        ], 
         // FUN34Clausify.negin
        vec![ // 112 
            COM(5,9,[4, 0, 1, 2, 3]), //XXXXX
            PTR(120),
            PTR(119),
            PTR(118),
            COM(5,1,[4, 0]), //XX
        ], 
        vec![ // 113 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(95),
            COM(5,1,[4, 0]), //XX
        ], 
        vec![ // 114 
            COM(5,36,[0, 1, 2, 3, 4, 2]), //X(X(X(XX)))X
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            PTR(99),
            PTR(112),
            PTR(95),
        ], 
        vec![ // 115 
            COM(3,2,[0, 2, 1]), //XXX
            PTR(114),
            PTR(95),
        ], 
        vec![ // 116 
            COM(5,36,[0, 1, 2, 3, 4, 2]), //X(X(X(XX)))X
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            PTR(94),
            PTR(112),
            PTR(95),
        ], 
        vec![ // 117 
            COM(3,2,[0, 2, 1]), //XXX
            PTR(116),
            PTR(95),
        ], 
        vec![ // 118 
            COM(5,9,[4, 0, 1, 2, 3]), //XXXXX
            PTR(117),
            PTR(115),
            PTR(112),
            PTR(113),
        ], 
        vec![ // 119 
            COM(4,13,[0, 1, 2, 3, 2]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(94),
            PTR(112),
        ], 
        vec![ // 120 
            COM(4,13,[0, 1, 2, 3, 2]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(99),
            PTR(112),
        ], 
         // FUN35NanoPrelude.replicate
        vec![ // 121 
            COM(3,2,[0, 2, 1]), //XXX
            PTR(123),
            COM(2,0,[0]), //X
        ], 
        vec![ // 122 
            COM(5,12,[0, 4, 1, 2, 3]), //X(XXX)X
            COM(5,34,[0, 1, 4, 2, 4, 3]), //X(XX(XX))X
            PRM(LE,false),
            INT(0),
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 123 
            COM(5,49,[0, 4, 1, 4, 2, 3]), //XX(X(XXX))
            PTR(122),
            PTR(121),
            PRM(SUB,fasle),
            INT(1),
        ], 
         // FUN36Clausify.eqv
        vec![ // 124 
            COM(3,6,[0, 2, 1, 2]), //XX(XX)
            PTR(126),
            PTR(125),
        ], 
        vec![ // 125 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(94),
            PTR(95),
        ], 
        vec![ // 126 
            COM(5,17,[0, 1, 2, 3, 4]), //XX(X(XX))
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            PTR(99),
            PTR(94),
            PTR(95),
        ], 
    ]
});