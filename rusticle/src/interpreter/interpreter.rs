use crate::commands::Command;
use crate::interpreter::environment::Environment;
use crate::interpreter::visitor::Visitor;
use crate::parser::ast::{Expr, Function, Object, Stmt};
use crate::utils::token::TokenType;
use crate::commands::install::Install;

pub struct Interpreter {
    environment: Environment,
}

#[derive(serde::Deserialize)]
struct PackagesLock {
    packages: Vec<Package>,
}
#[allow(unused)]
#[derive(serde::Deserialize)]
struct Package {
    name: String,
    version: String,
    functions: Vec<Function>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            self.execute(&statement);
        }
    }

    fn execute(&mut self, stmt: &Stmt) {
        stmt.accept(self);
    }

    fn evaluate(&mut self, expr: &Expr) -> Object {
        expr.accept(self)
    }

    pub fn execute_block(&mut self, statements: &[Stmt], environment: Environment) -> Object {
        let previous = self.environment.clone();
        self.environment = environment;

        for statement in statements {
            self.execute(statement);
        }

        self.environment = previous;
        Object::Nil
    }
}

#[allow(unused_variables)]
impl Visitor<Object> for Interpreter {
    fn visit_assign_expr(&mut self, expr: &Expr) -> Object {
        if let Expr::Assign { name, value } = expr {
            let value = self.evaluate(value);
            self.environment.assign(name, value.clone()).unwrap();
            value
        } else {
            panic!("Expected assign expression");
        }
    }

    fn visit_binary_expr(&mut self, expr: &Expr) -> Object {
        if let Expr::Binary {
            left,
            operator,
            right,
        } = expr
        {
            let left = self.evaluate(left);
            let right = self.evaluate(right);

            match operator.token_type {
                TokenType::Plus => Object::Number(left.as_number() + right.as_number()),
                TokenType::Minus => Object::Number(left.as_number() - right.as_number()),
                TokenType::Star => Object::Number(left.as_number() * right.as_number()),
                TokenType::Slash => Object::Number(left.as_number() / right.as_number()),
                TokenType::Greater => Object::Bool(left.as_number() > right.as_number()),
                TokenType::GreaterEqual => Object::Bool(left.as_number() >= right.as_number()),
                TokenType::Less => Object::Bool(left.as_number() < right.as_number()),
                TokenType::LessEqual => Object::Bool(left.as_number() <= right.as_number()),
                TokenType::BangEqual => Object::Bool(left != right),
                TokenType::EqualEqual => Object::Bool(left == right),
                _ => panic!("Unexpected binary operator"),
            }
        } else {
            panic!("Expected binary expression");
        }
    }

