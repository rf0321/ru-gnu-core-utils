#[macro_use]
extern crate nom;

use nom::{IResult, space, alpha};


fn test4(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag!(input, "Hello,")
}


fn main() {
    let sample = "Hello, World!";

    let tes: &[u8] = &[0; 4];

    match test4(sample.as_bytes()) {
        IResult::Done(i,_) => {
            println!(
                "{}",
                String::from_utf8(i.to_vec()).unwrap())
        }
        _ => println!("Other!\n"),
    };
}