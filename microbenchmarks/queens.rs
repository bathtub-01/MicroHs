use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 11
// Apps in this file: 32
// Combinators in this file: 56
#[rustfmt::skip]
 pub static queens: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Queens.main
        vec![ // 0 
            PTR(1),
            INT(6),
        ], 
         // FUN1Queens.nsoln
        vec![ // 1 
            COM(3,7,[0, 1, 2, 2]), //X(XXX)
            PTR(2),
            PTR(5),
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
         // FUN3Queens.gen
        vec![ // 5 
            COM(6,53,[0, 1, 2, 3, 5, 4]), //X(XX(XX)X)
            Y,
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(8),
            PTR(7),
            PTR(6),
        ], 
        vec![ // 6 
            COM(4,2,[3, 0, 1]), //XXX
            COM(2,0,[0]), //X
            COM(2,0,[0]), //X
        ], 
        vec![ // 7 
            COM(6,27,[0, 1, 2, 5, 3, 4]), //X(X(XX))XX
            COM(5,21,[0, 3, 4, 1, 2]), //X(X(XXX))
            PTR(9),
            PTR(15),
            PRM(SUB,fasle),
            INT(1),
        ], 
        vec![ // 8 
            COM(5,28,[4, 0, 1, 2, 4, 3]), //XXX(XX)X
            PRM(EQ,false),
            INT(0),
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
         // FUN6Queens.gen1
        vec![ // 15 
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(16),
            PTR(29),
        ], 
        vec![ // 16 
            COM(4,5,[0, 1, 3, 2]), //X(XX)X
            PTR(9),
            PTR(17),
        ], 
         // FUN7Queens.gen2
        vec![ // 17 
            COM(4,42,[0, 2, 3, 1, 2, 3]), //XXX(XXX)
            PTR(19),
            PTR(18),
        ], 
        vec![ // 18 
            COM(5,12,[0, 1, 4, 3, 2]), //X(XXX)X
            COM(4,2,[3, 0, 1]), //XXX
            COM(4,2,[3, 0, 1]), //XXX
            COM(2,0,[0]), //X
        ], 
        vec![ // 19 
            COM(5,9,[0, 4, 1, 3, 2]), //XXXXX
            PTR(20),
            INT(1),
            COM(2,0,[0]), //X
        ], 
         // FUN8Queens.safe
        vec![ // 20 
            COM(5,20,[0, 1, 2, 3, 4]), //X(XX(XX))
            Y,
            COM(3,3,[0, 1, 2]), //X(XX)
            PTR(27),
            PTR(26),
        ], 
        vec![ // 21 
            COM(5,11,[0, 1, 4, 2, 3]), //XX(XX)X
            COM(5,21,[0, 1, 4, 2, 3]), //X(X(XXX))
            PTR(28),
            PRM(EQ,true),
            PRM(SUB,fasle),
        ], 
        vec![ // 22 
            COM(5,11,[0, 1, 4, 2, 3]), //XX(XX)X
            COM(5,21,[0, 1, 4, 2, 3]), //X(X(XXX))
            PTR(28),
            PRM(EQ,true),
            PRM(ADD,false),
        ], 
        vec![ // 23 
            COM(5,41,[0, 1, 2, 4, 3, 4]), //X(X(XX))(XX)
            COM(5,22,[0, 1, 2, 3, 4]), //X(X(X(XX)))
            PTR(28),
        ], 
        vec![ // 24 
            COM(5,17,[0, 1, 2, 4, 3]), //XX(X(XX))
            COM(5,41,[0, 1, 2, 4, 3, 4]), //X(X(XX))(XX)
            COM(4,11,[0, 3, 1, 3, 2]), //XX(XX)X
            PTR(23),
            PRM(EQ,true),
        ], 
        vec![ // 25 
            COM(4,39,[0, 3, 1, 3, 2, 3]), //XX(XX)(XX)
            PTR(24),
            PTR(22),
            PTR(21),
        ], 
        vec![ // 26 
            COM(5,10,[0, 1, 4, 2, 3]), //X(XX)XX
            COM(5,49,[0, 4, 3, 4, 1, 2]), //XX(X(XXX))
            PTR(25),
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 27 
            COM(4,6,[0, 1, 2, 3]), //XX(XX)
            COM(3,2,[2, 0, 1]), //XXX
            COM(2,0,[1]), //X
        ], 
         // FUN9Data.Bool.&&
        vec![ // 28 
            COM(2,1,[1, 0]), //XX
            COM(2,0,[0]), //X
        ], 
         // FUN10Queens.toOne
        vec![ // 29 
            COM(5,28,[4, 0, 1, 2, 4, 3]), //XXX(XX)X
            PRM(EQ,false),
            INT(1),
            PTR(31),
            PTR(30),
        ], 
        vec![ // 30 
            COM(4,2,[3, 0, 1]), //XXX
            INT(1),
            COM(2,0,[0]), //X
        ], 
        vec![ // 31 
            COM(5,49,[0, 4, 1, 4, 2, 3]), //XX(X(XXX))
            COM(4,2,[3, 0, 1]), //XXX
            PTR(29),
            PRM(SUB,fasle),
            INT(1),
        ], 
    ]
});