    fn visit_call_expr(&mut self, expr: &Expr) -> Object {
        if let Expr::Call {
            callee,
            paren,
            arguments,
        } = expr
        {
            let callee = self.evaluate(callee);

            let mut args = Vec::new();
            for arg in arguments {
                args.push(self.evaluate(arg));
            }

            match callee {
                Object::Function(func) => {
                    if args.len() != func.arity() {
                        panic!(
                            "Expected {} arguments but got {}.",
                            func.arity(),
                            args.len()
                        );
                    }
                    func.call(self, args)
                }
                _ => panic!("Can only call functions and classes."),
            }
        } else {
            panic!("Expected call expression")
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Object {
        if let Expr::Grouping { expression } = expr {
            self.evaluate(expression)
        } else {
            panic!("Expected grouping expression")
        }
    }

    fn visit_literal_expr(&mut self, expr: &Expr) -> Object {
        if let Expr::Literal { value } = expr {
            value.clone()
        } else {
            panic!("Expected literal expression")
        }
    }

    fn visit_logical_expr(&mut self, expr: &Expr) -> Object {
        if let Expr::Logical {
            left,
            operator,
            right,
        } = expr
        {
            let left = self.evaluate(left);

            if operator.token_type == TokenType::Or {
                if left.as_bool() {
                    return left;
                }
            } else {
                if !left.as_bool() {
                    return left;
                }
            }

            self.evaluate(right)
        } else {
            panic!("Expected logical expression")
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> Object {
        if let Expr::Unary { operator, right } = expr {
            let right = self.evaluate(right);

            match operator.token_type {
                TokenType::Minus => Object::Number(-right.as_number()),
                TokenType::Bang => Object::Bool(!right.as_bool()),
                _ => panic!("Unexpected unary operator"),
            }
        } else {
            panic!("Expected unary expression")
        }
    }

    fn visit_variable_expr(&mut self, expr: &Expr) -> Object {
        if let Expr::Variable { name } = expr {
            self.environment.get(name).unwrap()
        } else {
            panic!("Expected variable expression")
        }
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) -> Object {
        if let Stmt::Block { statements } = stmt {
            for statement in statements {
                self.execute(statement);
            }
        }
        Object::Nil
    }

    fn visit_class_stmt(&mut self, stmt: &Stmt) -> Object {
        // Implement class statement logic
        Object::Nil
    }

    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> Object {
        if let Stmt::Expression { expression } = stmt {
            self.evaluate(expression);
        }
        Object::Nil
    }

    fn visit_function_stmt(&mut self, stmt: &Stmt) -> Object {
        if let Stmt::Function { name, params, body } = stmt {
            let function =
                Object::Function(Function::new(name.clone(), params.clone(), body.clone()));
            self.environment.define(name.lexeme.clone(), function);
        }
        Object::Nil
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt) -> Object {
        if let Stmt::If {
            condition,
            then_branch,
            else_branch,
        } = stmt
        {
            if self.evaluate(condition).as_bool() {
                self.execute(then_branch);
            } else if let Some(else_branch) = else_branch {
                self.execute(else_branch);
            }
        }
        Object::Nil
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) -> Object {
        if let Stmt::Print { expression } = stmt {
            let value = self.evaluate(expression);
            println!("{:?}", value);
        }
        Object::Nil
    }

    fn visit_return_stmt(&mut self, stmt: &Stmt) -> Object {
        if let Stmt::Return { keyword, value } = stmt {
            let value = if let Some(value) = value {
                self.evaluate(value)
            } else {
                Object::Nil
            };
            // Handle return value
            // This might involve unwinding the stack or similar mechanism
        }
        Object::Nil
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) -> Object {
        if let Stmt::Var { name, initializer } = stmt {
            let value = if let Some(initializer) = initializer {
                self.evaluate(initializer)
            } else {
                Object::Nil
            };
            self.environment.define(name.lexeme.clone(), value);
        }
        Object::Nil
    }

    fn visit_while_stmt(&mut self, stmt: &Stmt) -> Object {
        if let Stmt::While { condition, body } = stmt {
            while self.evaluate(condition).as_bool() {
                self.execute(body);
            }
        }
        Object::Nil
    }

    fn visit_import_stmt(&mut self, stmt: &Stmt) -> Object {
        if let Stmt::Import {
            function_name,
            package_name,
        } = stmt
        {
            // Read rusticle.lock and load the required functions
            let packages_lock_path: &str = "rusticle.lock";
            let packages_lock: String =
                if let Ok(content) = std::fs::read_to_string(packages_lock_path) {
                    content
                } else {
                    let empty_lock: &str = "{\"packages\": []}";
                    std::fs::write(packages_lock_path, empty_lock)
                        .expect("Unable to create rusticle.lock");
                    empty_lock.to_string()
                };

            let package_name: String = package_name.clone().trim_matches('"').to_string();
            let function_name: String = function_name.clone().trim_matches('"').to_string();

            let packages_lock_data: PackagesLock =
                serde_json::from_str(&packages_lock).expect("Unable to parse rusticle.lock");

            if let Some(package) = packages_lock_data
                .packages
                .iter()
                .find(|p: &&Package| p.name == package_name)
            {
                if let Some(function) = package
                    .functions
                    .iter()
                    .find(|f: &&Function| f.name.lexeme == function_name)
                {
                    self.environment
                        .define(function_name.clone(), Object::Function(function.clone()));
                } else {
                    panic!(
                        "Function '{}' not found in package '{}'",
                        function_name, package_name
                    );
                }
            } else {
                // Try to install the package
                println!(
                    "Package '{}' not found, attempting to install...",
                    package_name
                );
                let install_command = Install {
                    package: package_name.clone(),
                    temp: true,
                };
                install_command.execute();

                // Read rusticle.temp.lock and load the required functions
                let temp_packages_lock_path: &str = "rusticle.temp.lock";
                let temp_packages_lock: String = std::fs::read_to_string(temp_packages_lock_path)
                    .expect("Unable to read rusticle.temp.lock");

                let temp_packages_lock_data: PackagesLock =
                    serde_json::from_str(&temp_packages_lock)
                        .expect("Unable to parse rusticle.temp.lock");

                if let Some(package) = temp_packages_lock_data
                    .packages
                    .iter()
                    .find(|p: &&Package| p.name == package_name)
                {
                    if let Some(function) = package
                        .functions
                        .iter()
                        .find(|f: &&Function| f.name.lexeme == function_name)
                    {
                        self.environment
                            .define(function_name.clone(), Object::Function(function.clone()));
                    } else {
                        panic!(
                            "Function '{}' not found in package '{}'",
                            function_name, package_name
                        );
                    }
                } else {
                    panic!("Package '{}' not found after installation", package_name);
                }
            }
        }
        Object::Nil
    }
}
