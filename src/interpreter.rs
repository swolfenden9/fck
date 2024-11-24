use crate::error::Error;
use crate::parser::AstNode;

pub struct Interpreter {
    memory: Vec<u8>,
    pointer: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            memory: vec![0; 30_000],
            pointer: 0,
        }
    }

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
            AstNode::Input => unimplemented!("input is not implemented"),
            AstNode::Clear => self.memory[self.pointer] = 0,
            AstNode::Loop(body) => {
                while self.memory[self.pointer] != 0 {
                    self.execute(&AstNode::Sequence(body.to_owned()))?;
                }
            }
        }
        Ok(())
    }
}
