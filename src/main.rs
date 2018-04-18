#[macro_use]
#[allow(unused_must_use)]
extern crate nom; //  use parser combinator lib
extern crate reqwest;

use std::io::Read;
use nom::{ IResult };
use std::io;
use std::mem;
use std::fs;
use std::fs::File;



fn do_cat(filename: &str){
   let mut s = String::new();
   File::open(filename).unwrap().read_to_string(&mut s).unwrap();
   println!("{}",s);
}
fn curl_get_request(url: &str){
    let mut response = reqwest::get(url).unwrap();
    let mut s = String::new();
    response.read_to_string(&mut s);
    println!("{}",s);
}
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
    fs::create_dir(name);
}
fn do_rmdir(name: &str){
    fs::remove_dir(name).unwrap_or_else( |_|{
        println!("rmdir: cant delete {}{}",name,"No such file or directory");
    });
}
fn do_touch(filename: &str){
    fs::File::create(filename);
}
fn do_rm(filename: &str){
    fs::remove_file(filename).unwrap_or_else(|_|{
        println!("rm: cant delete {}{}",filename,"No such file or directory");
    });
}
fn do_echo(input: &[u8]){  
    print!("{}",String::from_utf8(input.to_vec()).unwrap());        
}
fn excute_echo(input: &str){
    match recognition_echo_keyword(input.as_bytes()) {
        IResult::Done(operand, _) => do_echo(operand),
        IResult::Error(_) => println!(""),
        IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
    };
}
fn excute_touch(input:&str){
    match recognition_touch_keyword(input){
        IResult::Done(filename, _) => do_touch(filename),
        IResult::Error(_) => println!("there isnt operand"),
        IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
    };
}
fn excute_rm(input:&str){
    match recognition_rm_keyword(input){
        IResult::Done(filename, _) => do_rm(filename),
        IResult::Error(_) => println!("there isnt operand"),
        IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
    };
}
fn excute_curl(input: &str){
    match recognition_curl_keyword(input){
        IResult::Done(url, _) => curl_get_request(url),
        IResult::Error(_) => println!("curl: usage: curl [URL]"),
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
fn excute_cat(input: &str){
    match recognition_cat_keyword(input){
        IResult::Done(operand, _) => do_cat(operand),
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
fn recognition_curl_keyword(input: &str) -> IResult<&str,&str>{
    tag!(input,"curl ")
}
fn recognition_touch_keyword(input: &str) -> IResult<&str,&str>{
    tag!(input,"touch ")
}
fn recognition_rm_keyword(input: &str) -> IResult<&str,&str>{
    tag!(input,"rm ")
}
fn recognition_cat_keyword(input: &str) -> IResult<&str,&str>{
    tag!(input,"cat ")
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
    }else if input.starts_with("pwd"){
        get_pwd();
    }else if input.starts_with("mkdir"){
        excute_mkdir(input);
    }else if input.starts_with("rmdir"){
        excute_rmdir(input);
    }else if input.starts_with("echo") {
        excute_echo(input);
    }else if input.starts_with("curl"){
        excute_curl(input);    
    }else if input.starts_with("touch"){
        excute_touch(input);
    }else if input.starts_with("rm"){
        excute_rm(input);
    }else if input.starts_with("cat"){
        excute_cat(input);
    }
    else {
        println!("This command is not found");
    }
}
fn init_dummy_shell(){
    loop {
        let mut standard_input = String::new();
        io::stdin().read_line(&mut standard_input).expect("Failed to read line");
        
        let input_to_parser = string_to_static_str(standard_input);
        parse_shell_keyword(input_to_parser);
     }
}
fn main(){
    init_dummy_shell();
}

