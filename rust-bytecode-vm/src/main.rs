use std::fmt::Display;

pub enum OpCode{
    OpReturn,
}
impl OpCode{
    pub fn process_instruction(&self, offset: usize) -> usize{
        match self{
            Self::OpReturn => {
                offset + 1
            }
        }
    }
}
impl Display for OpCode{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let instruction = match self{
            Self::OpReturn => "OpReturn"
        };
        write!(f, "{}", instruction)
    }
}


#[derive(Default)]
pub struct Chunk{
    code: Vec<OpCode>,
}
impl Chunk{
    pub fn write(&mut self, op: OpCode) {
        self.code.push(op);
    }
    pub fn free(&mut self){
        self.code = Vec::default();
    }

    pub fn disassemble(&self, name: &str){
        println!("=== {} ===", name);

        let mut offset: usize = 0;
        while offset < self.code.len(){
            offset = self.disassemble_instruction(offset);
        }
    }
    pub fn disassemble_instruction(&self, offset: usize) -> usize{
        let instruction = &self.code[offset];
        println!("{:04} {}", offset, instruction);
        instruction.process_instruction(offset)
    }
}


fn main() {
    let mut chunk = Chunk::default();
    chunk.write(OpCode::OpReturn);
    chunk.disassemble("test chunk");
    chunk.free();
}
