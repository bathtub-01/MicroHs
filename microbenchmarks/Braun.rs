use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 15
// Apps in this file: 36
// Combinators in this file: 68
#[rustfmt::skip]
 pub static Braun: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Braun.main
        vec![ // 0 
            PTR(4),
            PTR(3),
        ], 
        vec![ // 1 
            PTR(28),
            INT(0),
            INT(255),
        ], 
        vec![ // 2 
            PTR(25),
            INT(2),
            PTR(1),
        ], 
        vec![ // 3 
            PTR(5),
            PTR(9),
            PTR(2),
        ], 
         // FUN1Braun.int
        vec![ // 4 
            COM(3,2,[2, 0, 1]), //XXX
            INT(0),
            INT(1),
        ], 
         // FUN2NanoPrelude.all
        vec![ // 5 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(7),
        ], 
        vec![ // 6 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(8),
        ], 
        vec![ // 7 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[1]), //X
            PTR(6),
        ], 
         // FUN3Data.Bool.&&
        vec![ // 8 
            COM(2,1,[1, 0]), //XX
            COM(2,0,[0]), //X
        ], 
         // FUN4Braun.prop
        vec![ // 9 
            COM(4,17,[0, 3, 1, 2, 3]), //XX(X(XX))
            PTR(10),
            PTR(14),
            PTR(20),
        ], 
         // FUN5Braun.equal
        vec![ // 10 
            COM(5,40,[3, 4, 0, 1, 2, 4]), //X(XXX)(XX)
            COM(2,0,[1]), //X
            PTR(13),
            PTR(12),
        ], 
        vec![ // 11 
            COM(5,11,[0, 1, 4, 2, 3]), //XX(XX)X
            COM(5,12,[0, 1, 4, 2, 3]), //X(XXX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            PRM(EQ,false),
            COM(2,0,[0]), //X
        ], 
        vec![ // 12 
            COM(6,25,[0, 1, 5, 2, 3, 4]), //XX(XX)XX
            COM(5,11,[0, 1, 2, 4, 3]), //XX(XX)X
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            COM(2,0,[0]), //X
            PTR(11),
            PTR(10),
        ], 
        vec![ // 13 
            COM(3,0,[0]), //X
            COM(2,0,[0]), //X
        ], 
         // FUN6Braun.toList
        vec![ // 14 
            COM(3,2,[2, 0, 1]), //XXX
            PTR(15),
            COM(2,0,[0]), //X
        ], 
        vec![ // 15 
            COM(6,25,[0, 1, 2, 5, 3, 4]), //XX(XX)XX
            COM(5,31,[0, 1, 2, 3, 4, 3]), //XX(X(XX))X
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            COM(4,2,[3, 0, 1]), //XXX
            PTR(16),
            PTR(14),
        ], 
         // FUN7Braun.ilv
        vec![ // 16 
            COM(3,6,[1, 2, 0, 2]), //XX(XX)
            PTR(19),
        ], 
        vec![ // 17 
            COM(4,11,[0, 1, 2, 3, 2]), //XX(XX)X
            COM(5,11,[0, 1, 2, 4, 3]), //XX(XX)X
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 18 
            COM(5,39,[0, 3, 1, 4, 2, 4]), //XX(XX)(XX)
            COM(5,45,[0, 1, 4, 2, 3, 4]), //X(XX)(X(XX))
            COM(4,2,[3, 0, 1]), //XXX
            PTR(17),
        ], 
        vec![ // 19 
            COM(4,4,[0, 2, 3, 1]), //XXXX
            PTR(18),
            PTR(16),
        ], 
         // FUN8Braun.fromList
        vec![ // 20 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[1]), //X
            PTR(21),
        ], 
        vec![ // 21 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(22),
            PTR(20),
        ], 
         // FUN9Braun.insertTree
        vec![ // 22 
            COM(5,46,[0, 4, 1, 4, 2, 3]), //XX(XXXX)
            PTR(23),
            PTR(24),
            COM(2,0,[1]), //X
            COM(2,0,[1]), //X
        ], 
        vec![ // 23 
            COM(6,53,[0, 1, 2, 3, 5, 4]), //X(XX(XX)X)
            COM(3,2,[2, 0, 1]), //XXX
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(24),
            PTR(22),
        ], 
         // FUN10Braun.Branch
        vec![ // 24 
            COM(5,18,[0, 4, 1, 2, 3]), //X(XXXX)
            COM(2,0,[0]), //X
        ], 
         // FUN11NanoPrelude.replicate
        vec![ // 25 
            COM(3,2,[0, 2, 1]), //XXX
            PTR(27),
            COM(2,0,[0]), //X
        ], 
        vec![ // 26 
            COM(5,12,[0, 4, 1, 2, 3]), //X(XXX)X
            COM(5,34,[0, 1, 4, 2, 4, 3]), //X(XX(XX))X
            PRM(LE,false),
            INT(0),
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 27 
            COM(5,49,[0, 4, 1, 4, 2, 3]), //XX(X(XXX))
            PTR(26),
            PTR(25),
            PRM(SUB,fasle),
            INT(1),
        ], 
         // FUN12NanoPrelude.enumFromTo
        vec![ // 28 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(30),
            PTR(35),
        ], 
        vec![ // 29 
            COM(3,2,[2, 0, 1]), //XXX
            PRM(LE,false),
        ], 
        vec![ // 30 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(31),
            PTR(29),
        ], 
         // FUN13NanoPrelude.takeWhile
        vec![ // 31 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(34),
        ], 
        vec![ // 32 
            COM(5,40,[0, 3, 4, 1, 2, 4]), //X(XXX)(XX)
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            COM(2,0,[0]), //X
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 33 
            COM(4,4,[0, 1, 3, 2]), //XXXX
            PTR(32),
        ], 
        vec![ // 34 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(33),
        ], 
         // FUN14NanoPrelude.enumFrom
        vec![ // 35 
            COM(5,49,[0, 4, 1, 4, 2, 3]), //XX(X(XXX))
            COM(4,2,[3, 0, 1]), //XXX
            PTR(35),
            PRM(ADD,false),
            INT(1),
        ], 
    ]
});