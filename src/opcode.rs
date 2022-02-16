#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum OpCode {
    Constant,
    Return,
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == OpCode::Constant as u8 => Ok(OpCode::Constant),
            x if x == OpCode::Return as u8 => Ok(OpCode::Return),
            _ => Err(()),
        }
    }
}
