use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 19
// Apps in this file: 58
// Combinators in this file: 83
#[rustfmt::skip]
 pub static Queens2: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Queens2.main
        vec![ // 0 
            PTR(1),
            INT(5),
        ], 
         // FUN1Queens2.nqueens
        vec![ // 1 
            COM(5,58,[0, 1, 4, 2, 4, 3]), //X(XX(XXX))
            PTR(2),
            PTR(5),
            PTR(55),
            COM(2,0,[0]), //X
        ], 
         // FUN2NanoPrelude.length
        vec![ // 2 
            Y,
            PTR(4),
            INT(0),
        ], 
        vec![ // 3 
            COM(6,49,[0, 1, 4, 5, 2, 3]), //XX(X(XXX))
            COM(4,6,[3, 2, 0, 1]), //XX(XX)
            COM(2,0,[0]), //X
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 4 
            COM(3,4,[0, 1, 2, 2]), //XXXX
            PTR(3),
        ], 
         // FUN3Queens2.solve
        vec![ // 5 
            COM(4,4,[0, 3, 1, 2]), //XXXX
            PTR(8),
            PTR(45),
            PTR(6),
        ], 
        vec![ // 6 
            COM(4,2,[3, 0, 1]), //XXX
            COM(2,0,[0]), //X
            COM(2,0,[0]), //X
        ], 
        vec![ // 7 
            COM(5,21,[0, 1, 4, 2, 3]), //X(X(XXX))
            PTR(9),
            PTR(15),
            PRM(SUB,fasle),
            INT(1),
        ], 
        vec![ // 8 
            COM(5,40,[0, 4, 1, 2, 3, 4]), //X(XXX)(XX)
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            PRM(EQ,false),
            INT(0),
            PTR(7),
        ], 
         // FUN4Data.List_Type.concatMap
        vec![ // 9 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(11),
        ], 
        vec![ // 10 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(12),
        ], 
        vec![ // 11 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(10),
        ], 
         // FUN5Data.List_Type.++
        vec![ // 12 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            Y,
            PTR(14),
        ], 
        vec![ // 13 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 14 
            COM(4,6,[3, 1, 0, 2]), //XX(XX)
            PTR(13),
        ], 
         // FUN6Queens2.sol
        vec![ // 15 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(16),
            PTR(5),
            PTR(20),
        ], 
        vec![ // 16 
            COM(5,45,[0, 1, 4, 2, 3, 4]), //X(XX)(X(XX))
            PTR(17),
            COM(4,2,[3, 0, 1]), //XXX
        ], 
         // FUN7NanoPrelude.map
        vec![ // 17 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(19),
        ], 
        vec![ // 18 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 19 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(18),
        ], 
         // FUN8Queens2.next
        vec![ // 20 
            COM(3,6,[0, 2, 1, 2]), //XX(XX)
            PTR(21),
            PTR(40),
        ], 
        vec![ // 21 
            COM(5,57,[0, 1, 2, 4, 3, 4]), //X(X(XX)(XX))
            PTR(22),
            PTR(22),
            PTR(27),
            PTR(35),
        ], 
         // FUN9Queens2.merge
        vec![ // 22 
            COM(4,5,[0, 3, 1, 2]), //X(XX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(2,0,[0]), //X
            PTR(26),
        ], 
        vec![ // 23 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 24 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(23),
            PTR(12),
        ], 
        vec![ // 25 
            COM(5,39,[0, 3, 1, 4, 2, 4]), //XX(XX)(XX)
            COM(5,45,[0, 1, 4, 2, 3, 4]), //X(XX)(X(XX))
            COM(4,2,[3, 0, 1]), //XXX
            PTR(24),
        ], 
        vec![ // 26 
            COM(4,4,[0, 2, 3, 1]), //XXXX
            PTR(25),
            PTR(22),
        ], 
         // FUN10Queens2.down
        vec![ // 27 
            PTR(17),
            PTR(29),
        ], 
        vec![ // 28 
            PTR(34),
            INT(2),
        ], 
        vec![ // 29 
            PTR(30),
            PTR(28),
        ], 
         // FUN11Queens2.one
        vec![ // 30 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(33),
        ], 
        vec![ // 31 
            COM(4,2,[3, 1, 0]), //XXX
            COM(2,0,[0]), //X
        ], 
        vec![ // 32 
            COM(5,38,[0, 2, 4, 3, 1, 4]), //X(XX)X(XX)
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(31),
        ], 
        vec![ // 33 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(32),
        ], 
         // FUN12Queens2.eq
        vec![ // 34 
            COM(2,1,[1, 0]), //XX
            PRM(EQ,false),
        ], 
         // FUN13Queens2.left
        vec![ // 35 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(38),
            PTR(39),
        ], 
        vec![ // 36 
            PTR(34),
            INT(0),
        ], 
        vec![ // 37 
            PTR(30),
            PTR(36),
        ], 
        vec![ // 38 
            PTR(17),
            PTR(37),
        ], 
         // FUN14NanoPrelude.tail
        vec![ // 39 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[0]), //X
            COM(2,0,[1]), //X
        ], 
         // FUN15Queens2.right
        vec![ // 40 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(44),
            PTR(43),
        ], 
        vec![ // 41 
            PTR(34),
            INT(1),
        ], 
        vec![ // 42 
            PTR(30),
            PTR(41),
        ], 
        vec![ // 43 
            PTR(17),
            PTR(42),
        ], 
        vec![ // 44 
            COM(4,2,[3, 0, 1]), //XXX
            COM(2,0,[0]), //X
        ], 
         // FUN16Queens2.fill
        vec![ // 45 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[0]), //X
            PTR(48),
        ], 
        vec![ // 46 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(17),
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 47 
            COM(5,39,[0, 1, 2, 4, 3, 4]), //XX(XX)(XX)
            COM(5,45,[0, 1, 4, 2, 3, 4]), //X(XX)(X(XX))
            PTR(12),
            PTR(49),
            PTR(46),
        ], 
        vec![ // 48 
            COM(3,2,[0, 2, 1]), //XXX
            PTR(47),
            PTR(45),
        ], 
         // FUN17Queens2.lrd
        vec![ // 49 
            COM(6,33,[4, 0, 1, 5, 2, 3]), //X(X(XX)X)X
            COM(4,2,[3, 0, 1]), //XXX
            PTR(54),
            COM(2,0,[0]), //X
            PTR(50),
        ], 
        vec![ // 50 
            COM(3,0,[0]), //X
            COM(2,0,[0]), //X
        ], 
        vec![ // 51 
            COM(4,2,[3, 0, 1]), //XXX
            INT(2),
            COM(2,0,[0]), //X
        ], 
        vec![ // 52 
            COM(4,2,[3, 0, 1]), //XXX
            INT(1),
            PTR(51),
        ], 
        vec![ // 53 
            COM(4,2,[3, 0, 1]), //XXX
            INT(0),
            PTR(52),
        ], 
        vec![ // 54 
            COM(4,2,[3, 0, 1]), //XXX
            PTR(53),
        ], 
         // FUN18NanoPrelude.replicate
        vec![ // 55 
            COM(3,2,[0, 2, 1]), //XXX
            PTR(57),
            COM(2,0,[0]), //X
        ], 
        vec![ // 56 
            COM(5,12,[0, 4, 1, 2, 3]), //X(XXX)X
            COM(5,34,[0, 1, 4, 2, 4, 3]), //X(XX(XX))X
            PRM(LE,false),
            INT(0),
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 57 
            COM(5,49,[0, 4, 1, 4, 2, 3]), //XX(X(XXX))
            PTR(56),
            PTR(55),
            PRM(SUB,fasle),
            INT(1),
        ], 
    ]
});