use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 24
// Apps in this file: 79
// Combinators in this file: 129
#[rustfmt::skip]
 pub static taut: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Taut.main
        vec![ // 0 
            PTR(1),
            PTR(56),
            INT(0),
            INT(1),
        ], 
         // FUN1Taut.isTaut
        vec![ // 1 
            COM(5,57,[0, 1, 2, 4, 3, 4]), //X(X(XX)(XX))
            PTR(3),
            PTR(5),
            PTR(2),
            PTR(22),
        ], 
        vec![ // 2 
            COM(3,2,[0, 2, 1]), //XXX
            PTR(8),
        ], 
         // FUN2NanoPrelude.and
        vec![ // 3 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[1]), //X
            PTR(4),
        ], 
        vec![ // 4 
            COM(4,5,[0, 3, 1, 2]), //X(XX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(2,0,[0]), //X
            PTR(3),
        ], 
         // FUN3NanoPrelude.map
        vec![ // 5 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(7),
        ], 
        vec![ // 6 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 7 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(6),
        ], 
         // FUN4Taut.eval
        vec![ // 8 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(16),
        ], 
        vec![ // 9 
            COM(3,2,[0, 2, 1]), //XXX
            PTR(17),
        ], 
        vec![ // 10 
            COM(4,4,[2, 3, 0, 1]), //XXXX
            COM(2,0,[1]), //X
            COM(2,0,[0]), //X
        ], 
        vec![ // 11 
            COM(4,12,[0, 2, 3, 1, 2]), //X(XXX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(2,0,[1]), //X
        ], 
        vec![ // 12 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(11),
            COM(1,0,[0]), //X
        ], 
        vec![ // 13 
            COM(4,12,[0, 2, 3, 1, 2]), //X(XXX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(2,0,[0]), //X
        ], 
        vec![ // 14 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(13),
            COM(1,0,[0]), //X
        ], 
        vec![ // 15 
            COM(5,38,[0, 1, 4, 2, 3, 4]), //X(XX)X(XX)
            COM(6,23,[5, 0, 1, 2, 3, 4]), //XXXXXX
            PTR(14),
            COM(1,0,[0]), //X
            PTR(12),
        ], 
        vec![ // 16 
            COM(5,39,[0, 4, 1, 4, 2, 3]), //XX(XX)(XX)
            PTR(15),
            PTR(10),
            PTR(9),
        ], 
         // FUN5Taut.find
        vec![ // 17 
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(18),
            PTR(19),
        ], 
         // FUN6NanoPrelude.fromJust
        vec![ // 18 
            COM(3,2,[2, 0, 1]), //XXX
            ERR(4),
            COM(1,0,[0]), //X
        ], 
         // FUN7NanoPrelude.lookup
        vec![ // 19 
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            Y,
            PTR(21),
            PTR(20),
        ], 
        vec![ // 20 
            COM(6,34,[0, 1, 2, 5, 3, 4]), //X(XX(XX))X
            COM(5,19,[3, 0, 2, 4, 1]), //X(X(XX)X)
            COM(5,12,[0, 1, 4, 2, 3]), //X(XXX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            PRM(EQ,false),
            COM(3,1,[2, 0]), //XX
        ], 
        vec![ // 21 
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[0]), //X
        ], 
         // FUN8Taut.substs
        vec![ // 22 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(24),
            PTR(23),
        ], 
        vec![ // 23 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(44),
            PTR(51),
        ], 
        vec![ // 24 
            COM(5,45,[0, 1, 4, 2, 3, 4]), //X(XX)(X(XX))
            PTR(5),
            PTR(25),
            PTR(28),
            PTR(41),
        ], 
         // FUN9NanoPrelude.zip
        vec![ // 25 
            COM(4,5,[0, 3, 1, 2]), //X(XX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(2,0,[0]), //X
            PTR(27),
        ], 
        vec![ // 26 
            COM(6,30,[0, 1, 2, 3, 5, 4]), //XX(XXX)X
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(4,2,[3, 0, 1]), //XXX
            COM(3,2,[2, 0, 1]), //XXX
        ], 
        vec![ // 27 
            COM(6,25,[0, 1, 5, 2, 3, 4]), //XX(XX)XX
            COM(5,11,[0, 1, 2, 4, 3]), //XX(XX)X
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            COM(2,0,[0]), //X
            PTR(26),
            PTR(25),
        ], 
         // FUN10Taut.bools
        vec![ // 28 
            COM(5,28,[4, 0, 1, 2, 4, 3]), //XXX(XX)X
            PRM(EQ,false),
            INT(0),
            PTR(36),
            PTR(29),
        ], 
        vec![ // 29 
            COM(4,2,[3, 0, 1]), //XXX
            COM(2,0,[0]), //X
            COM(2,0,[0]), //X
        ], 
        vec![ // 30 
            PTR(40),
            COM(2,0,[1]), //X
        ], 
        vec![ // 31 
            PTR(5),
            PTR(30),
        ], 
        vec![ // 32 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(31),
            PTR(28),
        ], 
        vec![ // 33 
            PTR(40),
            COM(2,0,[0]), //X
        ], 
        vec![ // 34 
            PTR(5),
            PTR(33),
        ], 
        vec![ // 35 
            COM(5,41,[0, 1, 2, 4, 3, 4]), //X(X(XX))(XX)
            PTR(37),
            PTR(34),
            PTR(28),
            PTR(32),
        ], 
        vec![ // 36 
            COM(4,7,[0, 3, 1, 2]), //X(XXX)
            PTR(35),
            PRM(SUB,fasle),
            INT(1),
        ], 
         // FUN11Data.List_Type.++
        vec![ // 37 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            Y,
            PTR(39),
        ], 
        vec![ // 38 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 39 
            COM(4,6,[3, 1, 0, 2]), //XX(XX)
            PTR(38),
        ], 
         // FUN12Data.List_Type.:
        vec![ // 40 
            COM(4,2,[3, 0, 1]), //XXX
        ], 
         // FUN13NanoPrelude.length
        vec![ // 41 
            Y,
            PTR(43),
            INT(0),
        ], 
        vec![ // 42 
            COM(6,49,[0, 1, 4, 5, 2, 3]), //XX(X(XXX))
            COM(4,6,[3, 2, 0, 1]), //XX(XX)
            COM(2,0,[0]), //X
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 43 
            COM(3,4,[0, 1, 2, 2]), //XXXX
            PTR(42),
        ], 
         // FUN14Taut.rmdups
        vec![ // 44 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[0]), //X
            PTR(46),
        ], 
        vec![ // 45 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(47),
            PTR(50),
        ], 
        vec![ // 46 
            COM(5,38,[0, 1, 4, 2, 3, 4]), //X(XX)X(XX)
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            COM(4,2,[3, 0, 1]), //XXX
            PTR(44),
            PTR(45),
        ], 
         // FUN15NanoPrelude.filter
        vec![ // 47 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(49),
        ], 
        vec![ // 48 
            COM(5,38,[0, 2, 4, 3, 1, 4]), //X(XX)X(XX)
            COM(4,45,[0, 1, 3, 2, 1, 3]), //X(XX)(X(XX))
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 49 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(48),
        ], 
         // FUN16Taut.neq
        vec![ // 50 
            COM(2,1,[1, 0]), //XX
            PRM(EQ,true),
        ], 
         // FUN17Taut.vars
        vec![ // 51 
            COM(6,23,[5, 0, 1, 2, 3, 4]), //XXXXXX
            PTR(55),
            PTR(54),
            PTR(53),
            PTR(51),
            PTR(52),
        ], 
        vec![ // 52 
            COM(3,2,[0, 2, 1]), //XXX
            COM(4,2,[3, 0, 1]), //XXX
            COM(2,0,[0]), //X
        ], 
        vec![ // 53 
            COM(4,13,[0, 1, 2, 3, 2]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(37),
            PTR(51),
        ], 
        vec![ // 54 
            COM(2,0,[0]), //X
            COM(2,0,[0]), //X
        ], 
        vec![ // 55 
            COM(4,13,[0, 1, 2, 3, 2]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(37),
            PTR(51),
        ], 
         // FUN18Taut.testProp
        vec![ // 56 
            PTR(63),
            PTR(62),
            PTR(60),
        ], 
        vec![ // 57 
            PTR(5),
            COM(6,1,[5, 0]), //XX
            PTR(73),
        ], 
        vec![ // 58 
            PTR(64),
            PTR(68),
            PTR(57),
        ], 
        vec![ // 59 
            COM(6,1,[5, 0]), //XX
            INT(42),
        ], 
        vec![ // 60 
            PTR(63),
            PTR(59),
            PTR(58),
        ], 
        vec![ // 61 
            PTR(5),
            PTR(70),
            PTR(73),
        ], 
        vec![ // 62 
            PTR(64),
            PTR(68),
            PTR(61),
        ], 
         // FUN19Taut.Implies
        vec![ // 63 
            COM(7,16,[0, 1, 6, 2, 3]), //XX(XXX)
            COM(3,1,[0, 1]), //XX
            COM(2,0,[0]), //X
        ], 
         // FUN20NanoPrelude.foldr1
        vec![ // 64 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(67),
        ], 
        vec![ // 65 
            COM(5,17,[0, 1, 2, 3, 4]), //XX(X(XX))
            COM(3,1,[0, 1]), //XX
            COM(2,0,[0]), //X
        ], 
        vec![ // 66 
            COM(5,33,[0, 1, 2, 4, 3, 4]), //X(X(XX)X)X
            COM(3,6,[2, 1, 0, 2]), //XX(XX)
            PTR(65),
        ], 
        vec![ // 67 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            ERR(0),
            PTR(66),
        ], 
         // FUN21Taut.And
        vec![ // 68 
            COM(6,46,[0, 1, 2, 3, 4, 5]), //XX(XXXX)
            COM(3,1,[0, 1]), //XX
            COM(2,0,[0]), //X
            PTR(69),
        ], 
        vec![ // 69 
            COM(5,16,[0, 1, 4, 2, 3]), //XX(XXX)
            COM(3,1,[0, 1]), //XX
            COM(2,0,[0]), //X
        ], 
         // FUN22Taut.imp
        vec![ // 70 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(72),
            COM(6,1,[5, 0]), //XX
        ], 
        vec![ // 71 
            COM(6,1,[5, 0]), //XX
            INT(42),
        ], 
        vec![ // 72 
            PTR(63),
            PTR(71),
        ], 
         // FUN23Taut.names
        vec![ // 73 
            COM(4,2,[3, 0, 1]), //XXX
            INT(0),
            PTR(78),
        ], 
        vec![ // 74 
            COM(4,2,[3, 0, 1]), //XXX
            INT(5),
            COM(2,0,[0]), //X
        ], 
        vec![ // 75 
            COM(4,2,[3, 0, 1]), //XXX
            INT(4),
            PTR(74),
        ], 
        vec![ // 76 
            COM(4,2,[3, 0, 1]), //XXX
            INT(3),
            PTR(75),
        ], 
        vec![ // 77 
            COM(4,2,[3, 0, 1]), //XXX
            INT(2),
            PTR(76),
        ], 
        vec![ // 78 
            COM(4,2,[3, 0, 1]), //XXX
            INT(1),
            PTR(77),
        ], 
    ]
});