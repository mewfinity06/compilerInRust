#[allow(dead_code, unused_variables)]
use std::env;
use std::fs;

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut contents = fs::read_to_string(file_path)
        .expect("Should've been able to read file!\n");

    contents.push(' ');

    let tokens = lexer::tokenize(contents);

    for token in tokens {
        println!("{:#?}", token);
    }
}