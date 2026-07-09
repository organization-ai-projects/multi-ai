#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub kind: FunctionKind,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum FunctionKind {
    View,
    ViewOnEvent,
    Logic,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        name: String,
        value: Expr,
    },
    Component(Component),
    Call {
        name: String,
        args: Vec<Expr>,
    },
    If {
        condition: Expr,
        body: Vec<Statement>,
    },
    Return(Expr),
}

#[derive(Debug, Clone)]
pub struct Component {
    pub name: String,
    pub properties: Vec<(String, Expr)>,
    pub events: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    String(String),
    Number(f32),
    Identifier(String),
    BinaryOp {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}
