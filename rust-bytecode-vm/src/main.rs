use std::fmt::Display;

pub enum OpCode {
    OpReturn,
    OpConstant,
    OpLongConstant,
}
impl TryFrom<u8> for OpCode {
    type Error = String;
    // OpCode Mapping is here
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OpReturn),
            1 => Ok(Self::OpConstant),
            2 => Ok(Self::OpLongConstant),
            _ => Err(format!("Unkown OpCode, {}", value)),
        }
    }
}
impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let instruction = match self {
            Self::OpReturn => "OpReturn",
            Self::OpConstant => "OpConstant",
            Self::OpLongConstant => "OpLongConstant",
        };
        write!(f, "{}", instruction)
    }
}

// Some Notes:
// Since we are using Vec for our stack, it takes in offsets as usize
#[derive(Default)]
pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<f32>,
    lines: Vec<u32>,
}
impl Chunk {
    pub fn add_code(&mut self, op: u8, line: u32) {
        self.code.push(op);
        self.lines.push(line);
    }
    pub fn free_code(&mut self) {
        self.code = Vec::default();
        self.lines = Vec::default();
    }
    /// Returns offset of where constant is stored in constants stack
    pub fn add_constants(&mut self, constant: f32) -> usize {
        self.constants.push(constant);
        self.constants.len() - 1
    }
    pub fn free_constants(&mut self) {
        self.constants = Vec::default();
    }
    pub fn free(&mut self) {
        self.free_code();
        self.free_constants()
    }

    pub fn disassemble(&self, name: &str) {
        println!("=== {} ===", name);

        let mut offset: usize = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }
    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        let instruction = OpCode::try_from(self.code[offset]).unwrap();
        let instruction_prefix = if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            format!("{:04}    |", offset)
        } else {
            format!("{:04} {:4}", offset, self.lines[offset])
        };
        // Deal iwth instruction/OpCode here
        match instruction {
            OpCode::OpReturn => {
                println!("{} {}", instruction_prefix, instruction);
                offset + 1
            }
            OpCode::OpConstant => {
                let constant_offset = self.code[offset + 1];
                println!(
                    "{} {:16} {} '{}'",
                    instruction_prefix,
                    instruction,
                    constant_offset,
                    self.constants[constant_offset as usize]
                );
                offset + 2
            }
            OpCode::OpLongConstant => {
                // constant_offset is in little endian, last byte is set to 0x00
                // range of constant_offset is therefore 0-16777215
                // converted to u32, since I want to read in 4bytes, 32bits
                let mut constant_offset: [u8; 4] = [0; 4];
                constant_offset[0..3].copy_from_slice(&self.code[(offset + 1)..(offset + 4)]);
                let constant_offset = u32::from_le_bytes(constant_offset);
                println!(
                    "{} {:16} {} '{}'",
                    instruction_prefix,
                    instruction,
                    constant_offset,
                    self.constants[constant_offset as usize]
                );
                offset + 4
            }
            _ => panic!("OpCode not implemented yet, {}", instruction),
        }
    }
}

enum InterpretResult {
    Ok,
    Error(InterpretError),
}
enum InterpretError {
    Compile,
    Runtime,
}

struct VM {
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

fn main() {
    let mut chunk = Chunk::default();
    let constant_idx = chunk.add_constants(3.1);
    chunk.add_constants(123.2);
    chunk.add_code(1, 123);
    // When we add the constant offset into the stack, we need to encode it as a u8
    chunk.add_code(constant_idx as u8, 123);
    // chunk.add_code(2, 124);
    // chunk.add_code(0x01, 124);
    // chunk.add_code(0x00, 124);
    // chunk.add_code(0x00, 124);
    chunk.add_code(0, 124);
    let mut vm = VM::new(chunk);
    vm.run();
    vm.free();
    // chunk.disassemble("test chunk");
    // chunk.free();
}
