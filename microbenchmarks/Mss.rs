use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 15
// Apps in this file: 39
// Combinators in this file: 59
#[rustfmt::skip]
 pub static Mss: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Mss.main
        vec![ // 0 
            PTR(3),
            PTR(2),
        ], 
        vec![ // 1 
            INT(0),
            PRM(SUB,fasle),
            INT(20),
        ], 
        vec![ // 2 
            PTR(31),
            PTR(1),
            INT(20),
        ], 
         // FUN1Mss.mss
        vec![ // 3 
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            PTR(5),
            PTR(4),
            PTR(15),
        ], 
        vec![ // 4 
            PTR(11),
            PTR(14),
        ], 
         // FUN2NanoPrelude.maximum
        vec![ // 5 
            COM(3,2,[2, 0, 1]), //XXX
            INT(0),
            PTR(7),
        ], 
        vec![ // 6 
            COM(3,5,[0, 2, 1, 2]), //X(XX)X
            COM(3,4,[0, 2, 2, 1]), //XXXX
            PRM(LE,true),
        ], 
        vec![ // 7 
            PTR(8),
            PTR(6),
        ], 
         // FUN3NanoPrelude.foldr
        vec![ // 8 
            COM(4,7,[0, 1, 2, 3]), //X(XXX)
            Y,
            PTR(10),
        ], 
        vec![ // 9 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(3,3,[0, 1, 2]), //X(XX)
        ], 
        vec![ // 10 
            COM(5,16,[4, 2, 0, 1, 3]), //XX(XXX)
            PTR(9),
        ], 
         // FUN4NanoPrelude.map
        vec![ // 11 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(13),
        ], 
        vec![ // 12 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 13 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(12),
        ], 
         // FUN5NanoPrelude.sum
        vec![ // 14 
            PTR(8),
            PRM(ADD,false),
            INT(0),
        ], 
         // FUN6Mss.segments
        vec![ // 15 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(16),
            PTR(25),
        ], 
        vec![ // 16 
            PTR(17),
            PTR(23),
        ], 
         // FUN7Data.List_Type.concatMap
        vec![ // 17 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(19),
        ], 
        vec![ // 18 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(20),
        ], 
        vec![ // 19 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(18),
        ], 
         // FUN8Data.List_Type.++
        vec![ // 20 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            Y,
            PTR(22),
        ], 
        vec![ // 21 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 22 
            COM(4,6,[3, 1, 0, 2]), //XX(XX)
            PTR(21),
        ], 
         // FUN9Mss.tails
        vec![ // 23 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[0]), //X
            PTR(24),
        ], 
        vec![ // 24 
            COM(4,11,[0, 1, 1, 3, 2]), //XX(XX)X
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            COM(4,2,[3, 0, 1]), //XXX
            PTR(23),
        ], 
         // FUN10Mss.inits
        vec![ // 25 
            COM(5,48,[4, 0, 1, 2, 3, 4]), //XX(XX(XX))
            PTR(27),
            COM(3,1,[0, 1]), //XX
            COM(2,0,[0]), //X
            PTR(26),
        ], 
        vec![ // 26 
            COM(4,17,[0, 3, 1, 2, 3]), //XX(X(XX))
            COM(4,2,[3, 0, 1]), //XXX
            PTR(25),
            PTR(28),
        ], 
        vec![ // 27 
            COM(4,2,[3, 0, 1]), //XXX
            COM(2,0,[0]), //X
            COM(2,0,[0]), //X
        ], 
         // FUN11NanoPrelude.init
        vec![ // 28 
            COM(3,2,[2, 0, 1]), //XXX
            ERR(1),
            PTR(30),
        ], 
        vec![ // 29 
            COM(5,17,[0, 1, 2, 3, 4]), //XX(X(XX))
            COM(3,1,[0, 1]), //XX
            COM(2,0,[0]), //X
        ], 
        vec![ // 30 
            COM(6,47,[0, 1, 2, 3, 5, 4]), //XX(X(XX)X)
            COM(3,6,[2, 0, 1, 2]), //XX(XX)
            COM(2,0,[0]), //X
            PTR(29),
            COM(4,2,[3, 0, 1]), //XXX
            PTR(28),
        ], 
         // FUN12NanoPrelude.enumFromTo
        vec![ // 31 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(33),
            PTR(38),
        ], 
        vec![ // 32 
            COM(3,2,[2, 0, 1]), //XXX
            PRM(LE,false),
        ], 
        vec![ // 33 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(34),
            PTR(32),
        ], 
         // FUN13NanoPrelude.takeWhile
        vec![ // 34 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(37),
        ], 
        vec![ // 35 
            COM(5,40,[0, 3, 4, 1, 2, 4]), //X(XXX)(XX)
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            COM(2,0,[0]), //X
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 36 
            COM(4,4,[0, 1, 3, 2]), //XXXX
            PTR(35),
        ], 
        vec![ // 37 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(36),
        ], 
         // FUN14NanoPrelude.enumFrom
        vec![ // 38 
            COM(5,49,[0, 4, 1, 4, 2, 3]), //XX(X(XXX))
            COM(4,2,[3, 0, 1]), //XXX
            PTR(38),
            PRM(ADD,false),
            INT(1),
        ], 
    ]
});