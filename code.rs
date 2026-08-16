//^
//^ HEAD
//^

//> HEAD -> ALLOC
use alloc::string::String;

//> HEAD -> SUPER
use super::span::Span;


//^
//^ CODE
//^

//> CODE -> STRUCT
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Code {
    pub code: String,
    pub message: String,
    pub number: Option<usize> = None,
    pub span: Option<Span> = None
}