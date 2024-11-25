//! # FCK
//! 
//! A simple Brainfuck lexer, parser, and interpreter.
//! 
//! ## Examples
//! 
//! Using `run` and `run_file`.
//! ```rust
//! use fck::{run, run_file};
//! 
//! fn main() -> fck::Result<()> {
//!   run("+++++++++.[->+<]")?;
//!   run_file("path/to/file")?;
//! }
//! ```
//! 
//! Using the individual modules.
//! ```rust
//! use fck::lexer::lex;
//! use fck::parser::parse;
//! use fck::interpreter::Interpreter;
//! 
//! fn main() -> fck::Result<()> {
//!   let tokens = lex("source code")?;
//!   let ast = parse(&tokens)?;
//!   let mut interpreter = Interpreter::new();
//!   interpreter.run(&ast)?;
//! }
//! ```

use std::{fs, path::Path};

pub use error::{Error, Result};

use interpreter::Interpreter;
use lexer::lex;
use parser::parse;

pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod parser;

/// Run the provided brainfuck source code and return the interpreter instance.
pub fn run(source: &str) -> Result<Interpreter> {
    let tokens = lex(source)?;
    let ast = parse(&tokens)?;
    let mut interpreter = Interpreter::new();
    interpreter.run(&ast)?;
    Ok(interpreter)
}

/// Run the brainfuck source code at the path and return the interpreter instance.
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
