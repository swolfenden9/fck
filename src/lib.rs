use std::{fs, path::Path};

pub use error::{Error, Result};

use interpreter::Interpreter;
use lexer::lex;
use parser::parse;

pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod parser;

/// Run the provided brainfuck code and return the interpreter.
pub fn run(source: &str) -> Result<Interpreter> {
    let tokens = lex(source)?;
    let ast = parse(&tokens)?;
    let mut interpreter = Interpreter::new();
    interpreter.run(&ast)?;
    Ok(interpreter)
}

/// Read and run the brainfuck code from a file and return the interpreter.
pub fn run_file(path: &Path) -> Result<Interpreter> {
    let source = match fs::read_to_string(path) {
        Ok(v) => v,
        Err(e) => return Err(Error::Io { internal: e }),
    };
    let tokens = lex(&source)?;
    let ast = parse(&tokens)?;
    let mut interpreter = Interpreter::new();
    interpreter.run(&ast)?;
    Ok(interpreter)
}
