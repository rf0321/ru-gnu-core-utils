#[macro_use]
#[allow(unused_must_use)]
#[warn(unused_must_use)]
extern crate nom; //  use parser combinator lib
extern crate token;
use nom::{ IResult };
use std::io;
use std::mem;

mod functions;
mod test_parse;

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

fn handle_echo_command(input:&str){
    match echo_command_parser(input) {
        IResult::Done(operand, _) => functions::do_echo(operand),
        IResult::Error(_) => println!(""),
        IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
    }
}
fn handle_curl_command(input:&str){

}
fn handle_touch_command(input:&str){

}
fn handle_rm_command(input:&str){

}
fn handle_mkdir_command(input:&str){
    match mkdir_command_parser(input){
        IResult::Done(operand, _) => functions::do_mkdir(operand),
        IResult::Error(error) => println!("Error: {:?}", error),
        IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
    }
}
fn handle_rmdir_command(input:&str){
    match rmdir_command_parser(input){
            IResult::Done(operand, _) => functions::do_rmdir(operand),
            IResult::Error(error) => println!("Error: {:?}", error),
            IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed)
    }
}
fn handle_cat_command(input:&str){

}

fn tokenize_command(input: &str){
    let separators = vec![' ', '\n', '\t', '\r'];
    let mut tokenizer = token::Tokenizer::new(input.as_bytes(), separators);

    loop {
        let token = match tokenizer.next().unwrap() {
            Some(token) => token,
            None => { break; }
        };
        match token {
            "echo"   => handle_echo_command(input),
            "curl"   => handle_curl_command(input),
            "touch"  => handle_touch_command(input),
            "rm"     => handle_rm_command(input),
            "mkdir"  => handle_mkdir_command(input),
            "rmdir"  => handle_rmdir_command(input),
            "cat"    => handle_cat_command(input),
            "ls"     => functions::get_ls(),
            "pwd"    => functions::get_pwd(),
            _=> println!("")
        }
    }
}

fn string_to_static_str(s: String) -> &'static str { //  ReadLine's String convert to static str
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
   //     excute_command(input_to_excuter);
    }
}
fn main() {
    test_parse::call();
}

#[test]
fn test_tokenize_command(){
    tokenize_command("echo Hoge");
    tokenize_command("Hi! Im 17 guy. I like to play internet game")
}
#[test]
fn call_module_function(){
    functions::do_echo("Function called!");
    functions::curl_get_request("http://example.com");    
    functions::get_ls();    
}