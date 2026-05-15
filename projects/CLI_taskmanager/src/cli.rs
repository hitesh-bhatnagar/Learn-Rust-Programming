use std::io::{self, Write};

pub struct Cli;

impl Cli {
    pub fn new() -> Self {
        Cli
    }

    pub fn show_menu(&self) {
        println!("\n📝 TASK MANAGER");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Exit");
    }

    pub fn get_input(&self, prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}
