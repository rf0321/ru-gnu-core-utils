use parse;
use self::parse::{
    char_whitespace,
}

pub enum Token{
    WhiteSpace,
    PWD,
    LS,
    MKDIR,
}