pub enum AST {
    NewLine,
    Prefix(Box<str>, Box<AST>),
    Suffix(Box<str>, Box<AST>),
    Binary(Box<str>, Box<AST>, Box<AST>),
    Integer(i128),
    Decimal(f64),
    Symbol(Box<str>),
    String(Box<str>),
}