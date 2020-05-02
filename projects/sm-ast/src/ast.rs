use num::BigInt;

#[derive(Clone, Debug)]
pub enum AST {
    NewLine,

    //
    ///
    //Function(Box<str>, Vec<AST>),

    //
    ///
    Prefix(Box<str>, bool, Box<AST>),
    Suffix(Box<str>, bool, Box<AST>),
    Binary(Box<str>, bool, Box<AST>, Box<AST>),

    //
    ///
    //Boolean(bool),
    Integer(BigInt),
    Decimal(f64),
    Symbol(Box<str>),
    String(Box<str>),
}
