use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 38
// Apps in this file: 118
// Combinators in this file: 219
#[rustfmt::skip]
 pub static sumpuz: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Sumpuz.main
        vec![ // 0 
            COM(2,4,[0, 1, 1, 1]), //XXXX
            PTR(10),
            PTR(9),
        ], 
        vec![ // 1 
            COM(4,2,[3, 0, 1]), //XXX
            INT(2),
            COM(2,0,[0]), //X
        ], 
        vec![ // 2 
            COM(4,2,[3, 0, 1]), //XXX
            INT(1),
            PTR(1),
        ], 
        vec![ // 3 
            COM(4,2,[3, 0, 1]), //XXX
            INT(2),
            PTR(2),
        ], 
        vec![ // 4 
            COM(4,2,[3, 0, 1]), //XXX
            PTR(3),
            COM(2,0,[0]), //X
        ], 
        vec![ // 5 
            COM(4,2,[3, 0, 1]), //XXX
            INT(1),
            COM(2,0,[0]), //X
        ], 
        vec![ // 6 
            COM(4,2,[3, 0, 1]), //XXX
            INT(2),
            PTR(5),
        ], 
        vec![ // 7 
            COM(4,2,[3, 0, 1]), //XXX
            INT(1),
            PTR(6),
        ], 
        vec![ // 8 
            COM(4,2,[3, 0, 1]), //XXX
            INT(0),
            PTR(7),
        ], 
        vec![ // 9 
            COM(4,2,[3, 0, 1]), //XXX
            PTR(8),
            PTR(4),
        ], 
         // FUN1Sumpuz.count
        vec![ // 10 
            COM(5,11,[0, 1, 2, 4, 3]), //XX(XX)X
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(11),
            PTR(15),
        ], 
         // FUN2Sumpuz.sumMap
        vec![ // 11 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(3,2,[0, 2, 1]), //XXX
            PTR(12),
            INT(0),
        ], 
         // FUN3Sumpuz.sumMapAcc
        vec![ // 12 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(14),
        ], 
        vec![ // 13 
            COM(6,46,[0, 3, 2, 5, 1, 4]), //XX(XXXX)
            COM(3,2,[0, 2, 1]), //XXX
            PRM(ADD,false),
        ], 
        vec![ // 14 
            COM(5,46,[3, 4, 0, 1, 2, 4]), //XX(XXXX)
            PTR(13),
        ], 
         // FUN4Sumpuz.fx
        vec![ // 15 
            COM(5,12,[0, 1, 4, 3, 2]), //X(XXX)X
            PTR(11),
            PTR(16),
        ], 
         // FUN5Sumpuz.fy
        vec![ // 16 
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(11),
            PTR(17),
        ], 
         // FUN6Sumpuz.fz
        vec![ // 17 
            COM(6,25,[0, 1, 2, 5, 3, 4]), //XX(XX)XX
            COM(5,10,[0, 1, 4, 2, 3]), //X(XX)XX
            COM(4,4,[0, 3, 1, 2]), //XXXX
            PTR(18),
            INT(0),
            INT(1),
        ], 
         // FUN7Sumpuz.valid
        vec![ // 18 
            COM(3,6,[0, 2, 1, 2]), //XX(XX)
            PTR(24),
            PTR(22),
        ], 
        vec![ // 19 
            COM(3,2,[2, 0, 1]), //XXX
            INT(0),
            COM(2,0,[0]), //X
        ], 
        vec![ // 20 
            COM(6,30,[0, 1, 2, 5, 3, 4]), //XX(XXX)X
            COM(5,41,[0, 1, 2, 4, 3, 4]), //X(X(XX))(XX)
            PTR(25),
            PTR(26),
            PRM(EQ,false),
            PTR(26),
        ], 
        vec![ // 21 
            COM(5,10,[0, 1, 4, 2, 3]), //X(XX)XX
            COM(6,53,[0, 1, 2, 3, 5, 4]), //X(XX(XX)X)
            PTR(20),
            COM(4,7,[0, 1, 3, 2]), //X(XXX)
            PTR(29),
        ], 
        vec![ // 22 
            COM(4,11,[0, 3, 1, 3, 2]), //XX(XX)X
            PTR(21),
            PTR(32),
            PTR(19),
        ], 
        vec![ // 23 
            COM(5,22,[0, 1, 2, 3, 4]), //X(X(X(XX)))
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(25),
        ], 
        vec![ // 24 
            COM(6,54,[0, 1, 2, 5, 3, 4]), //X(X(XXX)X)
            COM(3,6,[0, 2, 1, 2]), //XX(XX)
            PTR(23),
            PTR(26),
            PRM(EQ,false),
            PTR(26),
        ], 
         // FUN8Data.Bool.&&
        vec![ // 25 
            COM(2,1,[1, 0]), //XX
            COM(2,0,[0]), //X
        ], 
         // FUN9NanoPrelude.length
        vec![ // 26 
            Y,
            PTR(28),
            INT(0),
        ], 
        vec![ // 27 
            COM(6,49,[0, 1, 4, 5, 2, 3]), //XX(X(XXX))
            COM(4,6,[3, 2, 0, 1]), //XX(XX)
            COM(2,0,[0]), //X
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 28 
            COM(3,4,[0, 1, 2, 2]), //XXXX
            PTR(27),
        ], 
         // FUN10Sumpuz.isSingleton
        vec![ // 29 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[0]), //X
            PTR(31),
        ], 
        vec![ // 30 
            COM(3,0,[0]), //X
            COM(2,0,[0]), //X
        ], 
        vec![ // 31 
            COM(4,2,[3, 0, 1]), //XXX
            COM(2,0,[1]), //X
            PTR(30),
        ], 
         // FUN11Sumpuz.solutions
        vec![ // 32 
            COM(5,15,[0, 1, 3, 2, 4]), //X(XX)(XX)
            COM(3,2,[0, 2, 1]), //XXX
            PTR(51),
            PTR(42),
        ], 
        vec![ // 33 
            COM(6,32,[0, 1, 5, 2, 3, 4]), //X(XXXX)X
            PTR(109),
            PTR(117),
            INT(1),
            INT(0),
            INT(9),
        ], 
        vec![ // 34 
            COM(6,52,[0, 1, 2, 5, 3, 4]), //X(X(XX)XX)
            PTR(109),
            PTR(117),
            PTR(108),
            INT(1),
            INT(0),
        ], 
        vec![ // 35 
            COM(5,44,[0, 1, 4, 2, 4, 3]), //X(XX)(XXX)
            PTR(54),
            PTR(107),
            PTR(34),
            INT(9),
        ], 
        vec![ // 36 
            COM(6,31,[0, 1, 2, 3, 5, 4]), //XX(X(XX))X
            COM(6,25,[0, 1, 2, 5, 3, 4]), //XX(XX)XX
            COM(5,19,[0, 1, 2, 4, 3]), //X(X(XX)X)
            PTR(84),
            PTR(35),
            PTR(54),
        ], 
        vec![ // 37 
            COM(5,12,[0, 1, 4, 2, 3]), //X(XXX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(36),
            PTR(33),
            PTR(53),
        ], 
        vec![ // 38 
            COM(6,36,[0, 1, 2, 3, 5, 4]), //X(X(X(XX)))X
            COM(1,0,[0]), //X
            PTR(84),
        ], 
        vec![ // 39 
            COM(6,58,[0, 1, 2, 3, 5, 4]), //X(XX(XXX))
            COM(5,40,[0, 1, 4, 2, 3, 4]), //X(XXX)(XX)
            COM(5,10,[0, 1, 4, 2, 3]), //X(XX)XX
            PTR(38),
        ], 
        vec![ // 40 
            COM(6,55,[0, 1, 2, 3, 5, 4]), //X(X(X(XX))X)
            COM(5,37,[0, 4, 1, 2, 3, 4]), //XXXX(XX)
            PTR(39),
            PTR(90),
            PTR(52),
        ], 
        vec![ // 41 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(5,37,[0, 4, 1, 2, 3, 4]), //XXXX(XX)
            PTR(40),
            PTR(107),
            PTR(32),
        ], 
        vec![ // 42 
            COM(4,39,[0, 3, 1, 3, 2, 3]), //XX(XX)(XX)
            PTR(41),
            PTR(108),
            PTR(37),
        ], 
        vec![ // 43 
            COM(4,6,[0, 1, 3, 2]), //XX(XX)
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(2,0,[0]), //X
        ], 
        vec![ // 44 
            COM(4,2,[3, 0, 1]), //XXX
            INT(1),
            COM(2,0,[0]), //X
        ], 
        vec![ // 45 
            COM(5,9,[0, 4, 1, 2, 3]), //XXXXX
            PTR(52),
            PRM(EQ,false),
            INT(1),
            COM(2,0,[0]), //X
        ], 
        vec![ // 46 
            COM(6,25,[0, 1, 2, 5, 3, 4]), //XX(XX)XX
            COM(6,60,[0, 1, 2, 5, 3, 4]), //X(X(XXXX))
            COM(3,2,[2, 0, 1]), //XXX
            PTR(45),
            PTR(54),
            PTR(44),
        ], 
        vec![ // 47 
            COM(5,34,[0, 1, 4, 2, 4, 3]), //X(XX(XX))X
            COM(4,6,[0, 3, 1, 2]), //XX(XX)
            PTR(46),
            PTR(53),
            COM(3,0,[0]), //X
        ], 
        vec![ // 48 
            COM(5,9,[0, 4, 1, 2, 3]), //XXXXX
            PTR(52),
            PRM(EQ,false),
            INT(0),
            COM(2,0,[0]), //X
        ], 
        vec![ // 49 
            COM(5,47,[0, 4, 1, 2, 4, 3]), //XX(X(XX)X)
            PTR(48),
            COM(4,2,[3, 0, 1]), //XXX
            PTR(53),
            COM(2,0,[0]), //X
        ], 
        vec![ // 50 
            COM(5,44,[3, 0, 4, 1, 4, 2]), //X(XX)(XXX)
            PTR(49),
            PTR(47),
            COM(2,0,[0]), //X
        ], 
        vec![ // 51 
            COM(5,39,[0, 3, 1, 4, 2, 4]), //XX(XX)(XX)
            COM(5,45,[0, 1, 4, 2, 3, 4]), //X(XX)(X(XX))
            PTR(50),
            PTR(43),
        ], 
         // FUN12NanoPrelude.fst
        vec![ // 52 
            COM(2,1,[1, 0]), //XX
            COM(2,0,[0]), //X
        ], 
         // FUN13NanoPrelude.snd
        vec![ // 53 
            COM(2,1,[1, 0]), //XX
            COM(2,0,[1]), //X
        ], 
         // FUN14Sumpuz.bindings
        vec![ // 54 
            COM(3,2,[0, 2, 1]), //XXX
            PTR(61),
            PTR(56),
        ], 
        vec![ // 55 
            COM(4,2,[3, 1, 0]), //XXX
            COM(2,0,[0]), //X
        ], 
        vec![ // 56 
            COM(6,37,[0, 5, 3, 1, 2, 4]), //XXXX(XX)
            PTR(81),
            COM(2,0,[0]), //X
            PTR(55),
        ], 
        vec![ // 57 
            COM(3,2,[0, 2, 1]), //XXX
            PTR(68),
        ], 
        vec![ // 58 
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            PTR(65),
            PTR(57),
        ], 
        vec![ // 59 
            COM(6,53,[0, 1, 2, 3, 5, 4]), //X(XX(XX)X)
            PTR(58),
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
        ], 
        vec![ // 60 
            COM(6,27,[0, 1, 2, 5, 3, 4]), //X(X(XX))XX
            PTR(59),
            PTR(69),
            PTR(72),
            PTR(73),
            PTR(80),
        ], 
        vec![ // 61 
            COM(5,39,[0, 1, 2, 4, 3, 4]), //XX(XX)(XX)
            COM(5,39,[0, 1, 2, 4, 3, 4]), //XX(XX)(XX)
            COM(4,39,[0, 3, 1, 3, 2, 3]), //XX(XX)(XX)
            PTR(62),
            PTR(60),
        ], 
         // FUN15NanoPrelude.lookup
        vec![ // 62 
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            Y,
            PTR(64),
            PTR(63),
        ], 
        vec![ // 63 
            COM(6,34,[0, 1, 2, 5, 3, 4]), //X(XX(XX))X
            COM(5,19,[3, 0, 2, 4, 1]), //X(X(XX)X)
            COM(5,12,[0, 1, 4, 2, 3]), //X(XXX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            PRM(EQ,false),
            COM(3,1,[2, 0]), //XX
        ], 
        vec![ // 64 
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[0]), //X
        ], 
         // FUN16NanoPrelude.map
        vec![ // 65 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(67),
        ], 
        vec![ // 66 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 67 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(66),
        ], 
         // FUN17Data.List_Type.:
        vec![ // 68 
            COM(4,2,[3, 0, 1]), //XXX
        ], 
         // FUN18NanoPrelude.zip
        vec![ // 69 
            COM(4,5,[0, 3, 1, 2]), //X(XX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(2,0,[0]), //X
            PTR(71),
        ], 
        vec![ // 70 
            COM(6,30,[0, 1, 2, 3, 5, 4]), //XX(XXX)X
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(4,2,[3, 0, 1]), //XXX
            COM(3,2,[2, 0, 1]), //XXX
        ], 
        vec![ // 71 
            COM(6,25,[0, 1, 5, 2, 3, 4]), //XX(XX)XX
            COM(5,11,[0, 1, 2, 4, 3]), //XX(XX)X
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            COM(2,0,[0]), //X
            PTR(70),
            PTR(69),
        ], 
         // FUN19NanoPrelude.repeat
        vec![ // 72 
            COM(3,6,[0, 2, 1, 2]), //XX(XX)
            COM(4,2,[3, 0, 1]), //XXX
            PTR(72),
        ], 
         // FUN20Sumpuz.diff
        vec![ // 73 
            PTR(75),
            PTR(74),
        ], 
        vec![ // 74 
            COM(3,2,[0, 2, 1]), //XXX
            PTR(77),
        ], 
         // FUN21NanoPrelude.foldl
        vec![ // 75 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(76),
        ], 
        vec![ // 76 
            COM(5,34,[0, 1, 3, 2, 4, 4]), //X(XX(XX))X
            COM(3,2,[2, 1, 0]), //XXX
            COM(3,3,[0, 1, 2]), //X(XX)
        ], 
         // FUN22Sumpuz.del
        vec![ // 77 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(79),
        ], 
        vec![ // 78 
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[0]), //X
        ], 
        vec![ // 79 
            COM(6,53,[0, 1, 2, 5, 3, 4]), //X(XX(XX)X)
            PTR(78),
            COM(5,29,[0, 1, 4, 2, 4, 3]), //X(XX)(XX)X
            COM(4,13,[0, 1, 2, 3, 3]), //X(X(XX))X
            PRM(EQ,false),
            COM(4,2,[3, 0, 1]), //XXX
        ], 
         // FUN23Sumpuz.rng
        vec![ // 80 
            PTR(65),
            PTR(53),
        ], 
         // FUN24NanoPrelude.elem
        vec![ // 81 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(83),
        ], 
        vec![ // 82 
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[0]), //X
        ], 
        vec![ // 83 
            COM(6,53,[0, 1, 2, 5, 3, 4]), //X(XX(XX)X)
            PTR(82),
            COM(5,10,[0, 1, 4, 3, 2]), //X(XX)XX
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PRM(EQ,false),
            COM(2,0,[1]), //X
        ], 
         // FUN25Sumpuz.ofAll
        vec![ // 84 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(86),
        ], 
        vec![ // 85 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(87),
        ], 
        vec![ // 86 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(85),
        ], 
         // FUN26Data.List_Type.++
        vec![ // 87 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            Y,
            PTR(89),
        ], 
        vec![ // 88 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 89 
            COM(4,6,[3, 1, 0, 2]), //XX(XX)
            PTR(88),
        ], 
         // FUN27Sumpuz.solns
        vec![ // 90 
            COM(4,7,[0, 1, 2, 3]), //X(XXX)
            PTR(98),
            PTR(95),
        ], 
        vec![ // 91 
            COM(3,2,[0, 2, 1]), //XXX
            PTR(105),
        ], 
        vec![ // 92 
            COM(5,37,[0, 4, 2, 1, 3, 4]), //XXXX(XX)
            PTR(105),
            PRM(ADD,false),
        ], 
        vec![ // 93 
            COM(4,6,[0, 2, 1, 3]), //XX(XX)
            PTR(92),
            PTR(91),
        ], 
        vec![ // 94 
            COM(6,51,[0, 1, 4, 5, 2, 3]), //X(XXXXX)
            PTR(100),
        ], 
        vec![ // 95 
            COM(5,10,[0, 1, 4, 2, 3]), //X(XX)XX
            PTR(94),
            PTR(93),
            PRM(ADD,false),
        ], 
        vec![ // 96 
            COM(6,24,[0, 1, 5, 2, 3, 4]), //X(XX)XXX
            COM(6,33,[0, 1, 2, 5, 3, 4]), //X(X(XX)X)X
            PTR(54),
            COM(4,2,[3, 0, 1]), //XXX
            PTR(53),
            COM(2,0,[0]), //X
        ], 
        vec![ // 97 
            COM(6,53,[0, 1, 2, 3, 5, 4]), //X(XX(XX)X)
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            COM(5,41,[0, 1, 2, 4, 3, 4]), //X(X(XX))(XX)
            PTR(84),
            PTR(99),
            PTR(52),
        ], 
        vec![ // 98 
            COM(6,39,[0, 1, 2, 5, 3, 4]), //XX(XX)(XX)
            COM(4,4,[0, 3, 1, 2]), //XXXX
            PTR(97),
            PTR(96),
        ], 
         // FUN28NanoPrelude.curry
        vec![ // 99 
            COM(4,7,[1, 0, 2, 3]), //X(XXX)
            COM(3,2,[2, 0, 1]), //XXX
        ], 
         // FUN29Sumpuz.divMod10
        vec![ // 100 
            COM(3,6,[0, 2, 1, 2]), //XX(XX)
            PTR(104),
            PTR(101),
        ], 
        vec![ // 101 
            COM(3,2,[2, 0, 1]), //XXX
            INT(0),
        ], 
        vec![ // 102 
            COM(4,7,[0, 3, 1, 2]), //X(XXX)
            COM(3,2,[2, 0, 1]), //XXX
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 103 
            COM(5,12,[0, 4, 1, 2, 3]), //X(XXX)X
            PTR(100),
            PRM(SUB,fasle),
            INT(10),
            PTR(102),
        ], 
        vec![ // 104 
            COM(4,14,[3, 0, 1, 2, 3]), //XXX(XX)
            PRM(LE,false),
            INT(9),
            PTR(103),
        ], 
         // FUN30Sumpuz.img
        vec![ // 105 
            COM(4,7,[0, 1, 3, 2]), //X(XXX)
            PTR(106),
            PTR(62),
        ], 
         // FUN31NanoPrelude.fromJust
        vec![ // 106 
            COM(3,2,[2, 0, 1]), //XXX
            ERR(4),
            COM(1,0,[0]), //X
        ], 
         // FUN32NanoPrelude.head
        vec![ // 107 
            COM(3,2,[2, 0, 1]), //XXX
            ERR(3),
            COM(2,0,[0]), //X
        ], 
         // FUN33NanoPrelude.tail
        vec![ // 108 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[0]), //X
            COM(2,0,[1]), //X
        ], 
         // FUN34NanoPrelude.enumFromTo
        vec![ // 109 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(111),
            PTR(116),
        ], 
        vec![ // 110 
            COM(3,2,[2, 0, 1]), //XXX
            PRM(LE,false),
        ], 
        vec![ // 111 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(112),
            PTR(110),
        ], 
         // FUN35NanoPrelude.takeWhile
        vec![ // 112 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(115),
        ], 
        vec![ // 113 
            COM(5,40,[0, 3, 4, 1, 2, 4]), //X(XXX)(XX)
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            COM(2,0,[0]), //X
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 114 
            COM(4,4,[0, 1, 3, 2]), //XXXX
            PTR(113),
        ], 
        vec![ // 115 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(114),
        ], 
         // FUN36NanoPrelude.enumFrom
        vec![ // 116 
            COM(5,49,[0, 4, 1, 4, 2, 3]), //XX(X(XXX))
            COM(4,2,[3, 0, 1]), //XXX
            PTR(116),
            PRM(ADD,false),
            INT(1),
        ], 
         // FUN37Sumpuz.ifNull
        vec![ // 117 
            COM(4,5,[0, 2, 3, 1]), //X(XX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(3,0,[0]), //X
        ], 
    ]
});