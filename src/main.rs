use std::env;
use std::io;
use std::fs;

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
fn do_mkdir(name: String){ /*make directory. fn argument  is "mkdir <dirname>"*/
    fs::create_dir(name);
}
fn main() {

}
