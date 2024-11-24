#[derive(Debug, PartialEq)]
pub enum Token {
    Right(usize),
    Left(usize),
    Increment(usize),
    Decrement(usize),
    Output,
    Input,
    LoopStart,
    LoopEnd,
    Clear, // Represents [-]
}

pub fn lex(source: &str) -> Result<Vec<Token>, crate::error::Error> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(&c) = chars.peek() {
        // TODO: Optimise checking
        // if c == '[' {
        //     let mut iter = chars.clone();
        //     if iter.next() == Some('[') && iter.next() == Some('-') && iter.next() == Some(']') {
        //         // Consume the characters for the Clear token
        //         chars.next(); // Consume '['
        //         chars.next(); // Consume '-'
        //         chars.next(); // Consume ']'
        //         tokens.push(Token::Clear);
        //         continue;
        //     }
        // }

        match c {
            '>' => tokens.push(Token::Right(consume(&mut chars, '>'))),
            '<' => tokens.push(Token::Left(consume(&mut chars, '<'))),
            '+' => tokens.push(Token::Increment(consume(&mut chars, '+'))),
            '-' => tokens.push(Token::Decrement(consume(&mut chars, '-'))),
            '.' => {
                tokens.push(Token::Output);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Input);
                chars.next();
            }
            '[' => {
                tokens.push(Token::LoopStart);
                chars.next();
            }
            ']' => {
                tokens.push(Token::LoopEnd);
                chars.next();
            }
            _ => {
                // Ignore other characters
                chars.next();
            }
        }
    }

    Ok(tokens)
}

fn consume(chars: &mut std::iter::Peekable<std::str::Chars>, c: char) -> usize {
    let mut count = 0;
    while chars.peek() == Some(&c) {
        chars.next();
        count += 1;
    }
    count
}
