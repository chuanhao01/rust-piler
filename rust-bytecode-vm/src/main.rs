use std::fmt::Display;

pub enum OpCode {
    OpReturn,
    OpConstant,
}
impl TryFrom<u8> for OpCode {
    type Error = String;
    // OpCode Mapping is here
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OpReturn),
            1 => Ok(Self::OpConstant),
            _ => Err(format!("Unkown OpCode, {}", value)),
        }
    }
}
impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let instruction = match self {
            Self::OpReturn => "OpReturn",
            Self::OpConstant => "OpConstant",
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
            _ => panic!("OpCode not implemented yet, {}", instruction),
        }
    }
}

fn main() {
    let mut chunk = Chunk::default();
    let constant_idx = chunk.add_constants(3.1);
    chunk.add_code(1, 123);
    // When we add the constant offset into the stack, we need to encode it as a u8
    chunk.add_code(constant_idx as u8, 123);
    chunk.add_code(0, 123);
    chunk.disassemble("test chunk");
    chunk.free();
}
