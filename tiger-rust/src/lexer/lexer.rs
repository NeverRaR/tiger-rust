use std::fmt::Display;

use logos::{Lexer, Logos};
use regex::Regex;

fn parse_id(lex: &mut Lexer<Token>) -> String {
    String::from(lex.slice())
}

fn parse_string(lex: &mut Lexer<Token>) -> String {
    let escape_ignore_reg = Regex::new("\\\\[ \\n\\t\\r\\f]+\\\\").unwrap();
    String::from(escape_ignore_reg.replace(lex.slice(), ""))
}

/// Comment: /\*[^*]*\*+([^/*][^*]*\*+)*/
/// String : "((\\("|\\|n|t|(\^[a-zA-Z\^\\\[\]_])|[01][0-7][0-7]|[ \n\t\r\f]+\\))|[^"\n\\])*"
/// Int: [0-9]+
/// Id: [a-zA-Z][a-zA-Z_]*
#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex("/\\*[^*]*\\*+([^/*][^*]*\\*+)*/")]
    Comment,
    #[regex("[a-zA-Z][a-zA-Z_]*", parse_id)]
    Id(String),
    #[regex("\"((\\\\(\"|\\\\|n|t|(\\^[a-zA-Z\\^\\\\\\[\\]_])|[01][0-7][0-7]|[ \\n\\t\\r\\f]+\\\\))|[^\"\\n\\\\])*\"",parse_string)]
    String(String),
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Int(u64),
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token("(")]
    Lparen,
    #[token(")")]
    Rparen,
    #[token("[")]
    Lbrack,
    #[token("]")]
    Rbrack,
    #[token("{")]
    Lbrace,
    #[token("}")]
    Rbrace,
    #[token(".")]
    Dot,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Times,
    #[token("/")]
    Divide,
    #[token("=")]
    Eq,
    #[token("<>")]
    Neq,
    #[token("<")]
    Lt,
    #[token("<=")]
    Le,
    #[token(">")]
    Gt,
    #[token(">=")]
    Ge,
    #[token("&")]
    And,
    #[token("|")]
    Or,
    #[token(":=")]
    Assign,
    #[token("array")]
    Array,
    #[token("if")]
    If,
    #[token("then")]
    Then,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("to")]
    To,
    #[token("do")]
    Do,
    #[token("let")]
    Let,
    #[token("in")]
    In,
    #[token("end")]
    End,
    #[token("of")]
    Of,
    #[token("break")]
    Break,
    #[token("nil")]
    Nil,
    #[token("function")]
    Function,
    #[token("var")]
    Var,
    #[token("type")]
    Type,
    #[error]
    #[regex(r"[ \r\t\n\f]+", logos::skip)]
    Error,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token_str = match &self {
            Token::Comment => String::from("Comment"),
            Token::Id(id) => format!("Id : {}", id),
            Token::String(str) => format!("String : {}", str),
            Token::Int(i) => format!("Int : {}", i),
            Token::Comma => String::from(","),
            Token::Colon => String::from(":"),
            Token::Semicolon => String::from(";"),
            Token::Lparen => String::from("("),
            Token::Rparen => String::from(")"),
            Token::Lbrack => String::from("["),
            Token::Rbrack => String::from("]"),
            Token::Lbrace => String::from("{"),
            Token::Rbrace => String::from("}"),
            Token::Dot => String::from("."),
            Token::Plus => String::from("+"),
            Token::Minus => String::from("-"),
            Token::Times => String::from("*"),
            Token::Divide => String::from("/"),
            Token::Eq => String::from("="),
            Token::Neq => String::from("<>"),
            Token::Lt => String::from("<"),
            Token::Le => String::from("<="),
            Token::Gt => String::from(">"),
            Token::Ge => String::from(">="),
            Token::And => String::from("&"),
            Token::Or => String::from("|"),
            Token::Assign => String::from(":="),
            Token::Array => String::from("array"),
            Token::If => String::from("if"),
            Token::Then => String::from("then"),
            Token::Else => String::from("else"),
            Token::While => String::from("while"),
            Token::For => String::from("for"),
            Token::To => String::from("to"),
            Token::Do => String::from("do"),
            Token::Let => String::from("let"),
            Token::In => String::from("in"),
            Token::End => String::from("end"),
            Token::Of => String::from("of"),
            Token::Break => String::from("break"),
            Token::Nil => String::from("nil"),
            Token::Function => String::from("function"),
            Token::Var => String::from("var"),
            Token::Type => String::from("type"),
            Token::Error => String::from("error"),
        };
        write!(f, "{}", token_str)
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut result = Vec::new();
    let mut lex = Token::lexer(input);
    while let Some(token) = lex.next() {
        match token {
            Token::Comment => (),
            Token::Error => {
                let slice = lex.slice();
                println!("wrong token : {}", slice);
                panic!()
            }
            _ => result.push(token),
        }
    }
    result
}
