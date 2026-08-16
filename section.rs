//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    Issue,
    span::Span
};

//> HEAD -> ALLOC
use alloc::string::String;


//^
//^ SECTION
//^

//> SECTION -> ENUM
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Section {
    Description(String),
    Code {
        code: String,
        message: String,
        line: Option<usize>,
        span: Option<Span>,
        language: &'static str
    },
    Traceback(String),
    Child(Issue),
    Help(String),
    Note(String)
}