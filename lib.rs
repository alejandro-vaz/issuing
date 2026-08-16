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
mod codeline;
mod conversions;
mod identifier;
mod span;

//> HEAD -> ALLOC
use alloc::string::String;

//> HEAD -> CODELINE
pub use codeline::Codeline;

//> HEAD -> SPAN
pub use span::Span;

//> HEAD -> IDENTIFIER
pub use identifier::Identifier;


//^
//^ ISSUE
//^

//> ISSUE -> STRUCT
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Issue {
    pub name: &'static str,
    pub description: Option<String> = None,
    pub help: Option<String> = None,
    pub traceback: Option<String> = None,
    pub codeline: Option<Codeline> = None,
    pub identifier: Identifier = const {Identifier::default()}
}