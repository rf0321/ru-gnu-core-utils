#[macro_use]
#[allow(unused_must_use)]
#[warn(unused_must_use)]
extern crate nom; //  use parser combinator lib
extern crate reqwest;

use std::io::Read;
use nom::{ IResult };
use std::io;
use std::mem;
use std::fs;
use std::fs::File;
use std::env;

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
fn do_echo(input: &str){
    if input.starts_with("$"){
        let path =  input.replace("$","");
        let out_dir = env::var(path).unwrap();
        print!("{}",out_dir);        
    }else {
        print!("{}",input);        
    }
}
named!(echo_command_parser<&str,&str>,
    ws!(tag_s!("echo "))
);
named!(curl_command_parser<&str,&str>,
    ws!(tag_s!("curl "))
);
named!(touch_command_parser<&str,&str>,
    ws!(tag_s!("touch "))
);
named!(mkdir_command_parser<&str,&str>,
    ws!(tag_s!("mkdir "))
);
named!(rmdir_command_parser<&str,&str>,
    ws!(tag_s!("rmdir "))
);
named!(cat_command_parser<&str,&str>,
    ws!(tag_s!("cat "))
);
named!(rm_command_parser<&str,&str>,
    ws!(tag_s!("rm "))
);
fn  excute_command(input: &str){
  if input.starts_with("ls"){
        get_ls();
    }else if input.starts_with("pwd"){
        get_pwd();
    }else if input.starts_with("mkdir"){
        match mkdir_command_parser(input){
            IResult::Done(operand, _) => do_mkdir(operand),
            IResult::Error(error) => println!("Error: {:?}", error),
            IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
        }
    }else if input.starts_with("rmdir"){
        match rmdir_command_parser(input){
            IResult::Done(operand, _) => do_rmdir(operand),
            IResult::Error(error) => println!("Error: {:?}", error),
            IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
        }
    }else if input.starts_with("echo") {
        match echo_command_parser(input) {
            IResult::Done(operand, _) => do_echo(operand),
            IResult::Error(_) => println!(""),
            IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
        }
    }else if input.starts_with("curl"){
        match curl_command_parser(input) {
            IResult::Done(url, _) => curl_get_request(url),
            IResult::Error(_) => println!("curl: usage: curl [URL]"),
            IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
        }
    }else if input.starts_with("rm"){
        match rm_command_parser(input){
            IResult::Done(filename, _) => do_rm(filename),
            IResult::Error(_) => println!("there isnt operand"),
            IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
        }
    }else if input.starts_with("touch"){
        match touch_command_parser(input){
            IResult::Done(filename, _) => do_touch(filename),
            IResult::Error(_) => println!("there isnt operand"),
            IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
        }
    }else if input.starts_with("cat"){
        match cat_command_parser(input){
            IResult::Done(operand, _) => do_cat(operand),
            IResult::Error(error) => println!("Error: {:?}", error),
            IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
        }
    }
    else {
        println!("This command is unimplemented now. Sry :o");
    }
}
#[test]
fn test_parse_greeting(){
    excute_command("echo Hello");
    excute_command("mkdir a");
    excute_command("ls");
    excute_command("Hello");
}
fn string_to_static_str(s: String) -> &'static str { // String convert to static str
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
        ret
    }
}
fn init_dummy_shell(){
    loop {
        let mut standard_input = String::new();
        io::stdin().read_line(&mut standard_input).expect("Failed to read line");
        let input_to_excuter = string_to_static_str(standard_input);
        excute_command(input_to_excuter);
    }
}
fn main() {
  init_dummy_shell();
}

