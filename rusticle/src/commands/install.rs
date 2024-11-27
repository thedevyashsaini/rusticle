use crate::commands::Command;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use crate::utils::token::{Token, TokenType};
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;

pub struct Install {
    pub package: String,
    pub temp: bool,
}

#[derive(Deserialize, Serialize)]
struct Package {
    name: String,
    version: String,
    functions: Vec<Function>,
}

#[derive(Deserialize, Serialize)]
struct Function {
    name: String,
    params: Vec<String>,
    body: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct PackagesLock {
    packages: Vec<Package>,
}

impl Command for Install {
    fn execute(&self) {
        println!("> Installing package: {}", self.package);

        let mut packages_lock = String::new();
        let packages_lock_path: &str;

        if !self.temp {
            packages_lock_path = "rusticle.lock";
        } else {
            packages_lock_path = "rusticle.temp.lock";
        }

        // Check if rusticle.lock exists, if not create it with an empty array
        if let Ok(mut file) = fs::File::open(packages_lock_path) {
            file.read_to_string(&mut packages_lock).expect("Unable to read rusticle.lock");
        } else {
            let mut file = fs::File::create(packages_lock_path).expect("Unable to create rusticle.lock");
            file.write_all(b"{\"packages\": []}").expect("Unable to write to rusticle.lock");
        }

        let mut packages_lock_data: PackagesLock = if !packages_lock.is_empty() {
            serde_json::from_str(&packages_lock).expect("Unable to parse rusticle.lock")
        } else {
            PackagesLock { packages: Vec::new() }
        };

        if packages_lock_data.packages.iter().any(|p: &Package| p.name == self.package) {
            println!("> Package '{}' is already installed.", self.package);
            return;
        }

        let client: Client = Client::new();
        let url: String = format!("http://127.0.0.1:8080/packages/{}", self.package);

        match client.get(&url).send() {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Package>() {
                        Ok(package) => {
                            let success: String = format!("> Package '{}' version '{}' installed successfully.", package.name, package.version);
                            let parsed_functions: Vec<Function> = package.functions.into_iter().map(|f| {
                                let params: Vec<String> = f.params.into_iter().map(|p| format!("{:?}", Token {
                                    token_type: TokenType::Identifier,
                                    lexeme: p,
                                    line: 1,
                                })).collect();

                                let body: Vec<String> = f.body.into_iter().map(|stmt| {
                                    let mut lexer = Lexer::new(stmt.clone());
                                    let tokens = lexer.scan_tokens().expect("Failed to scan tokens");
                                    let mut parser = Parser::new(tokens.clone());
                                    format!("{:?}", parser.parse())
                                }).collect();

                                Function {
                                    name: format!("{:?}", Token {
                                        token_type: TokenType::Identifier,
                                        lexeme: f.name,
                                        line: 1,
                                    }),
                                    params,
                                    body,
                                }
                            }).collect();

                            let new_package = Package {
                                name: package.name,
                                version: package.version,
                                functions: parsed_functions,
                            };

                            packages_lock_data.packages.push(new_package);
                            let packages_lock_content: String = serde_json::to_string_pretty(&packages_lock_data).expect("Unable to serialize rusticle.lock");
                            let mut file: fs::File = OpenOptions::new()
                                .write(true)
                                .create(true)
                                .truncate(true)
                                .open(packages_lock_path)
                                .expect("Unable to open rusticle.lock");
                            file.write_all(packages_lock_content.as_bytes()).expect("Unable to write to rusticle.lock");
                            println!("{}", success);
                        }
                        Err(err) => {
                            eprintln!("> Failed to parse package details or package doesn't exist: {}", err);
                        }
                    }
                } else {
                    eprintln!("> Failed to fetch package: {}", response.status());
                }
            }
            Err(err) => {
                eprintln!("> Failed to request package: {}", err);
            }
        }
    }
}