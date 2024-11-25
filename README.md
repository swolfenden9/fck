# FCK

Fck is a simple Brainfuck lexer, parser, and interpreter. It includes a library crate that exports most of the functionality, and an executable that provides a CLI for lexing, parsing, and executing Brainfuck programs.

## Fck binary

To see a list of all commands, use:
```bash
fck --help
```

## Examples

Using `run` and `run_file`.
```rust
use fck::{run, run_file};

fn main() -> fck::Result<()> {
  run("+++++++++.[->+<]")?;
  run_file("path/to/file")?;
}
```

Using the individual modules.
```rust
use fck::lexer::lex;
use fck::parser::parse;
use fck::interpreter::Interpreter;

fn main() -> fck::Result<()> {
  let tokens = lex("source code")?;
  let ast = parse(&tokens)?;
  let mut interpreter = Interpreter::new();
  interpreter.run(&ast)?;
}
```
