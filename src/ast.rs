#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Module(ModuleDecl),
    Function(FunctionDecl),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleDecl {
    pub name: String,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: String,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub type_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VariableDeclaration {
        mutable: bool,
        name: String,
        type_name: Option<String>,
        initializer: Expression,
    },
    Assignment {
        name: String,
        value: Expression,
    },
    If {
        condition: Expression,
        then_branch: Block,
        else_branch: Option<Box<Statement>>,
    },
    While {
        condition: Expression,
        body: Block,
    },
    For {
        variable: String,
        iterable: Expression,
        body: Block,
    },
    Return(Option<Expression>),
    Expression(Expression),
    Block(Block),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Integer(String),
    Float(String),
    String(String),
    Char(String),
    Bool(bool),
    Identifier(String),
    Unary {
        operator: String,
        right: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    Range {
        start: Box<Expression>,
        inclusive: bool,
        end: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Grouping(Box<Expression>),
}
