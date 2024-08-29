use std::{env, fs, io::Write};

use rust_bytecode_vm::{
    helper,
    vm::{InterpretError, InterpretResult},
    Chunk, Scanner, Token, TokenType, VM,
};

fn old_main() {
    let mut chunk = Chunk::default();
    let constant_offset = chunk.add_constant(1.2);
    chunk.add_code(1, 123);
    // When we add the constant offset into the stack, we need to encode it as a u8
    chunk.add_code(constant_offset as u8, 123);

    let constant_offset = chunk.add_constant(3.4);
    chunk.add_code(1, 123);
    chunk.add_code(constant_offset as u8, 123);

    chunk.add_code(4, 123);

    let constant_offset = helper::constant_offset_to_long_constant_offset(chunk.add_constant(5.6));
    chunk.add_code(2, 123);
    chunk.add_code(constant_offset[0], 123);
    chunk.add_code(constant_offset[1], 123);
    chunk.add_code(constant_offset[2], 123);

    chunk.add_code(7, 123);

    // chunk.add_code(2, 124);
    // chunk.add_code(0x01, 124);
    // chunk.add_code(0x00, 124);
    // chunk.add_code(0x00, 124);
    chunk.add_code(3, 123);
    chunk.add_code(0, 124);
    let mut vm = VM::new(chunk);
    vm.run();
    vm.free();
    // chunk.disassemble("test chunk");
    // chunk.free();
}

fn compile(source: String) {
    let source: Vec<char> = source.chars().collect();
    let mut scanner = Scanner::new(source);
    let mut line = 2i64.pow(60) as usize; // Big number that should not be correct on first iteration
    loop {
        let token = scanner.scan_token();
        match token {
            Token::ErrorToken {
                line: token_line,
                msg,
            } => {
                let line_fmt = if line == token_line {
                    String::from("   |")
                } else {
                    line = token_line;
                    format!("{:4}", line)
                };
                println!("{} {:16} '{}'", line_fmt, "ErrorToken", msg);
            }
            Token::NormalToken {
                _type,
                start,
                length,
                line: token_line,
            } => {
                let line_fmt = if line == token_line {
                    String::from("   |")
                } else {
                    line = token_line;
                    format!("{:4}", line)
                };
                println!(
                    "{} {:16} '{}'",
                    line_fmt,
                    _type,
                    match _type {
                        TokenType::Eof => {
                            String::from("EOF")
                        }
                        _ => {
                            scanner.source[start..start + length]
                                .iter()
                                .collect::<String>()
                        }
                    }
                );

                // Stop compiling
                if let TokenType::Eof = _type {
                    break;
                }
            }
        }
    }
}

fn interpret(source: String) -> InterpretResult {
    compile(source);
    InterpretResult::Ok
}

fn repl() -> Result<(), String> {
    // #[allow(clippy::never_loop)]
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => {
                // dbg!(line.clone());
                if line.len() == 1 {
                    // Empty line
                    break Ok(());
                }
                interpret(line);
            }
            Err(_) => {
                // If stdin receives and err
                break Ok(());
            }
        };
    }
}

fn run_file(file: &str) -> Result<(), String> {
    let source = fs::read_to_string(file).expect("Read have read the file contents");
    match interpret(source) {
        InterpretResult::Ok => Ok(()),
        InterpretResult::Error(e) => match e {
            InterpretError::Runtime => Err(String::from("Runtime")),
            InterpretError::Compile => Err(String::from("Compile")),
        },
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        repl()
    } else if args.len() == 2 {
        run_file(&args[1])
    } else {
        Err(String::from("Usage: clox [path]"))
    }
}
