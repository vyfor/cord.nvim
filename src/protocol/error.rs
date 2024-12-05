#[derive(Debug)]
pub enum ProtocolError {
    InvalidLength,
    InvalidMapKey,
    InvalidSyntax(&'static str),
    InvalidUtf8,
    UnexpectedByte(u8),
    UnexpectedChar(char),
    UnexpectedEnd,
}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolError::InvalidLength => write!(f, "Invalid data length"),
            ProtocolError::InvalidMapKey => write!(f, "Invalid map key"),
            ProtocolError::InvalidSyntax(msg) => {
                write!(f, "Invalid syntax: {}", msg)
            }
            ProtocolError::InvalidUtf8 => write!(f, "Invalid utf8"),
            ProtocolError::UnexpectedByte(b) => {
                write!(f, "Unexpected byte: {:#x}", b)
            }
            ProtocolError::UnexpectedChar(c) => {
                write!(f, "Unexpected character: '{}'", c)
            }
            ProtocolError::UnexpectedEnd => {
                write!(f, "Unexpected end of input")
            }
        }
    }
}

impl std::error::Error for ProtocolError {}

impl From<std::string::FromUtf8Error> for ProtocolError {
    fn from(_: std::string::FromUtf8Error) -> Self {
        ProtocolError::InvalidUtf8
    }
}
