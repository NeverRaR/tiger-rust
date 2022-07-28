use std::collections::HashMap;

use prog::*;
use slp::*;
pub mod prog;
pub mod slp;

fn exp_max_args(exp: &AExp) -> i32 {
    match exp {
        AExp::Op {
            left,
            oper: _,
            right,
        } => exp_max_args(left) + exp_max_args(right),
        AExp::Eseq { stm, exp } => max_args(stm) + exp_max_args(exp),
        _ => 0,
    }
}

fn list_max_args(list: &AExpList) -> i32 {
    match list {
        AExpList::Pair { head, tail } => exp_max_args(head) + list_max_args(tail),
        AExpList::Last(exp) => exp_max_args(exp),
    }
}

fn max_args(stm: &AStm) -> i32 {
    match stm {
        AStm::Compound { stm1, stm2 } => max_args(stm1) + max_args(stm2),
        AStm::Assign { id: _, exp } => exp_max_args(exp),
        AStm::Print { list } => 1 + list_max_args(list),
    }
}

fn stm_interp(stm: &AStm, map: &mut HashMap<String, i32>) {
    match stm {
        AStm::Compound { stm1, stm2 } => {
            stm_interp(stm1, map);
            stm_interp(stm2, map);
        }
        AStm::Assign { id, exp } => {
            let result = exp_interp(exp, map);
            map.insert(id.clone(), result);
        }
        AStm::Print { list } => {
            let l = list_interp(list, map);
            for (_, v) in l.iter().enumerate() {
                print!("{} ", v);
            }
            print!("\n");
        }
    }
}

fn exp_interp(exp: &AExp, map: &mut HashMap<String, i32>) -> i32 {
    match exp {
        AExp::Id(id) => map.get(id).unwrap().clone(),
        AExp::Num(num) => num.clone(),
        AExp::Op { left, oper, right } => {
            let l = exp_interp(left, map);
            let r = exp_interp(right, map);
            match oper {
                ABinop::Plus => l + r,
                ABinop::Minus => l - r,
                ABinop::Times => l * r,
                ABinop::Div => l / r,
            }
        }
        AExp::Eseq { stm, exp } => {
            stm_interp(stm, map);
            exp_interp(exp, map)
        }
    }
}

fn list_interp(list: &AExpList, map: &mut HashMap<String, i32>) -> Vec<i32> {
    match list {
        AExpList::Pair { head, tail } => {
            let mut l = list_interp(tail, map);
            let v = exp_interp(head, map);
            l.push(v.clone());
            l.reverse();
            l
        }
        AExpList::Last(exp) => vec![exp_interp(exp, map)],
    }
}

fn interp(stm: &AStm) {
    let mut map = HashMap::<String, i32>::new();
    stm_interp(stm, &mut map);
}

fn main() {
    let p = prog();
    assert_eq!(2, max_args(&p));
    interp(&p);
}
