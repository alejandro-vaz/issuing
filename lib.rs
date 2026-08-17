//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> DOCS
#![doc = include_str!("README.md")]

//> HEAD -> FEATURES
#![feature(const_default)]
#![feature(default_field_values)]
#![feature(const_convert)]
#![feature(const_trait_impl)]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> MODULES
mod conversions;
mod identifier;
mod section;
mod span;

//> HEAD -> ALLOC
use alloc::vec::Vec;

//> HEAD -> SPAN
pub use span::Span;

//> HEAD -> IDENTIFIER
pub use identifier::Identifier;

//> HEAD -> SECTION
pub use section::Section;


//^
//^ ISSUE
//^

//> ISSUE -> STRUCT
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Issue {
    pub name: &'static str,
    pub sections: Vec<Section> = const {Vec::default()},
    pub identifier: Identifier = const {Identifier::default()}
}