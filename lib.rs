//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> DOCS
#![doc = include_str!("README.md")]

//> HEAD -> FEATURES
#![feature(const_convert)]
#![feature(const_default)]
#![feature(default_field_values)]
#![feature(const_trait_impl)]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> MODULES
mod code;
mod conversions;
mod identifier;
mod span;

//> HEAD -> ALLOC
use alloc::{
    string::String,
    vec::Vec
};

//> HEAD -> CODE
pub use code::Code;

//> HEAD -> SPAN
pub use span::Span;

//> HEAD -> IDENTIFIER
pub use identifier::Identifier;


//^
//^ ISSUE
//^

//> ISSUE -> ENUM
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Issue {
    Single {
        name: &'static str,
        description: Option<String> = None,
        help: Option<String> = None,
        traceback: Option<String> = None,
        code: Option<Code> = None,
        identifier: Identifier = const {Identifier::default()}
    },
    Group(Vec<Issue>)
}