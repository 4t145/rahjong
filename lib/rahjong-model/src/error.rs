use std::borrow::Cow;

pub enum ErrorKind {
    StatusConflict,
}

pub struct Error {
    kind: ErrorKind,
    message_code: MessageCode,
}

impl Error {
    pub fn new(kind: ErrorKind, message_code: MessageCode) -> Self {
        Error { kind, message_code }
    }
    pub fn status_conflict(code: MessageCode) -> Self {
        Error::new(ErrorKind::StatusConflict, code)
    }
}

pub struct MessageCode(pub Cow<'static, str>);

macro_rules! message_code {
    ($($name:ident)*) => {
       $( pub const $name: MessageCode = MessageCode::new_static(stringify!($name));)*
    };
}
impl MessageCode {
    pub const fn new_static(code: &'static str) -> Self {
        MessageCode(Cow::Borrowed(code))
    }
    message_code!(
        TILE_NOT_IN_HAND
        NOT_IN_TURN_TO_DISCARD
        PON_WITH_DIFFERENT_FACES
    );
}
