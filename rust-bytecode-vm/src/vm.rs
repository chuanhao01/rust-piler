use crate::{Chunk, OpCode};

pub enum InterpretResult {
    Ok,
    Error(InterpretError),
}
pub enum InterpretError {
    Compile,
    Runtime,
}

pub struct VM {
    chunk: Chunk,
    // Instruction Pointer
    ip: usize,
}
impl VM {
    pub fn init() {}
    pub fn new(chunk: Chunk) -> Self {
        Self::init();
        Self { chunk, ip: 0 }
    }
    pub fn free(&mut self) {
        self.chunk.free();
    }
    pub fn read_byte(&mut self) -> u8 {
        let b = self.chunk.code[self.ip];
        self.ip += 1;
        b
    }
    pub fn run(&mut self) -> InterpretResult {
        #[allow(clippy::never_loop)]
        loop {
            #[cfg(feature = "debug")]
            {
                self.chunk.disassemble_instruction(self.ip);
            }
            let instruction = OpCode::try_from(self.read_byte()).unwrap();
            match instruction {
                OpCode::OpReturn => {
                    return InterpretResult::Ok;
                }
                OpCode::OpConstant => {
                    let constant_offset = self.read_byte() as usize;
                    let value = self.chunk.constants[constant_offset];
                    println!("{}", value);
                }
                _ => return InterpretResult::Error(InterpretError::Compile),
            }
        }
    }
}
