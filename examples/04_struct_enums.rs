use std::io;

// STRUCTS --> defined outside main()

struct User {
    username: String,
    email: String,
    age: u32,
    is_active: bool,
}

struct Rectangle {
    width: f64,
    length: f64,
}

// impl = attach methods to a struct
impl Rectangle {
    fn new(width: f64, length: f64) -> Self {
        Self { width, length }
    }

    // & self = borrow the struct -> don't consume it

    fn area(&self) -> f64 {
        self.width * self.length
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.length)
    }

    fn is_square(&self) -> bool {
        self.width == self.length
    }
}

// ENUMS --> defined outside main()
enum Direction {
    North,
    South,
    West,
    East,
}

// Enum with DATA attached to variants
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

enum Cmd {
    Echo(String),
    Cd(String),
    Exit,
    Unknown(String),
}

// FUNCTIONS --> defined outside main()

fn area_of_shape(what_shape: Shape) -> f64 {
    match what_shape {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}

fn parse_command(input: &str) -> Cmd {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    match parts[0] {
        "echo" => Cmd::Echo(parts.get(1).unwrap_or(&"").to_string()), // unwrap_or(&"") means if value exists -> use it, if not -> use empty string
        "cd" => Cmd::Cd(parts.get(1).unwrap_or(&"").to_string()),
        "exit" => Cmd::Exit,
        other => Cmd::Unknown(other.to_string()),
    }
}

fn main() {
    // PART 1 : Creating and Using Structs

    let user = User {
        // Direct struct initialization
        username: String::from("alice"),
        email: String::from("abc@gmail.com"),
        age: 20,
        is_active: true,
    };

    println!("username: {}", user.username);
    println!("email: {}", user.email);
    println!("age: {}", user.age);
    println!("active: {}", user.is_active);

    // PART 2; Structs with Methods

    let rect = Rectangle::new(10.0, 5.0); // constructor like
    println!("area: {}", rect.area());
    println!("perimeter {}", rect.perimeter());
    println!("is square: {}", rect.is_square());

    let square = Rectangle::new(4.0, 4.0);
    println!("is square: {}", square.is_square());

    // Part 3: Basic Enume with Match

    let dir = Direction::North;
    match dir {
        Direction::North => println!("Going north!"),
        Direction::South => println!("Going south!"),
        Direction::East => println!("Going east!"),
        Direction::West => println!("Going west!"),
    }

    // PART 4 : Enum with Date

    let c = Shape::Circle(5.0);
    let r = Shape::Rectangle(4.0, 6.0);
    let t = Shape::Triangle(3.0, 4.0, 5.0);

    println!("Circle area: {:.2}", area_of_shape(c));
    println!("Rectangle area: {:.2}", area_of_shape(r));
    println!("Triangle area: {:.2}", area_of_shape(t));

    // PART 6: Option => values that might not exist

    let numbers = vec![1, 2, 3, 4, 5];

    // .get() return Option -- safely gets in index
    match numbers.get(2) {
        Some(n) => println!("Found {}", n),
        None => println!("Not found"),
    }

    // PART 6: Result - operations that might fail

    let good = "42".parse::<i32>(); // parsing a valid number
    let bad = "abc".parse::<i32>(); // parsing invalid text

    match good {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    match bad {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("could not parse: {}", e),
    }

    // PART 7: Mini Shell using Enum

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match parse_command(input) {
            Cmd::Echo(text) => print!("{}\n", text),
            Cmd::Cd(path) => println!("Would change to : {}", path),
            Cmd::Exit => {
                println!("See yaa!!");
                break;
            }
            Cmd::Unknown(c) => println!("Unknown: {}", c),
        }
    }
}
