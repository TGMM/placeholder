#[derive(Debug, PartialEq, Eq)]
pub enum OpType {
    ADD,
    SUB,
    MUL,
    DIV,
    EQ,
    EQEQ,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PrimitiveInfo {
    String(String),
    Number(i64),
    Boolean(bool),
    Null,
    Undefined,
    BigInt(i64),
    Symbol(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct SignedPrimitiveInfo(pub OpType, pub PrimitiveInfo);

#[derive(Debug, PartialEq, Eq)]
pub enum ExprResult {
    SignedPrimitive(SignedPrimitiveInfo),
    Primitive(PrimitiveInfo),
    ExprResult(Box<ExprResult>, OpType, Box<ExprResult>),
    Identifier(String),
    Assignment(String, Box<ExprResult>)
}

#[derive(Debug, PartialEq, Eq)]
pub enum QualifierType {
    CONST,
    LET,
    VAR,
}

#[derive(Debug, PartialEq, Eq)]
pub struct VarDeclInfo(
    pub QualifierType,
    pub String,
    pub Option<String>,
    pub ExprResult,
);
