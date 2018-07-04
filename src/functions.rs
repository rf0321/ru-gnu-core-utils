extern crate reqwest;

use std::fs;
use std::fs::File;
use std::io::Read;
use std::env;

pub fn do_cat(filename: &str){
    let mut s = String::new();
    File::open(filename).unwrap().read_to_string(&mut s).unwrap();
    println!("{}",s);
}
pub fn curl_get_request(url: &str){
    let mut response = reqwest::get(url).unwrap();
    let mut s = String::new();
    response.read_to_string(&mut s);
    println!("{}",s);
}
// get current_directory
pub fn get_pwd(){    
    let path = env::current_dir().unwrap();
    println!("{}",path.display());    
}
pub fn get_ls(){  // get current dir and file name
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        println!("{}",path.unwrap().path().display());
    }
}
pub fn do_mkdir(name: &str){ /*make directory. fn argument  is "mkdir <dirname>"*/
    fs::create_dir(name);
}
pub fn do_rmdir(name: &str){
    fs::remove_dir(name).unwrap_or_else( |_|{
        println!("rmdir: cant delete {}{}",name,"No such file or directory");
    });
}
pub fn do_touch(filename: &str){
    fs::File::create(filename);
}
pub fn do_rm(filename: &str){
    fs::remove_file(filename).unwrap_or_else(|_|{
        println!("rm: cant delete {}{}",filename,"No such file or directory");
    });
}
pub fn do_echo(input: &str){
    print!("{}",input);
}