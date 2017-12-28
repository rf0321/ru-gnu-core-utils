#[macro_use]
extern crate nom; //  use parser combinator lib
use nom::{IResult, space, alpha};
use std::env;
use std::io;
use std::fs;
use std::mem;


// get current_directory
fn get_pwd(){    
    let path = env::current_dir().unwrap();
    println!("{}",path.display());    
}
fn get_ls(){  // get current dir and file name
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        println!("{}",path.unwrap().path().display());
    }
}
fn do_mkdir(name: &str){ /*make directory. fn argument  is "mkdir <dirname>"*/
    fs::create_dir(name);
}
//  parse the mkdir key word
named!(name_parser<&str>,
    chain!(
        tag!("mkdir") ~
        space? ~
        operand: map_res!(
            alpha,
            std::str::from_utf8
        ) ~
        tag!("") ,
        || operand
    )
);
fn parsing_operand_expression(input: &str) {
    match name_parser(input.as_bytes()) {
        IResult::Done(_, operand) => do_mkdir(operand),
		IResult::Error(error) => println!("Error: {:?}", error),
        IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
    }
}
fn parsing_nonoperand_expression(input: &str){
	match input {
	      "ls"  => get_ls(),
	      "pwd"  => get_pwd(),
		    _ => println!("command is not found or umimplemanted now")
	}
}
fn string_to_static_str(s: String) -> &'static str { //  Convert String to &str for parse input
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
		ret
    }
}
fn main(){
	loop {
		let mut standard_input = String::new();
		io::stdin().read_line(&mut standard_input).expect("Failed to read line");
		let input_to_parser: &str = string_to_static_str(standard_input);

		if input_to_parser.starts_with("mkdir"){
			parsing_operand_expression(input_to_parser);
		}
		else {
			parsing_nonoperand_expression(input_to_parser);
		}
	}
}