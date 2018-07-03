#[macro_use]
#[allow(unused_must_use)]
#[warn(unused_must_use)]
extern crate nom;
use nom::{IResult, space, alpha};

named!(command_parser<&str,&str>,
    ws!(tag_s!("Hello, "))
);
fn print(){
    println!("aa");
}

pub fn call() {
    match command_parser("Hello,"){
        IResult::Done(operand, _) => print(),
        IResult::Error(error) => println!("Error: {:?}", error),
        IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
    }
}