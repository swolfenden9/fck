use clap::{Parser, Subcommand};
use fck::{lexer::lex, parser::parse, run, Error as BFError, Result as BFResult};
use std::fs;

#[derive(Parser)]
#[command(name = "fck")]
#[command(about = "A Brainfuck CLI interpreter and toolkit", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// run a Brainfuck program from a file or a string
    Run {
        /// source file containing Brainfuck code
        file: Option<String>,

        /// Brainfuck code as a string
        #[arg(short, long, conflicts_with = "file")]
        source: Option<String>,
    },

    /// lex a Brainfuck program and display tokens
    Lex {
        /// source file containing Brainfuck code
        file: Option<String>,

        /// Brainfuck code as a string
        #[arg(short, long, conflicts_with = "file")]
        source: Option<String>,
    },

    /// parse a Brainfuck program and display its AST
    Parse {
        /// source file containing Brainfuck code
        file: Option<String>,

        /// Brainfuck code as a string
        #[arg(short, long, conflicts_with = "file")]
        source: Option<String>,
    },
}

fn main() {
    run_cli().unwrap_or_else(|e| eprintln!("{}", e));
}

fn run_cli() -> BFResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file, source } => {
            let source_code = read_source(file, source)?;
            run(&source_code)?;
        }
        Commands::Lex { file, source } => {
            let source_code = read_source(file, source)?;
            let tokens = lex(&source_code)?;
            println!("{:?}", tokens);
        }
        Commands::Parse { file, source } => {
            let source_code = read_source(file, source)?;
            let tokens = lex(&source_code)?;
            let ast = parse(&tokens)?;
            println!("{}", ast);
        }
    };

    Ok(())
}

fn read_source(file: Option<String>, source: Option<String>) -> BFResult<String> {
    match (file, source) {
        (Some(file_path), None) => {
            let content = fs::read_to_string(file_path).map_err(|e| BFError::Io { internal: e })?;
            Ok(content)
        }
        (None, Some(source_code)) => Ok(source_code),
        _ => panic!("either a file or a source string must be provided."),
    }
}
