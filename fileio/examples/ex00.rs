use std::env;
use std::fs;
use std::process;
use std::string::String;

fn read_file_to_string(filename: String) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = "ex00.txt";

    if args.len() == 2 {
        filename = &args[1];
    }

    println!("In file {}", filename);

    let contents = read_file_to_string(filename.to_string());

    println!("With text:\n{}", contents);
}
