#[macro_use]
#[warn(unused_must_use)]
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
fn parse_echo_keyword(input: &[u8]) -> IResult<&[u8], &[u8]>{
    tag!( input, "echo ")
}
fn parse_mkdir_keyword(input: &str) -> IResult<&str, &str>{
    tag!( input, "mkdir ")
}
fn call_function_ls_keyword(input: &str){
    let ls_keyword = "ls".as_bytes();
    match input.as_bytes(){
        ls_keyword => get_ls(),
    }
}
fn call_function_pwd_keyword(input: &str){
    let pwd_keyword = "pwd".as_bytes();
    match input.as_bytes() {
        pwd_keyword => get_pwd(),
    }
}
fn excute_echo(input: &str){
    match parse_echo_keyword(input.as_bytes()) {
        IResult::Done(i, _) => {
            println!("{}",String::from_utf8(i.to_vec()).unwrap())
        }
        IResult::Error(error) => println!("Error: {:?}", error),
        IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
    };
}
fn excute_mkdir(input: &str){
    match parse_mkdir_keyword(input){
        IResult::Done(operand, _) => do_mkdir(operand),
        IResult::Error(error) => println!("Error: {:?}", error),
        IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
    }
}
fn string_to_static_str(s: String) -> &'static str { // static String string_to_static_str()
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
            excute_mkdir(input_to_parser);
        }
        else if input_to_parser.starts_with("ls") {
            call_function_ls_keyword(input_to_parser);
        }
        else if input_to_parser.starts_with("pwd") {
            call_function_pwd_keyword(input_to_parser);
        }
        else if input_to_parser.starts_with("echo") {
            excute_echo(input_to_parser);
        }
        else {
            println!("The commad is not found");
        }
     }
}
