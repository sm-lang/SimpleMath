use num::BigInt;

#[derive(Clone, Debug)]
pub enum AST {
    NewLine,
    Prefix(Box<str>, Box<AST>),
    Suffix(Box<str>, Box<AST>),
    Binary(Box<str>, Box<AST>, Box<AST>),
    Integer(BigInt),
    Decimal(f64),
    Symbol(Box<str>),
    String(Box<str>),
}
