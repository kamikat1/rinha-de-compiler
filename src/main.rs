#![allow(unused)]

use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct File {
    name: String,
    expression: Term,
    //location: 
}


#[derive(Debug, Deserialize)]
pub struct Int {
    value: i32,
}

#[derive(Debug, Deserialize)]
pub struct Str {
    value: String,
}

#[derive(Debug, Deserialize)]
pub struct Print {
    value: Box<Term>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    Int(Int),
    Str(Str),
//  #[serde(alias = "Print")]
    Print(Print),
}


#[derive(Debug)]
pub enum Val {
    Void,
    Int(i32),
    Bool(bool),
    Str(String),
}


fn eval(term: Term) -> Val {
        match term {
            Term::Int(number) => Val::Int(number.value),
            Term::Str(str) => Val::Str(str.value),
            Term::Print(print) => {
                let val = eval(*print.value);
                match val {
                    Val::Int(i) => print!("{i}"),
                    Val::Bool(b) => print!("{b}"),
                    Val::Str(s) => print!("{s}"),
                    _ => panic!("Value not supported."),
                };
                Val::Void
            },
        }
}
    
// The main function wrapper around [`crate::program`].
fn main() {
    let program: String = fs::read_to_string("./examples/hello.json").unwrap();
    let program: File = serde_json::from_str::<File>(&program).unwrap();
    let term = program.expression;
    eval(term);
    // println!("{program:?}");
    // Avoid printing print `Error: ` before the error message
    // to maintain the language beauty!
    //if let Err(e) = rinha::program() {
    //    eprintln!("{e:?}");
    //    std::process::exit(1);
    //}
}