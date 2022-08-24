use crate::error::error::{self, PosInfo};
use logos::Logos;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValueInfo<V> {
    pub info: error::PosInfo,
    pub v: V,
}

fn parse_id(lex: &mut logos::Lexer<Token>) -> ValueInfo<String> {
    ValueInfo::<String> {
        info: error::get_position_info(lex.span().start),
        v: String::from(lex.slice()),
    }
}

fn parse_string(lex: &mut logos::Lexer<Token>) -> ValueInfo<String> {
    let escape_ignore_reg = Regex::new("\\\\[ \\n\\t\\r\\f]+\\\\").unwrap();
    ValueInfo::<String> {
        info: error::get_position_info(lex.span().start),
        v: String::from(escape_ignore_reg.replace(lex.slice(), "")),
    }
}

fn parse_int(lex: &mut logos::Lexer<Token>) -> ValueInfo<u64> {
    ValueInfo::<u64> {
        info: error::get_position_info(lex.span().start),
        v: lex.slice().parse().unwrap(),
    }
}

fn parse_pos(lex: &mut logos::Lexer<Token>) -> PosInfo {
    error::get_position_info(lex.span().start)
}
/// Comment: /\*[^*]*\*+([^/*][^*]*\*+)*/
/// String : "((\\("|\\|n|t|(\^[a-zA-Z\^\\\[\]_])|[01][0-7][0-7]|[ \n\t\r\f]+\\))|[^"\n\\])*"
/// Int: [0-9]+
/// Id: [a-zA-Z][a-zA-Z_]*
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex("/\\*[^*]*\\*+([^/*][^*]*\\*+)*/")]
    Comment,
    #[regex("[a-zA-Z][a-zA-Z_0-9]*", parse_id)]
    Id(ValueInfo<String>),
    #[regex("\"((\\\\(\"|\\\\|n|t|(\\^[a-zA-Z\\^\\\\\\[\\]_])|[01][0-7][0-7]|[ \\n\\t\\r\\f]+\\\\))|[^\"\\n\\\\])*\"",parse_string)]
    String(ValueInfo<String>),
    #[regex("[0-9]+", parse_int)]
    Int(ValueInfo<u64>),
    #[token(",", parse_pos)]
    Comma(PosInfo),
    #[token(":", parse_pos)]
    Colon(PosInfo),
    #[token(";", parse_pos)]
    Semicolon(PosInfo),
    #[token("(", parse_pos)]
    Lparen(PosInfo),
    #[token(")", parse_pos)]
    Rparen(PosInfo),
    #[token("[", parse_pos)]
    Lbrack(PosInfo),
    #[token("]", parse_pos)]
    Rbrack(PosInfo),
    #[token("{", parse_pos)]
    Lbrace(PosInfo),
    #[token("}", parse_pos)]
    Rbrace(PosInfo),
    #[token(".", parse_pos)]
    Dot(PosInfo),
    #[token("+", parse_pos)]
    Plus(PosInfo),
    #[token("-", parse_pos)]
    Minus(PosInfo),
    #[token("*", parse_pos)]
    Times(PosInfo),
    #[token("/", parse_pos)]
    Divide(PosInfo),
    #[token("=", parse_pos)]
    Eq(PosInfo),
    #[token("<>", parse_pos)]
    Neq(PosInfo),
    #[token("<", parse_pos)]
    Lt(PosInfo),
    #[token("<=", parse_pos)]
    Le(PosInfo),
    #[token(">", parse_pos)]
    Gt(PosInfo),
    #[token(">=", parse_pos)]
    Ge(PosInfo),
    #[token("&", parse_pos)]
    And(PosInfo),
    #[token("|", parse_pos)]
    Or(PosInfo),
    #[token(":=", parse_pos)]
    Assign(PosInfo),
    #[token("array", parse_pos)]
    Array(PosInfo),
    #[token("if", parse_pos)]
    If(PosInfo),
    #[token("then", parse_pos)]
    Then(PosInfo),
    #[token("else", parse_pos)]
    Else(PosInfo),
    #[token("while", parse_pos)]
    While(PosInfo),
    #[token("for", parse_pos)]
    For(PosInfo),
    #[token("to", parse_pos)]
    To(PosInfo),
    #[token("do", parse_pos)]
    Do(PosInfo),
    #[token("let", parse_pos)]
    Let(PosInfo),
    #[token("in", parse_pos)]
    In(PosInfo),
    #[token("end", parse_pos)]
    End(PosInfo),
    #[token("of", parse_pos)]
    Of(PosInfo),
    #[token("break", parse_pos)]
    Break(PosInfo),
    #[token("nil", parse_pos)]
    Nil(PosInfo),
    #[token("function", parse_pos)]
    Function(PosInfo),
    #[token("var", parse_pos)]
    Var(PosInfo),
    #[token("type", parse_pos)]
    Type(PosInfo),
    #[error]
    #[regex(r"[ \r\t\n\f]+", logos::skip)]
    Error,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token_str = match &self {
            Token::Comment => String::from("Comment"),
            Token::Id(id) => format!("Id : {}", id.v),
            Token::String(str) => format!("String : {}", str.v),
            Token::Int(i) => format!("Int : {}", i.v),
            Token::Comma(_) => String::from(","),
            Token::Colon(_) => String::from(":"),
            Token::Semicolon(_) => String::from(";"),
            Token::Lparen(_) => String::from("("),
            Token::Rparen(_) => String::from(")"),
            Token::Lbrack(_) => String::from("["),
            Token::Rbrack(_) => String::from("]"),
            Token::Lbrace(_) => String::from("{"),
            Token::Rbrace(_) => String::from("}"),
            Token::Dot(_) => String::from("."),
            Token::Plus(_) => String::from("+"),
            Token::Minus(_) => String::from("-"),
            Token::Times(_) => String::from("*"),
            Token::Divide(_) => String::from("/"),
            Token::Eq(_) => String::from("="),
            Token::Neq(_) => String::from("<>"),
            Token::Lt(_) => String::from("<"),
            Token::Le(_) => String::from("<="),
            Token::Gt(_) => String::from(">"),
            Token::Ge(_) => String::from(">="),
            Token::And(_) => String::from("&"),
            Token::Or(_) => String::from("|"),
            Token::Assign(_) => String::from(":="),
            Token::Array(_) => String::from("array"),
            Token::If(_) => String::from("if"),
            Token::Then(_) => String::from("then"),
            Token::Else(_) => String::from("else"),
            Token::While(_) => String::from("while"),
            Token::For(_) => String::from("for"),
            Token::To(_) => String::from("to"),
            Token::Do(_) => String::from("do"),
            Token::Let(_) => String::from("let"),
            Token::In(_) => String::from("in"),
            Token::End(_) => String::from("end"),
            Token::Of(_) => String::from("of"),
            Token::Break(_) => String::from("break"),
            Token::Nil(_) => String::from("nil"),
            Token::Function(_) => String::from("function"),
            Token::Var(_) => String::from("var"),
            Token::Type(_) => String::from("type"),
            Token::Error => String::from("error"),
        };
        write!(f, "{}", token_str)
    }
}

