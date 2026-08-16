//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(default_field_values)]

//> HEAD -> ISSUING
use issuing::{
    Issue,
    Section
};


//^
//^ TESTS
//^

//> TESTS -> CREATE
#[test]
fn create() -> () {
    let _issue = Issue::from("hello");
    let _manual = Issue {
        name: "myname",
        sections: Vec::from([
            Section::Help("get help".to_string())
        ]),
        ..
    };
}