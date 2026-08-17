//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    Issue,
    span::Span
};

//> HEAD -> ALLOC
use alloc::{
    string::String,
    boxed::Box
};


//^
//^ SECTION
//^

//> SECTION -> ENUM
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Section {
    Code {
        extends: Box<Section>,
        code: String,
        language: Option<&'static str> = None,
        path: Option<String> = None,
        line: Option<usize> = None,
        span: Span = Span::RangeFull(..),
    },
    Cause(String),
    Deprecated(String),
    Child(Issue),
    Help(String),
    Note(String)
}