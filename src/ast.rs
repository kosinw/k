#[derive(PartialEq, Clone, Debug)]
pub struct Identifier(pub String);

#[derive(PartialEq, Clone, Debug)]
pub enum Stmt {
    Blank,
    Let(Identifier, Expr),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Empty,
    Identifier(Identifier),
}

pub type BlockStmt = Vec<Stmt>;

#[derive(PartialEq, Clone, Debug)]
pub struct Program(pub BlockStmt);

impl Program {
    pub fn new() -> Program {
        Program(Vec::new())
    }   
}

impl Identifier {
    pub fn new(value: String) -> Identifier {
        Identifier(value)
    }
}