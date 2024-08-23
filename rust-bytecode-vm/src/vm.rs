use crate::{Chunk, OpCode, STACK_MAX};

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
    stack: [f32; STACK_MAX],
    // Points just past the element at the top of the stack
    stack_top: usize,
}
impl VM {
    fn init(&mut self) {}
    pub fn new(chunk: Chunk) -> Self {
        let mut vm = Self {
            chunk,
            ip: 0,
            stack: [0f32; STACK_MAX],
            stack_top: 0,
        };
        vm.init();
        vm
    }
    pub fn free(&mut self) {
        self.chunk.free();
    }

    fn read_byte(&mut self) -> u8 {
        let b = self.chunk.code[self.ip];
        self.ip += 1;
        b
    }
    fn push(&mut self, value: f32) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }
    fn pop(&mut self) -> f32 {
        let value = self.stack[self.stack_top - 1];
        self.stack_top -= 1;
        value
    }
    pub fn run(&mut self) -> InterpretResult {
        #[allow(clippy::never_loop)]
        loop {
            #[cfg(feature = "debug")]
            {
                println!("{:#?}", self.stack);
                self.chunk.disassemble_instruction(self.ip);
            }
            let instruction = OpCode::try_from(self.read_byte()).unwrap();
            match instruction {
                OpCode::OpReturn => {
                    println!("{}", self.pop());
                    return InterpretResult::Ok;
                }
                OpCode::OpConstant => {
                    let constant_offset = self.read_byte() as usize;
                    let value = self.chunk.constants[constant_offset];
                    // println!("{}", value);
                    self.push(value);
                }
                OpCode::OpNegate => {
                    let value = self.pop();
                    self.push(-value);
                }
                _ => return InterpretResult::Error(InterpretError::Compile),
            }
        }
    }
}
