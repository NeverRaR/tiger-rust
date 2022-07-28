use crate::slp::*;
use std::boxed::Box;

pub fn prog() -> AStm {
    AStm::Compound {
        stm1: Box::new(AStm::Assign {
            id: String::from("a"),
            exp: Box::new(AExp::Op {
                left: Box::new(AExp::Num(5)),
                oper: ABinop::Plus,
                right: Box::new(AExp::Num(3)),
            }),
        }),
        stm2: Box::new(AStm::Compound {
            stm1: Box::new(AStm::Assign {
                id: String::from("b"),
                exp: Box::new(AExp::Eseq {
                    stm: Box::new(AStm::Print {
                        list: AExpList::Pair {
                            head: Box::new(AExp::Id(String::from("a"))),
                            tail: Box::new(AExpList::Last(Box::new(AExp::Op {
                                left: Box::new(AExp::Id(String::from("a"))),
                                oper: ABinop::Minus,
                                right: Box::new(AExp::Num(1)),
                            }))),
                        },
                    }),
                    exp: Box::new(AExp::Op {
                        left: Box::new(AExp::Num(10)),
                        oper: ABinop::Times,
                        right: Box::new(AExp::Id(String::from("a"))),
                    }),
                }),
            }),
            stm2: Box::new(AStm::Print {
                list: AExpList::Last(Box::new(AExp::Id(String::from("b")))),
            }),
        }),
    }
}

/*
A_stm prog(void) {

return
A_CompoundStm(A_AssignStm("a",
                A_OpExp(A_NumExp(5), A_plus, A_NumExp(3))),
 A_CompoundStm(A_AssignStm("b",
     A_EseqExp(A_PrintStm(A_PairExpList(A_IdExp("a"),
                A_LastExpList(A_OpExp(A_IdExp("a"), A_minus,
                                      A_NumExp(1))))),
             A_OpExp(A_NumExp(10), A_times, A_IdExp("a")))),
  A_PrintStm(A_LastExpList(A_IdExp("b")))));
}

*/
