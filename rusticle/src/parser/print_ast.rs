use crate::parser::ast::{Expr, Stmt};
use crate::interpreter::visitor::Visitor;

pub struct AstPrinter;


impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter
    }

    pub fn print(&mut self, statements: &[Stmt]) {
        for stmt in statements {
            println!("{}", stmt.accept(self));
        }
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_assign_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Assign { name, value } = expr {
            format!("(assign {} {})", name.lexeme, value.accept(self))
        } else {
            unreachable!()
        }
    }
    
    fn visit_import_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::Import { function_name, package_name } = stmt {
            format!("(import \"{}\" from \"{}\")", function_name, package_name)
        } else {
            unreachable!()
        }
    }

    fn visit_binary_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Binary { left, operator, right } = expr {
            format!("({} {} {})", operator.lexeme, left.accept(self), right.accept(self))
        } else {
            unreachable!()
        }
    }

    fn visit_call_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Call { callee, arguments, .. } = expr {
            let args: Vec<String> = arguments.iter().map(|arg| arg.accept(self)).collect();
            format!("(call {} {})", callee.accept(self), args.join(" "))
        } else {
            unreachable!()
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Grouping { expression } = expr {
            format!("(group {})", expression.accept(self))
        } else {
            unreachable!()
        }
    }

    fn visit_literal_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Literal { value } = expr {
            format!("{:?}", value)
        } else {
            unreachable!()
        }
    }

    fn visit_logical_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Logical { left, operator, right } = expr {
            format!("({} {} {})", operator.lexeme, left.accept(self), right.accept(self))
        } else {
            unreachable!()
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Unary { operator, right } = expr {
            format!("({} {})", operator.lexeme, right.accept(self))
        } else {
            unreachable!()
        }
    }

    fn visit_variable_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Variable { name } = expr {
            format!("{}", name.lexeme)
        } else {
            unreachable!()
        }
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::Block { statements } = stmt {
            let stmts: Vec<String> = statements.iter().map(|stmt| stmt.accept(self)).collect();
            format!("(block {})", stmts.join(" "))
        } else {
            unreachable!()
        }
    }

    fn visit_class_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::Class { name, superclass, methods } = stmt {
            let super_str = if let Some(superclass) = superclass {
                format!(" < {}", superclass.accept(self))
            } else {
                String::new()
            };
            let methods_str: Vec<String> = methods.iter().map(|method| method.accept(self)).collect();
            format!("(class {}{} {})", name.lexeme, super_str, methods_str.join(" "))
        } else {
            unreachable!()
        }
    }

    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::Expression { expression } = stmt {
            expression.accept(self)
        } else {
            unreachable!()
        }
    }

    fn visit_function_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::Function { name, params, body } = stmt {
            let params_str: Vec<String> = params.iter().map(|param| param.lexeme.clone()).collect();
            let body_str: Vec<String> = body.iter().map(|stmt| stmt.accept(self)).collect();
            format!("(fun {} ({}) {})", name.lexeme, params_str.join(" "), body_str.join(" "))
        } else {
            unreachable!()
        }
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::If { condition, then_branch, else_branch } = stmt {
            let else_str = if let Some(else_branch) = else_branch {
                format!(" else {}", else_branch.accept(self))
            } else {
                String::new()
            };
            format!("(if {} {}{})", condition.accept(self), then_branch.accept(self), else_str)
        } else {
            unreachable!()
        }
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::Print { expression } = stmt {
            format!("(print {})", expression.accept(self))
        } else {
            unreachable!()
        }
    }

    fn visit_return_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::Return { keyword, value } = stmt {
            if let Some(value) = value {
                format!("(return {} {})", keyword.lexeme, value.accept(self))
            } else {
                format!("(return {})", keyword.lexeme)
            }
        } else {
            unreachable!()
        }
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::Var { name, initializer } = stmt {
            if let Some(initializer) = initializer {
                format!("(var {} {})", name.lexeme, initializer.accept(self))
            } else {
                format!("(var {})", name.lexeme)
            }
        } else {
            unreachable!()
        }
    }

    fn visit_while_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::While { condition, body } = stmt {
            format!("(while {} {})", condition.accept(self), body.accept(self))
        } else {
            unreachable!()
        }
    }
}