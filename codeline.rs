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
#[derive(Debug)]
pub struct Codeline {
    pub code: String,
    pub message: String,
    pub span: Span = Span::RangeFull(..)
}