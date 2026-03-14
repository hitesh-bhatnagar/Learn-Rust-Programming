// TO RUN THE FILE : cargo run --example 02_functions
// use std::io;
use std::io::{self, Write};

fn add(x: i32, y: i32) -> i32{
    x+y         // NOTE : In rust no ; means return 
}

fn prod(x: i32, y: i32) -> i32{
    return x*y; // can work as well
}

fn grade (score : u32) -> &'static str{ // refers to a reference where the borrowed data is valid for the entire duration of the running program
    match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    }
}

fn main() -> io::Result<()>{            // this is for safety so that program do not panic
    println!("add(5,3) = {}", add(5, 3));
    println!("prod(5,3) = {}", prod(5, 3));
    println!("grade(95) = {}", grade(95));
    
    // Conditional expression
    let number = 67;
    
    if number > 5{
        println!("{} is greater than 5", number);
    }else{
        println!("{} is less than 5 ", number);
    }
    
    // In Rust you can assign if/else value to a variable
    let message = if number > 5{"big"} else {"small"};
    println!("Number is {}", message);
    
    // Loops
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 5{
            break count*2;
        }
    };
    
    println!("Loop result = {}", result);
    
    count = 0;
    while count <5{
        count += 1;
        
    }
    println!("While loop result = {}", count);
    
    count = 0;
    for i in 0..=5 {
        count += i;
    }
    println!("For loop result = {}", count);
    
    // MATCH
    
    let cmd = "echo";
    match cmd {
        "quit" => println!("Exiting"),
        "help" => println!("help"),
        "echo" => println!("Hello world"),
        _ => println!("Unknown cmd"),
        
    }
    
//  MATCH can be used inside a variable     
    let code = 404;
    let meaning = match code {
        200 => "OK",
        404 => "Not Found",
        500 => "Server Error",
        _ => "Unknown",
    };
    println!("HTTP {} : {}", code, meaning);
    
    println!("Score 95 = grade {}", grade(95));
    println!("Score 83 = grade {}", grade(83));
    println!("Score 60 = grade {}", grade(60));
    
    
    // PART 5 : Reading user input
    println!("Type a command (echo / help / exit");
    
    let mut input = String::new();
    io::stdout().flush()?;  // ensures the prompt apprears before the program waits for input
    io::stdin().read_line(&mut input)?;
    
    input= input.trim().to_string();
    
    if input.is_empty(){
        println!("Please enter a command");
    } else {
        match input.as_str() {      // when matching against 'input' use as_str() to ensure the type is correct
            "echo" => println!("Hello world "),
            "help" => println!("Available commands : echo, help, exit"),
            "quit" => println!("Exiting ..."),
            other => println!("Unknown Command: {}", other),
        }
    }
    Ok(())
}