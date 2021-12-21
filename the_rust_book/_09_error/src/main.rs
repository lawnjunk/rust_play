use std::io;

fn main() {
    let mut input = String::new();

    println!("type \"y\" to panic, anything else to continue");
    if let Ok(byte_count) = io::stdin().read_line(&mut input) {
        println!("byte_count: {} input: {}", byte_count, input);
        if input.trim() == "y" {
            panic!("you asked for it");
        }
    }

    println!("Hello, world!");
}