pub struct Lexer<'source> {
    native: logos::Lexer<'source, Token>,
}

impl<'source> Iterator for Lexer<'source>
where
    Token: Logos<'source>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let mut next = self.native.next();
        let mut flag = true;
        while flag {
            match next {
                Some(ref token) => match token {
                    Token::Comment => next = self.native.next(),
                    _ => flag = false,
                },
                None => return None,
            }
        }
        match next {
            Some(token) => match token {
                Token::Error => {
                    let slice = self.slice();
                    let error_pos = self.span().start;
                    error::emit_error(error_pos, format!("wrong token: {}", slice).as_str());
                    Some(token)
                }
                _ => Some(token),
            },
            None => None,
        }
    }
}

impl<'source> Lexer<'source> {
    pub fn new(input: &'source str) -> Self {
        error::reset_line_pos();
        error::parse_line_pos(input);
        Lexer {
            native: Token::lexer(input),
        }
    }

    pub fn span(&self) -> logos::Span {
        self.native.span()
    }
    pub fn slice(&self) -> &'source str {
        self.native.slice()
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut result = Vec::new();
    let mut lex = Lexer::new(input);
    while let Some(token) = lex.next() {
        match token {
            Token::Comment => (),
            Token::Error => {
                let slice = lex.slice();
                let error_pos = lex.span().start;
                error::emit_error(error_pos, format!("wrong token: {}", slice).as_str());
            }
            _ => result.push(token),
        }
    }
    result
}
