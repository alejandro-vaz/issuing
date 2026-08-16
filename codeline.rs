//^
//^ HEAD
//^

//> HEAD -> ALLOC
use alloc::string::String;

//> HEAD -> SUPER
use super::span::Span;


//^
//^ CODELINE
//^

//> CODELINE -> STRUCT
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Codeline {
    pub code: String,
    pub message: String,
    pub number: Option<usize> = None,
    pub span: Option<Span> = None
}