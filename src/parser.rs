//! Contains utilities for parsing a string of Brainfuck tokens into an Abstract Syntax Tree (AST).

use std::fmt::Display;

use crate::error::Error;
use crate::lexer::Token;

/// Represents a node in a Brainfuck program's AST.
#[derive(Debug, Clone)]
pub enum AstNode {
    Sequence(Vec<AstNode>),
    Loop(Vec<AstNode>),
    Clear,
    Right(usize),
    Left(usize),
    Increment(usize),
    Decrement(usize),
    Output,
    Input,
}

impl Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn write_indented(
            f: &mut std::fmt::Formatter<'_>,
            depth: usize,
            s: &str,
        ) -> std::fmt::Result {
            writeln!(f, "{}{}", "  ".repeat(depth), s)
        }

        fn display_node(
            node: &AstNode,
            f: &mut std::fmt::Formatter<'_>,
            depth: usize,
        ) -> std::fmt::Result {
            match node {
                AstNode::Sequence(nodes) => {
                    write_indented(f, depth, "Sequence")?;
                    for node in nodes {
                        display_node(node, f, depth + 1)?;
                    }
                }
                AstNode::Loop(nodes) => {
                    write_indented(f, depth, "Loop")?;
                    for node in nodes {
                        display_node(node, f, depth + 1)?;
                    }
                }
                AstNode::Clear => write_indented(f, depth, "Clear")?,
                AstNode::Right(n) => write_indented(f, depth, &format!("Right({})", n))?,
                AstNode::Left(n) => write_indented(f, depth, &format!("Left({})", n))?,
                AstNode::Increment(n) => write_indented(f, depth, &format!("Increment({})", n))?,
                AstNode::Decrement(n) => write_indented(f, depth, &format!("Decrement({})", n))?,
                AstNode::Output => write_indented(f, depth, "Output")?,
                AstNode::Input => write_indented(f, depth, "Input")?,
            }
            Ok(())
        }

        display_node(self, f, 0)
    }
}

/// Parse a slice of `Token`s into an AST.
pub fn parse(tokens: &[Token]) -> Result<AstNode, Error> {
    // Start parsing from the beginning of the token list
    let mut position = 0;
    let nodes = parse_sequence(tokens, &mut position)?;

    if position != tokens.len() {
        return Err(Error::UnmatchedBracket { position });
    }

    Ok(AstNode::Sequence(nodes))
}

fn parse_sequence(tokens: &[Token], start: &mut usize) -> Result<Vec<AstNode>, Error> {
    let mut nodes = Vec::new();

    while *start < tokens.len() {
        match tokens[*start] {
            Token::Right(n) => nodes.push(AstNode::Right(n)),
            Token::Left(n) => nodes.push(AstNode::Left(n)),
            Token::Increment(n) => nodes.push(AstNode::Increment(n)),
            Token::Decrement(n) => nodes.push(AstNode::Decrement(n)),
            Token::Output => nodes.push(AstNode::Output),
            Token::Input => nodes.push(AstNode::Input),
            Token::Clear => nodes.push(AstNode::Clear),
            Token::LoopStart => {
                *start += 1;
                let loop_body = parse_sequence(tokens, start)?;
                if *start >= tokens.len() || !matches!(tokens[*start], Token::LoopEnd) {
                    return Err(Error::UnmatchedBracket { position: *start });
                }
                nodes.push(AstNode::Loop(loop_body));
            }
            Token::LoopEnd => break,
        }
        *start += 1;
    }

    Ok(nodes)
}
