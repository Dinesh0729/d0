use std::env;
use std::fs;
use std::process::ExitCode;

use d0c::{check_semantics, lex, parse};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && (args[1] == "--version" || args[1] == "-V") {
        println!("D0 compiler frontend d0c {}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }

    if args.len() != 3 || (args[1] != "lex" && args[1] != "parse" && args[1] != "check") {
        eprintln!("D0 compiler frontend (d0c)");
        eprintln!("Usage:");
        eprintln!("  d0c --version");
        eprintln!("  d0c lex <source.d0>       Tokenize and print tokens");
        eprintln!("  d0c parse <source.d0>     Parse source code into AST");
        eprintln!("  d0c check <source.d0>     Run semantic analysis and static type checking");
        return ExitCode::from(2);
    }

    let command = args[1].as_str();
    let path = &args[2];
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("d0c: cannot read '{}': {}", path, error);
            return ExitCode::FAILURE;
        }
    };

    let tokens = match lex(&source) {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("{}: {}", path, error);
            return ExitCode::FAILURE;
        }
    };

    match command {
        "lex" => {
            for token in tokens {
                println!(
                    "{:?}({}) @ {}:{}",
                    token.kind,
                    token.lexeme.escape_debug(),
                    token.line,
                    token.column
                );
            }
            ExitCode::SUCCESS
        }
        "parse" => match parse(&tokens) {
            Ok(program) => {
                println!("{:#?}", program);
                ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("{}: {}", path, error);
                ExitCode::FAILURE
            }
        },
        "check" => match parse(&tokens) {
            Ok(program) => match check_semantics(&program) {
                Ok(()) => {
                    println!("{}: semantic analysis passed", path);
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    eprintln!("{}: {} semantic error(s) found:", path, errors.len());
                    for err in errors {
                        eprintln!("  - {}", err);
                    }
                    ExitCode::FAILURE
                }
            },
            Err(error) => {
                eprintln!("{}: syntax error: {}", path, error);
                ExitCode::FAILURE
            }
        },
        _ => ExitCode::from(2),
    }
}
