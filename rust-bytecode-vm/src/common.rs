use std::fmt::Display;

pub enum OpCode {
    OpReturn,
    OpConstant,
    OpLongConstant,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
}
impl TryFrom<u8> for OpCode {
    type Error = String;
    // OpCode Mapping is here
    // When adding a new OpCode, ensure functionality is added to chunk.disassemble_instruction and vm.run
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OpReturn),
            1 => Ok(Self::OpConstant),
            2 => Ok(Self::OpLongConstant),
            3 => Ok(Self::OpNegate),
            4 => Ok(Self::OpAdd),
            5 => Ok(Self::OpSubtract),
            6 => Ok(Self::OpMultiply),
            7 => Ok(Self::OpDivide),
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
            Self::OpNegate => "OpNegate",
            Self::OpAdd => "OpAdd",
            Self::OpSubtract => "OpSubtract",
            Self::OpMultiply => "OpMultiply",
            Self::OpDivide => "OpDivide",
        };
        write!(f, "{}", instruction)
    }
}
