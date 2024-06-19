#![allow(unused)]

use serde::Deserialize;
use std::fs;
use std::io::{self, Read};

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
pub struct Binary {
    rhs: Box<Term>,
    op: BinaryOp,
    lhs: Box<Term>,
}

#[derive(Debug, Deserialize)]
pub struct Bool {
    value: bool,
}

#[derive(Debug, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    Int(Int),
    Str(Str),
    //  #[serde(alias = "Print")]
    Print(Print),
    Binary(Binary),
    Bool(Bool),
    If(If),
}


#[derive(Debug)]
pub enum Val {
    Void,
    Int(i32),
    Bool(bool),
    Str(String),
}

#[derive(Debug, Deserialize)]
pub struct If {
    condition: Box<Term>,
    then: Box<Term>,
    otherwise: Box<Term>,
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct Let {
    name: Parameter,
    value: Box<Term>,
    next: Box<Term>,
}

fn eval(term: Term) -> Val {
    match term {
        Term::Int(number) => Val::Int(number.value),
        Term::Str(str) => Val::Str(str.value),
        Term::Bool(bool) => Val::Bool(bool.value),
        Term::Print(print) => {
            let val = eval(*print.value);
            match val {
                Val::Int(i) => print!("{i}"),
                Val::Bool(b) => print!("{b}"),
                Val::Str(s) => print!("{s}"),
                _ => panic!("Value not supported."),
            };
            Val::Void
        }

        Term::Binary(bin) => match bin.op {
            BinaryOp::Add => {
                let lhs = eval(*bin.lhs);
                let rhs = eval(*bin.rhs);

                match (lhs, rhs) {
                    (Val::Int(i), Val::Int(j)) => Val::Int(i + j),
                    (Val::Str(s), Val::Int(i)) => Val::Str(format!("{}{}", s, i)),
                    (Val::Int(i), Val::Str(s)) => Val::Str(format!("{}{}", i, s)),
                    (Val::Str(s), Val::Str(t)) => Val::Str(format!("{}{}", s, t)),
                    _ => panic!("Invalid operator used."),
                }
            }

            BinaryOp::Sub => {
                let lhs = eval(*bin.lhs);
                let rhs = eval(*bin.rhs);

                match (lhs, rhs) {
                    (Val::Int(a), Val::Int(b)) => Val::Int(a - b),
                    _ => panic!("Invalid operator used."),
                }
            }
        },

        Term::If(iff) => match eval(*iff.condition) {
            Val::Bool(true) => eval(*iff.then),
            Val::Bool(false) => eval(*iff.otherwise),
            _ => panic!("Condition in If statement must be a boolean."),
        },
    }
}

// The main function wrapper around [`crate::program`].
fn main() {
    let mut program = String::new();
    io::stdin().lock().read_to_string(&mut program).unwrap();
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
