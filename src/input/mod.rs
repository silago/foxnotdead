mod input;

use std::io;

pub fn getInput(header : &str) -> String {
    println!("{}: ",header);
    let mut input = String::new();
    while (input.is_empty()) {
        io::stdin().read_line(&mut input);
    }
    return input;
}
