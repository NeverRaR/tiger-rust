use serde::{Deserialize, Serialize};

use crate::lexer::lexer::ValueInfo;

#[derive(Serialize, Deserialize, Debug)]
pub enum Decs {
    Empty,
    Decs(Box<Dec>, Box<Decs>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Dec {
    TypeDec(Box<TypeDec>),
    VarDec(Box<VarDec>),
    FunDec(Box<FunDec>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TypeDec {
    Dec(ValueInfo<String>, Box<Ty>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Ty {
    Id(ValueInfo<String>),
    Fields(Box<TyFields>),
    Array(ValueInfo<String>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TyFields {
    Empty,
    Some(Box<SomeTyFields>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SomeTyFields {
    Some {
        id: ValueInfo<String>,
        type_id: ValueInfo<String>,
    },
    List {
        id: ValueInfo<String>,
        type_id: ValueInfo<String>,
        tail: Box<SomeTyFields>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VarDec {
    DefaultInit(ValueInfo<String>, Box<Exp>),
    TypeInit {
        var_id: ValueInfo<String>,
        type_id: ValueInfo<String>,
        exp: Box<Exp>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FunDec {
    Proc(ValueInfo<String>, Box<TyFields>, Box<Exp>),
    Func {
        id: ValueInfo<String>,
        fields: Box<TyFields>,
        type_id: ValueInfo<String>,
        exp: Box<Exp>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ExpSeq {
    Empty,
    Some(Box<SomeExpSeq>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SomeExpSeq {
    Some(Box<Exp>),
    List(Box<Exp>, Box<SomeExpSeq>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Exp {
    Match(Box<Match>),
    Unmatch(Box<Unmatch>),
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Match {
    IfThenElse {
        if_exp: Box<Exp>,
        then_exp: Box<Match>,
        else_exp: Box<Match>,
    },
    While {
        cond: Box<Exp>,
        body: Box<Match>,
    },
    For {
        id: ValueInfo<String>,
        start: Box<Exp>,
        end: Box<Exp>,
        body: Box<Match>,
    },
    Assign(Box<LValue>, Box<Slice>),
    Slice(Box<Slice>),
    Break,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Unmatch {
    IfThen {
        if_exp: Box<Exp>,
        then_exp: Box<Exp>,
    },
    IfThenElse(Box<Exp>, Box<Match>, Box<Unmatch>),
    While {
        cond: Box<Exp>,
        body: Box<Unmatch>,
    },
    For {
        id: ValueInfo<String>,
        start: Box<Exp>,
        end: Box<Exp>,
        body: Box<Unmatch>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LValue {
    Id(ValueInfo<String>),
    Refer(Box<Refer>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Refer {
    Field(ValueInfo<String>, ValueInfo<String>),
    ReferField(Box<Refer>, ValueInfo<String>),
    Array(ValueInfo<String>, Box<Exp>),
    ReferArray(Box<Refer>, Box<Exp>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Slice {
    Array {
        type_id: ValueInfo<String>,
        len: Box<Exp>,
        init: Box<Slice>,
    },
    Record(ValueInfo<String>, Box<RecList>),
    Sheet(Box<Sheet>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Sheet {
    Or(Box<Sheet>, Box<Piece>),
    Piece(Box<Piece>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Piece {
    And(Box<Piece>, Box<Bit>),
    Bit(Box<Bit>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Bit {
    Eq(Box<Item>, Box<Item>),
    Neq(Box<Item>, Box<Item>),
    Lt(Box<Item>, Box<Item>),
    Le(Box<Item>, Box<Item>),
    Gt(Box<Item>, Box<Item>),
    Ge(Box<Item>, Box<Item>),
    Item(Box<Item>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Item {
    Plus(Box<Item>, Box<Term>),
    Minus(Box<Item>, Box<Term>),
    Term(Box<Term>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Term {
    Times(Box<Term>, Box<Factor>),
    Divide(Box<Term>, Box<Factor>),
    Factor(Box<Factor>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Factor {
    MMeta(Box<Meta>),
    Meta(Box<Meta>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Meta {
    Int(ValueInfo<u64>),
    String(ValueInfo<String>),
    Nil,
    Id(ValueInfo<String>),
    Refer(Box<Refer>),
    CapSeq(Box<ExpSeq>),
    Call(ValueInfo<String>, Box<ArgsList>),
    Let(Box<Decs>, Box<ExpSeq>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ArgsList {
    Empty,
    Some(Box<SomeArgsList>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SomeArgsList {
    Some(Box<Exp>),
    List(Box<Exp>, Box<SomeArgsList>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RecList {
    Empty,
    Some(Box<SomeRecList>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SomeRecList {
    Some(ValueInfo<String>, Box<Exp>),
    List(ValueInfo<String>, Box<Exp>, Box<SomeRecList>),
}
