use crate::parser::ast::{Expr, Stmt};

pub trait Visitor<R> {
    fn visit_assign_expr(&mut self, expr: &Expr) -> R;
    fn visit_binary_expr(&mut self, expr: &Expr) -> R;
    fn visit_call_expr(&mut self, expr: &Expr) -> R;
    fn visit_grouping_expr(&mut self, expr: &Expr) -> R;
    fn visit_literal_expr(&mut self, expr: &Expr) -> R;
    fn visit_logical_expr(&mut self, expr: &Expr) -> R;
    fn visit_unary_expr(&mut self, expr: &Expr) -> R;
    fn visit_variable_expr(&mut self, expr: &Expr) -> R;

    fn visit_block_stmt(&mut self, stmt: &Stmt) -> R;
    fn visit_class_stmt(&mut self, stmt: &Stmt) -> R;
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> R;
    fn visit_function_stmt(&mut self, stmt: &Stmt) -> R;
    fn visit_if_stmt(&mut self, stmt: &Stmt) -> R;
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> R;
    fn visit_return_stmt(&mut self, stmt: &Stmt) -> R;
    fn visit_var_stmt(&mut self, stmt: &Stmt) -> R;
    fn visit_while_stmt(&mut self, stmt: &Stmt) -> R;
    fn visit_import_stmt(&mut self, stmt: &Stmt) -> R;
}