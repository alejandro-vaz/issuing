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
    Code {
        code: String,
        language: Option<&'static str> = None,
        line: Option<usize> = None,
        message: Option<String> = None,
        span: Option<Span> = None,
    },
    Traceback(String),
    Child(Issue),
    Help(String),
    Note(String)
}