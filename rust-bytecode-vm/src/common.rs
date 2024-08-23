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
