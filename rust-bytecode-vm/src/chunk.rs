use crate::OpCode;

// Some Notes:
// Since we are using Vec for our stack, it takes in offsets as usize
#[derive(Default)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<f32>,
    pub lines: Vec<u32>,
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
    pub fn add_constant(&mut self, constant: f32) -> usize {
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
            OpCode::OpReturn
            | OpCode::OpNegate
            | OpCode::OpAdd
            | OpCode::OpSubtract
            | OpCode::OpMultiply
            | OpCode::OpDivide => {
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
                let mut constant_offset: [u8; 4] = Default::default();
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
