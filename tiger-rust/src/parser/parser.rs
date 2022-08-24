use crate::lexer::lexer::Lexer;
use crate::lexer::lexer::Token;
use lalrpop_util::ParseError;

use super::ast;

lalrpop_mod!(pub parser,"/parser/tiger.rs");

pub fn compile(input: &str) -> Result<Box<ast::Exp>, ParseError<(), Token, ()>> {
    parser::ExpParser::new().parse(Lexer::new(input))
}

#[test]
fn parse_correct() {
    let mut input = "a+b";
    let mut result = compile(input);
    result.unwrap();
    input = "a+b*c";
    result = compile(input);
    assert!(result.is_ok());
    input = "a<b+c";
    result = compile(input);
    assert!(result.is_ok());
    input = "a+(a<B)";
    result = compile(input);
    assert!(result.is_ok());
    input = "-(-1<1|2&a+b-c*d)+1-(-1)";
    result = compile(input);
    assert!(result.is_ok());
    input = "a[b]";
    result = compile(input);
    assert!(result.is_ok());
    input = "if a then if a then d else c+d";
    result = compile(input);
    assert!(result.is_ok());
    input = "f()+c/d";
    result = compile(input);
    assert!(result.is_ok());
    input = "a := 1+a[b]";
    result = compile(input);
    assert!(result.is_ok());
    result.unwrap();
    input = "a := T[a] of b";
    result = compile(input);
    assert!(result.is_ok());
    input = "b := K[b] of T[a] of b";
    result = compile(input);
    assert!(result.is_ok());
    input = "b := 1 + (a;b)";
    result = compile(input);
    assert!(result.is_ok());
    input = "while a do a+b";
    result = compile(input);
    assert!(result.is_ok());
    input = "while a do a := a[b] of a";
    result = compile(input);
    assert!(result.is_ok());
    input = "for a:= b to c do a+c/d+5";
    result = compile(input);
    assert!(result.is_ok());
    input = "for a:= b to d do a[b] of a";
    result = compile(input);
    assert!(result.is_ok());
    input = "if a<b & c>0 then a else b";
    result = compile(input);
    assert!(result.is_ok());
    input = "a.b";
    result = compile(input);
    assert!(result.is_ok());
    input = "()";
    result = compile(input);
    assert!(result.is_ok());
    input = "let var a:=b in end";
    result = compile(input);
    assert!(result.is_ok());
}

#[test]
fn parse_error() {
    let mut input = "--a";
    let mut result = compile(input);
    assert!(result.is_err());
    input = "a<b<c";
    result = compile(input);
    assert!(result.is_err());
    input = "a:=b:=c";
    result = compile(input);
    assert!(result.is_err());
    input = "{} + a";
    result = compile(input);
    assert!(result.is_err());
}
