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
    std::fs::create_dir(name);
}
//  parse keywords
named!(mkdir_parser<&str>,
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
fn call_function_mkdir_keyword(input: &str){
    match mkdir_parser(input.as_bytes()){
        IResult::Done(_, operand) => do_mkdir(operand),
        IResult::Error(error) => println!("Error: {:?}", error),
        IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
    }
}
fn call_function_ls_keyword(input: &str){
    let ls_keyword = "ls".as_bytes();
    match input.as_bytes(){
        ls_keyword => get_ls(),
        _ => println!("The command is not found")
    }
}
fn call_function_pwd_keyword(input: &str){
    let pwd_keyword = "pwd".as_bytes();
    match input.as_bytes() {
        pwd_keyword => get_pwd(),
        _=> println!("The command is not found")
    }
}

fn string_to_static_str(s: String) -> &'static str {
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
        let input_to_parser = string_to_static_str(standard_input);
        if input_to_parser.starts_with("mkdir"){
            call_function_mkdir_keyword(input_to_parser);
        }
        else if input_to_parser.starts_with("ls") {
            call_function_ls_keyword(input_to_parser);
        }
        else if input_to_parser.starts_with("pwd") {
            call_function_pwd_keyword(input_to_parser);
        }
     }
}