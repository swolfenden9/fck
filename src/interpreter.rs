//! Contains structs and functions for interpreting and running a Brainfuck AST.

use std::io::{stdin, Read};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::error::Error;
use crate::parser::AstNode;

/// Contains the current state of the Brainfuck program.
pub struct Interpreter {
    memory: Vec<u8>,
    pointer: usize,
}

impl Interpreter {
    /// Create a new, empty Brainfuck instance with and empty tape and a pointer at zero.
    pub fn new() -> Self {
        Self {
            memory: vec![0; 30_000],
            pointer: 0,
        }
    }

    /// Run the provided AST.
    pub fn run(&mut self, ast: &AstNode) -> Result<(), Error> {
        self.execute(ast)
    }

    fn execute(&mut self, node: &AstNode) -> Result<(), Error> {
        match node {
            AstNode::Sequence(nodes) => {
                for node in nodes {
                    self.execute(node)?;
                }
            }
            AstNode::Right(n) => match self.pointer.checked_add(*n) {
                Some(v) => self.pointer = v,
                None => {
                    return Err(Error::MemoryOutOfBounds {
                        position: self.pointer,
                    })
                }
            },
            AstNode::Left(n) => match self.pointer.checked_sub(*n) {
                Some(v) => self.pointer = v,
                None => {
                    return Err(Error::MemoryOutOfBounds {
                        position: self.pointer,
                    })
                }
            },
            AstNode::Increment(n) => {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(*n as u8)
            }
            AstNode::Decrement(n) => {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(*n as u8)
            }
            AstNode::Output => print!("{}", self.memory[self.pointer] as char),
            AstNode::Input => {
                let mut buffer = [0; 1];
                enable_raw_mode()?; // Enter raw mode to capture input
                stdin().read_exact(&mut buffer)?;
                disable_raw_mode()?; // Exit raw mode
                print!("{}", buffer[0] as char);

                self.memory[self.pointer] = buffer[0];
            }
            AstNode::Clear => self.memory[self.pointer] = 0,
            AstNode::Loop(body) => {
                while self.memory[self.pointer] != 0 {
                    self.execute(&AstNode::Sequence(body.to_owned()))?;
                }
            }
        }
        Ok(())
    }

    /// Get the value at the provided memory location.
    pub fn memory(&self, pointer: usize) -> u8 {
        self.memory[pointer]
    }
}
