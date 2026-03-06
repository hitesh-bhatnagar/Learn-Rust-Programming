// TO RUN THE EXAMPLE : cargo run --example 01_variables

fn main() {
    
    // PART 1 : Immutable vs Mutable Variables
    let x = 5;
    let mut y = 10;
    y = 20 ;
    
    println!("x = {} ", x);
    println!("y = {} ", y);
    
    // PART 2 : Data Types
    
    let age: u32 = 25; // unsigned 32-bit integer -- can't be negative
    let count: i32 = 100; // signed 32=bit integer -- can be negative or positive
    let big: i64 = 1_000_000; // underscores make numbers readable
    let pi: f64 = 3.14159; // float
    let flag: bool = true; // boolean
    let letter: char = 'A'; // character
    let name: &str = "Hitesh Bhatnagar"; // fixed text known at compile time
    
    let greeting: String = String::from("Hello, Universe"); // owned text - flexible
    
    // PART 3: Shadowing 
    
    // shadowing allows redeclare a variable with the same name
    // Each let x creates a BRAND NEW VARIABLE
    let x = 5;
    let x = x*2; // new x created, old x is deleted
    let x = x+3; // another new x
    
    println!("Shadowed x  = {}", x); // 13
    
    let spaces = "   "; // &str type
    let spaces = spaces.len(); // now usize ( a number ) -- totally fine 
    println!("spaces = {}", spaces); // 3
    
    // PART 4: mut V/S Shadowing 
    
    let mut score = 0;
    score = 20;
    println!(" score = {}", score);
    
    // Shadowing => Creates new variable, can change VALUE & TYPE
    let status = "inactive";
    println!("status = {}", status);
    let status = true; // changes type from &str to bool
    println!("status = {}", status);
    
    // PART5 : Constants
    
    const euler_num: f32 = 2.71828;
    println!("Max euler_num= {}", euler_num );
    
    
    
}