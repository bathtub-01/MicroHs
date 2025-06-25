use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 12
// Apps in this file: 43
// Combinators in this file: 68
#[rustfmt::skip]
 pub static Ordlist: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Ordlist.main
        vec![ // 0 
            PTR(5),
            PTR(4),
            INT(0),
            INT(1),
        ], 
        vec![ // 1 
            PTR(42),
            COM(2,0,[1]), //X
        ], 
        vec![ // 2 
            PTR(42),
            PTR(1),
        ], 
        vec![ // 3 
            PTR(42),
            PTR(2),
        ], 
        vec![ // 4 
            PTR(42),
            PTR(3),
        ], 
         // FUN1Ordlist.top
        vec![ // 5 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(12),
            PTR(11),
        ], 
        vec![ // 6 
            PTR(20),
            COM(2,0,[0]), //X
        ], 
        vec![ // 7 
            PTR(17),
            PTR(6),
        ], 
        vec![ // 8 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(7),
            PTR(33),
        ], 
        vec![ // 9 
            PTR(20),
            COM(2,0,[1]), //X
        ], 
        vec![ // 10 
            PTR(17),
            PTR(9),
        ], 
        vec![ // 11 
            COM(5,41,[0, 1, 2, 4, 3, 4]), //X(X(XX))(XX)
            PTR(14),
            PTR(10),
            PTR(33),
            PTR(8),
        ], 
         // FUN2NanoPrelude.and
        vec![ // 12 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[1]), //X
            PTR(13),
        ], 
        vec![ // 13 
            COM(4,5,[0, 3, 1, 2]), //X(XX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(2,0,[0]), //X
            PTR(12),
        ], 
         // FUN3Data.List_Type.++
        vec![ // 14 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            Y,
            PTR(16),
        ], 
        vec![ // 15 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            COM(3,3,[0, 1, 2]), //X(XX)
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 16 
            COM(4,6,[3, 1, 0, 2]), //XX(XX)
            PTR(15),
        ], 
         // FUN4NanoPrelude.map
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
         // FUN5Ordlist.prop
        vec![ // 20 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(21),
            PTR(28),
        ], 
        vec![ // 21 
            COM(4,45,[0, 1, 3, 1, 2, 3]), //X(XX)(X(XX))
            PTR(22),
            PTR(23),
        ], 
         // FUN6Ordlist.implies
        vec![ // 22 
            COM(2,1,[1, 0]), //XX
            COM(2,0,[1]), //X
        ], 
         // FUN7Ordlist.ord
        vec![ // 23 
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[1]), //X
            PTR(26),
        ], 
        vec![ // 24 
            COM(5,13,[0, 1, 2, 4, 3]), //X(X(XX))X
            COM(4,8,[0, 1, 2, 3]), //X(X(XX))
            PTR(27),
        ], 
        vec![ // 25 
            COM(6,33,[0, 1, 2, 5, 3, 4]), //X(X(XX)X)X
            COM(3,6,[0, 2, 1, 2]), //XX(XX)
            PTR(24),
            PTR(22),
            PTR(23),
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 26 
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[1]), //X
            PTR(25),
        ], 
         // FUN8Data.Bool.&&
        vec![ // 27 
            COM(2,1,[1, 0]), //XX
            COM(2,0,[0]), //X
        ], 
         // FUN9Ordlist.ins
        vec![ // 28 
            COM(4,20,[0, 1, 3, 2, 3]), //X(XX(XX))
            Y,
            PTR(32),
            PTR(31),
        ], 
        vec![ // 29 
            COM(5,11,[0, 1, 2, 4, 3]), //XX(XX)X
            COM(5,29,[0, 1, 4, 2, 4, 3]), //X(XX)(XX)X
            COM(5,41,[0, 1, 2, 4, 3, 4]), //X(X(XX))(XX)
            PTR(22),
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 30 
            COM(5,11,[0, 1, 2, 4, 3]), //XX(XX)X
            COM(6,24,[0, 1, 5, 2, 3, 4]), //X(XX)XXX
            COM(5,48,[0, 4, 1, 2, 3, 4]), //XX(XX(XX))
            PTR(29),
            COM(3,3,[0, 1, 2]), //X(XX)
        ], 
        vec![ // 31 
            COM(4,11,[0, 3, 1, 3, 2]), //XX(XX)X
            PTR(30),
            COM(4,2,[3, 0, 1]), //XXX
            COM(4,2,[3, 0, 1]), //XXX
        ], 
        vec![ // 32 
            COM(5,16,[0, 1, 2, 3, 4]), //XX(XXX)
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,2,[2, 0, 1]), //XXX
            COM(4,2,[3, 1, 0]), //XXX
            COM(2,0,[0]), //X
        ], 
         // FUN10Ordlist.boolList
        vec![ // 33 
            COM(3,2,[2, 0, 1]), //XXX
            PTR(41),
            PTR(34),
        ], 
        vec![ // 34 
            COM(4,2,[3, 0, 1]), //XXX
            COM(2,0,[0]), //X
            COM(2,0,[0]), //X
        ], 
        vec![ // 35 
            COM(4,2,[3, 0, 1]), //XXX
            COM(2,0,[1]), //X
        ], 
        vec![ // 36 
            PTR(17),
            PTR(35),
        ], 
        vec![ // 37 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(36),
            PTR(33),
        ], 
        vec![ // 38 
            COM(4,2,[3, 0, 1]), //XXX
            COM(2,0,[0]), //X
        ], 
        vec![ // 39 
            PTR(17),
            PTR(38),
        ], 
        vec![ // 40 
            COM(5,41,[0, 1, 2, 4, 3, 4]), //X(X(XX))(XX)
            PTR(14),
            PTR(39),
            PTR(33),
            PTR(37),
        ], 
        vec![ // 41 
            COM(4,15,[0, 1, 3, 2, 3]), //X(XX)(XX)
            PTR(14),
            PTR(33),
            PTR(40),
        ], 
         // FUN11Ordlist.S
        vec![ // 42 
            COM(3,3,[0, 2, 1]), //X(XX)
            COM(2,0,[0]), //X
        ], 
    ]
});