#[macro_use]
#[warn(unused_must_use)]
extern crate nom; //  use parser combinator lib
use nom::{ IResult };
use std::io;
use std::mem;

// get current_directory
fn get_pwd(){    
    let path = std::env::current_dir().unwrap();
    println!("{}",path.display());    
}
fn get_ls(){  // get current dir and file name
    let paths = std::fs::read_dir("./").unwrap();
    for path in paths {
        println!("{}",path.unwrap().path().display());
    }
}
fn do_mkdir(name: &str){ /*make directory. fn argument  is "mkdir <dirname>"*/
    std::fs::create_dir(name);
}
fn do_rmdir(name: &str){
    std::fs::remove_dir(name).unwrap_or_else( |why|{
        println!("rmdir: cant delete {}{}",name,"No such file or directory");
    });
}
fn do_echo(input: &[u8]){  
    println!("{}",String::from_utf8(input.to_vec()).unwrap());        
}
fn excute_echo(input: &str){
    match recognition_echo_keyword(input.as_bytes()) {
        IResult::Done(i, _) => do_echo(i),
        IResult::Error(_) => println!(""),
        IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
    };
}
fn excute_rmdir(input: &str){
    match recognition_rmdir_keyword(input){
        IResult::Done(operand, _) => do_rmdir(operand),
        IResult::Error(error) => println!("Error: {:?}", error),
        IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
    }
}
fn excute_mkdir(input: &str){
    match recognition_mkdir_keyword(input){
        IResult::Done(operand, _) => do_mkdir(operand),
        IResult::Error(error) => println!("Error: {:?}", error),
        IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
    }
}
//  parse keywords
fn recognition_echo_keyword(input: &[u8]) -> IResult<&[u8], &[u8]>{
    tag!( input, "echo ")
}
fn recognition_mkdir_keyword(input: &str) -> IResult<&str, &str>{
    tag!( input, "mkdir ")
}
fn recognition_rmdir_keyword(input: &str) -> IResult<&str, &str> {
    tag!(input, "rmdir ")
}
fn string_to_static_str(s: String) -> &'static str { // String convert to static str
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
        ret
    }
}
fn parse_shell_keyword(input: &str){
    if input.starts_with("ls"){
        get_ls();
    } 
    else if input.starts_with("pwd"){
        get_pwd();
    }
    else if input.starts_with("mkdir"){
        excute_mkdir(input);
    }
    else if input.starts_with("rmdir"){
         excute_rmdir(input);
    }   
    else if input.starts_with("echo") {
        excute_echo(input);
    }
    else {
         println!("The command not found");
    }    
}
fn main(){
	loop {
	let mut standard_input = String::new();
	io::stdin().read_line(&mut standard_input).expect("Failed to read line");
        let input_to_parser = string_to_static_str(standard_input);
        parse_shell_keyword(input_to_parser);
     }
}
