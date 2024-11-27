use crate::interpreter::environment::Environment;
use crate::interpreter::interpreter::Interpreter;
use crate::utils::token::Token;
use crate::interpreter::visitor::Visitor;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub enum Object {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    Function(Function),
}

impl Object {
    pub fn as_number(&self) -> f64 {
        if let Object::Number(n) = self {
            *n
        } else {
            panic!("Expected a number");
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Object::Bool(b) => *b,
            Object::Nil => false,
            _ => true,
        }
    }

    // pub fn as_string(&self) -> String {
    //     if let Object::String(s) = self {
    //         s.clone()
    //     } else {
    //         panic!("Expected a string");
    //     }
    // }
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Function {
    pub name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
}

impl Function {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        Function { name, params, body }
    }

    pub fn arity(&self) -> usize {
        self.params.len()
    }

    pub fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Object {
        let mut environment = Environment::new();
        for (param, arg) in self.params.iter().zip(arguments.iter()) {
            environment.define(param.lexeme.clone(), arg.clone());
        }
        interpreter.execute_block(&self.body, environment)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub enum Expr {
    Assign { name: Token, value: Box<Expr> },
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Call { callee: Box<Expr>, paren: Token, arguments: Vec<Expr> },
    Grouping { expression: Box<Expr> },
    Literal { value: Object },
    Logical { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Unary { operator: Token, right: Box<Expr> },
    Variable { name: Token },
}

impl Expr {
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        match self {
            Expr::Assign { .. } => visitor.visit_assign_expr(self),
            Expr::Binary { .. } => visitor.visit_binary_expr(self),
            Expr::Call { .. } => visitor.visit_call_expr(self),
            Expr::Grouping { .. } => visitor.visit_grouping_expr(self),
            Expr::Literal { .. } => visitor.visit_literal_expr(self),
            Expr::Logical { .. } => visitor.visit_logical_expr(self),
            Expr::Unary { .. } => visitor.visit_unary_expr(self),
            Expr::Variable { .. } => visitor.visit_variable_expr(self),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub enum Stmt {
    Block { statements: Vec<Stmt> },
    Class { name: Token, superclass: Option<Expr>, methods: Vec<Stmt> },
    Expression { expression: Expr },
    Function { name: Token, params: Vec<Token>, body: Vec<Stmt> },
    If { condition: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    Print { expression: Expr },
    Return { keyword: Token, value: Option<Expr> },
    Var { name: Token, initializer: Option<Expr> },
    While { condition: Expr, body: Box<Stmt> },
    Import { function_name: String, package_name: String }, // Add Import variant
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        match self {
            Stmt::Block { .. } => visitor.visit_block_stmt(self),
            Stmt::Class { .. } => visitor.visit_class_stmt(self),
            Stmt::Expression { .. } => visitor.visit_expression_stmt(self),
            Stmt::Function { .. } => visitor.visit_function_stmt(self),
            Stmt::If { .. } => visitor.visit_if_stmt(self),
            Stmt::Print { .. } => visitor.visit_print_stmt(self),
            Stmt::Return { .. } => visitor.visit_return_stmt(self),
            Stmt::Var { .. } => visitor.visit_var_stmt(self),
            Stmt::While { .. } => visitor.visit_while_stmt(self),
            Stmt::Import { .. } => visitor.visit_import_stmt(self), // Add Import variant
        }
    }
}