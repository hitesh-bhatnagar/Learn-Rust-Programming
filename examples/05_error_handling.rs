use std::fs;
use std::io::{self, Read};
use std::num::ParseIntError;

// FUNCTIONS defined outside main()
//
fn divide(a: f64, b: f64) -> Result<f64, String> {
    // Returns result --> caller must handle Ok or Err
    if b == 0.0 {
        Err(String::from("Can't divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let contents = fs::read_to_string(path)?; // returns Err if file missing
    Ok(contents)
}

// Using ? with multiple operations
fn read_and_parse(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?; // could fail -> file missing
    let number: i32 = contents.trim().parse()?; // could fail -> not a number
    Ok(number)
}

// Custom Error Type
#[derive(Debug)]
enum AppError {
    NotFound(String),
    ParseFailed(String),
    DivisionByZero,
}

// How this error displays when printed
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::NotFound(e) => write!(f, "Not found: {}", e),
            AppError::ParseFailed(s) => write!(f, "Parse failed: {}", s),
            AppError::DivisionByZero => write!(f, "Cannot divide by zero"),
        }
    }
}

fn safe_divide(a: f64, b: f64) -> Result<f64, AppError> {
    if b == 0.0 {
        Err(AppError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = "42".parse::<i32>().unwrap(); // unwrap() => give me the value, crash if Err
    println!("parsed: {}", result);

    // PART - 2: match - full control over Ok and Err

    match divide(10.0, 2.0) {
        Ok(result) => println!("10/2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("10/ 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match read_file("hello.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Could not read file: {}", e),
    }

    // PART 4: unwrap_or == safe default value

    // if Err, use the default value instead of crashing
    let name = std::env::var("USERNAME").unwrap_or(String::from("guest"));
    println!("Hello, {}!", name);

    let number = "abc".parse::<i32>().unwrap_or(0);
    println!("parsed number (or 0): {}", number);

    // PART 5: if let -- handle just the success case

    // if let Ok -- only runs if result is Ok
    if let Ok(n) = "99".parse::<i32>() {
        println!("Successfully parsed: {}", n);
    }

    // uf let Some -- only runs if Option has a value
    let numbers = vec![1, 2, 3];
    if let Some(first) = numbers.first() {
        println!("First number: {}", first);
    }

    // Part 6: Custom Error Types

    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("AppError: {}", e),
    }

    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("AppError: {}", e),
    }

    // PART 7: Three levels of Error Handling -- side by side

    // level 1 -- unwrap (crash on error)
    let a = "10".parse::<i32>().unwrap();

    // level 2 -- unwrap_or (default on error)
    let b = "bad".parse::<i32>().unwrap_or(0);

    // level 3 -- match (full control)
    let c = match "42".parse::<i32>() {
        Ok(n) => n,
        Err(_) => -1,
    };

    println!("a = {}, b = {}, c = {}", a, b, c)
}
