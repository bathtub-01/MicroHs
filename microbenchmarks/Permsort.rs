use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 11
// Apps in this file: 37
// Combinators in this file: 69
#[rustfmt::skip]
 pub static Permsort: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Permsort.main
        vec![ // 0 
            PTR(9),
            PTR(8),
        ], 
        vec![ // 1 
            COM(4,2,[3, 0, 1]), //XXX
            INT(12),
            COM(2,0,[0]), //X
        ], 
        vec![ // 2 
            COM(4,2,[3, 0, 1]), //XXX
            INT(12),
            PTR(1),
        ], 
        vec![ // 3 
            COM(4,2,[3, 0, 1]), //XXX
            INT(6),
            PTR(2),
        ], 
        vec![ // 4 
            COM(4,2,[3, 0, 1]), //XXX
            INT(9),
            PTR(3),
        ], 
        vec![ // 5 
            COM(4,2,[3, 0, 1]), //XXX
            INT(7),
            PTR(4),
        ], 
        vec![ // 6 
            COM(4,2,[3, 0, 1]), //XXX
            INT(6),
            PTR(5),
        ], 
        vec![ // 7 
            COM(4,2,[3, 0, 1]), //XXX
            INT(10),
            PTR(6),
        ], 
        vec![ // 8 
            PTR(10),
            PTR(7),
        ], 
         // FUN1NanoPrelude.head
        vec![ // 9 
            COM(3,2,[2, 0, 1]), //XXX
            ERR(3),
            COM(2,0,[0]), //X
        ], 
         // FUN2Permsort.permSort
        vec![ // 10 
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            PTR(9),
            PTR(11),
            PTR(20),
        ], 
        vec![ // 11 
            PTR(12),
            PTR(15),
        ], 
         // FUN3NanoPrelude.filter
        vec![ // 12 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(14),
        ], 
        vec![ // 13 
            COM(5,38,[0, 2, 4, 3, 1, 4]), //X(XX)X(XX)
            COM(4,45,[0, 1, 3, 2, 1, 3]), //X(XX)(X(XX))
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 14 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(13),
        ], 
         // FUN4Permsort.ord
        vec![ // 15 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[1]), //X
            PTR(18),
        ], 
        vec![ // 16 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            PTR(19),
        ], 
        vec![ // 17 
            COM(6,33,[0, 1, 5, 2, 3, 4]), //X(X(XX)X)X
            COM(3,6,[0, 2, 1, 2]), //XX(XX)
            PTR(16),
            PRM(LE,false),
            PTR(15),
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 18 
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[1]), //X
            PTR(17),
        ], 
         // FUN5Data.Bool.&&
        vec![ // 19 
            COM(2,1,[1, 0]), //XX
            COM(2,0,[0]), //X
        ], 
         // FUN6Permsort.perm
        vec![ // 20 
            COM(3,2,[2, 0, 1]), //XXX
            PTR(22),
            PTR(21),
        ], 
        vec![ // 21 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(23),
            PTR(29),
            PTR(20),
        ], 
        vec![ // 22 
            COM(4,2,[3, 0, 1]), //XXX
            COM(2,0,[0]), //X
            COM(2,0,[0]), //X
        ], 
         // FUN7Data.List_Type.concatMap
        vec![ // 23 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(25),
        ], 
        vec![ // 24 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(26),
        ], 
        vec![ // 25 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(24),
        ], 
         // FUN8Data.List_Type.++
        vec![ // 26 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            Y,
            PTR(28),
        ], 
        vec![ // 27 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 28 
            COM(4,6,[3, 1, 0, 2]), //XX(XX)
            PTR(27),
        ], 
         // FUN9Permsort.place
        vec![ // 29 
            COM(4,7,[0, 1, 3, 2]), //X(XXX)
            Y,
            PTR(33),
            PTR(30),
        ], 
        vec![ // 30 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(34),
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 31 
            COM(6,34,[0, 1, 2, 3, 5, 4]), //X(XX(XX))X
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            COM(5,41,[0, 1, 2, 4, 3, 4]), //X(X(XX))(XX)
            COM(4,2,[3, 0, 1]), //XXX
            COM(4,2,[3, 0, 1]), //XXX
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 32 
            COM(5,12,[0, 1, 2, 4, 3]), //X(XXX)X
            COM(4,2,[3, 0, 1]), //XXX
            COM(4,2,[3, 1, 0]), //XXX
            COM(2,0,[0]), //X
            COM(2,0,[0]), //X
        ], 
        vec![ // 33 
            COM(5,39,[0, 1, 2, 4, 3, 4]), //XX(XX)(XX)
            COM(5,17,[0, 1, 2, 3, 4]), //XX(X(XX))
            COM(3,2,[2, 0, 1]), //XXX
            PTR(32),
            PTR(31),
        ], 
         // FUN10NanoPrelude.map
        vec![ // 34 
            COM(3,3,[0, 1, 2]), //X(XX)
            Y,
            PTR(36),
        ], 
        vec![ // 35 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 36 
            COM(5,16,[4, 0, 1, 2, 3]), //XX(XXX)
            COM(2,0,[0]), //X
            PTR(35),
        ], 
    ]
});