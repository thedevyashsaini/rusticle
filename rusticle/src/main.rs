mod commands;
mod lexer;
mod parser;
mod utils;
mod interpreter;

use crate::commands::execute::Interpret;
use crate::commands::reject::Invalid;
use crate::commands::install::Install;
use clap::{Arg, Command as ClapCommand};
use commands::Command;

fn main() {
    let matches: clap::ArgMatches = ClapCommand::new("Rusticle")
        .about("Custom Lin language interpreter")
        .arg(
            Arg::new("arg")
                .required(true)
                .help("The .lin file to interpret"),
        )
        .arg(
            Arg::new("tokens")
                .long("tokens")
                .help("Run and show lexical analysis (tokens)")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ast")
                .long("ast")
                .help("Run and show AST")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ast-raw")
                .long("ast-raw")
                .help("Show raw AST statements")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("install")
                .value_name("PACKAGE")
                .help("Install a specific package.")
        )
        .get_matches();

        let arg: &String = matches.get_one::<String>("arg").unwrap();

        let command: Box<dyn Command> = if arg.ends_with(".lin") {
            Box::new(Interpret {
                filename: arg.clone(),
                tokens: match matches.get_flag("tokens") {
                    true => {Some(true)},
                    false => None,
                },
                ast: match matches.get_flag("ast") {
                    true => {Some(true)},
                    false => None,
                },
                ast_raw: match matches.get_flag("ast-raw") {
                    true => {Some(true)},
                    false => None,
                },
            })
        } else if arg == "install"{
            if matches.get_one::<String>("install").is_some() {
                Box::new(Install {
                    package: matches.get_one::<String>("install").unwrap().clone(),
                    temp: false,
                })
            } else {
                Box::new(Invalid {
                    message: format!(
                        "\x1b[31merror:\x1b[0m the following command requires a package name. \n  \x1b[32m<install>\x1b[0m\n\n\x1b[4mUsage:\x1b[0m \x1b[1mrusticle\x1b[0m install <package>"
                    ),
                })
            }
        } else {
            Box::new(Invalid {
                message: format!("Invalid command {}", arg),
            })
        };

        command.execute();
}