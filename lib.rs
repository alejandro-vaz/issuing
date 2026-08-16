//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> DOCS
#![doc = include_str!("README.md")]

//> HEAD -> FEATURES
#![feature(const_convert)]
#![feature(default_field_values)]
#![feature(const_trait_impl)]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> MODULES
mod codeline;
mod conversions;
mod span;

//> HEAD -> ALLOC
use alloc::string::String;

//> HEAD -> CORE
use core::hash::{
    Hash,
    Hasher
};

//> HEAD -> CODELINE
pub use codeline::Codeline;

//> HEAD -> SPAN
pub use span::Span;


//^
//^ ISSUE
//^

//> ISSUE -> STRUCT
#[derive(Debug)]
pub struct Issue {
    pub name: &'static str,
    pub description: Option<String> = None,
    pub help: Option<String> = None,
    pub traceback: Option<String> = None,
    pub codeline: Option<Codeline> = None
}

//> ISSUE -> HASH
impl Hash for Issue {
    fn hash<H: Hasher>(&self, state: &mut H) {return Hash::hash(&self.name, state)}
}
