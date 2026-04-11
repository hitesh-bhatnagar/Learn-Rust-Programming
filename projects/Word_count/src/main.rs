use std::env;
use std::fs;
use std::process;

// to run the program
//      cargo run -- sample.txt

// it prints : lines, words, characte1rs

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: word count cli tool <file_path>");
        process::exit(1);
    }

    let file_path = &args[1];

    // fs::read_to_string reads the entire file into memory
    // that is fine for small projects
    // for hug, use buffered reading instead

    let content = match fs::read_to_string(file_path) {
        Ok(something) => something,
        Err(error) => {
            eprintln!("Error reading '{}': {}", file_path, error);
            process::exit(1);
        }
    };

    let line_count = content.lines().count(); //count lines by splitting on line boundaries

    let word_count = content.split_whitespace().count(); // count words by splitting whitespace,split_whitespace treats spaces, tabs,newlines as seperators

    let char_count = content.chars().count();

    println!("File: {}", file_path);
    println!("Lines: {}", line_count);
    println!("words: {}", word_count);
    println!("characters : {}", char_count);
}
