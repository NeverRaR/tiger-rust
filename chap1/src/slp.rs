pub enum AStm {
    Compound { stm1: Box<AStm>, stm2: Box<AStm> },
    Assign { id: String, exp: Box<AExp> },
    Print { list: AExpList },
}

pub enum AExp {
    Id(String),
    Num(i32),
    Op {
        left: Box<AExp>,
        oper: ABinop,
        right: Box<AExp>,
    },
    Eseq {
        stm: Box<AStm>,
        exp: Box<AExp>,
    },
}

pub enum AExpList {
    Pair {
        head: Box<AExp>,
        tail: Box<AExpList>,
    },
    Last(Box<AExp>),
}

pub enum ABinop {
    Plus,
    Minus,
    Times,
    Div,
}
