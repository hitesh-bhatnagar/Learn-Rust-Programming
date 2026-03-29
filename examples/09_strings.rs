fn main(){

	// PART 1: &str vs String

	// &str --> fized text baked into your program
	let fixed: &str = "hello world";
	println!("&str: {}", fixed);

	// String --> owned, growable txt
	let owned: String = String::from("hello world");
	println!("String: {}", owned);

	// convert &str --> String
	let s1 = "help".to_string();
	let s2 = String::from("me please");
	println!("both equal: {}", s1 == s2);	// true

	// convert String --> &str
	let owned = String::from("hello");
	let borrowed : &str = &owned;		// borrow it as &str
	println!("borrowed: {}", borrowed);

	// PART 2: Building Strings

	// push_str --> appedn a &str
	let mut s =String::from("hello");
	s.push_str(" Universe");
	println!("{} ", s);

	// push --> append a single character
	s.push('!');
	println!("{}", s);

	// + operator --> concatenates
	// NOTE: Consumes first string!!
	let s1 = String::from("Is this a real life ");
	let s2 = String::from("or just fantasy ");
	
	let s3 = s1 + &s2;		// s1 is MOVED here --> can't use s1 after this

	println!("{}", s3);

	// format! macro --> joins w/o consuming (most flexible)
	let first = String::from("Caught in a landscape, ");
	let sec = String::from("no escape from reality");
	let combined = format!("{} {}!", first, sec);

	println!("{}", combined);		//NOTE: first & sec are still valid -- format! borrows them

	// repeat
	let repeated = "ha".repeat(3);
	println!("{}", repeated);

	// PART 3: Trimming
	let s = "		Namaste India		";

	println!("'{}'", s.trim());		// 'Namaste India'
	println!("'{}'", s.trim_start());
	println!("'{}'", s.trim_end());

	// trim specific characters
    let s2 = "###hello###";
    println!("{}", s2.trim_matches('#')); 

    // PART 4: Searching & Checking

    let s = String::from("hello, rust world!!");

    println!("contains 'Rust': {}", s.contains("Rust"));
    println!("starts_with 'Hello': {}", s.starts_with("Hello"));
    println!("ends_with 'World!': {}", s.ends_with("World!"));
    println!("is it empty? : {}", s.is_empty());

    // find --> returns the index of first match, or None
    match s.find("Rust") {
    	Some(i) => println!("'Rust' found at index {}", i),
    	None => println!("not found"),
    }

    // PART 5 : Replacing

    let s = String::from("I love Java and Java was my first programming language");
	
    // replace --> replaces ALL occurences
	let replaced = s.replace("Java", "Rust");
	println!("{}", replaced);

	// replacen --> replaces only N occurrences
	let replaced_once = s.replacen("Java", "Rust", 1);
	println!("{}", replaced_once);		

	// PART 6 : Splitting --> most important for CLI tools

	let sentence = "hello world foo bar";

	let words: Vec<&str> = sentence.split(' ').collect();
	println!("split: {:?}", words);

	// splitn --> split into AT MOST n parts
	// Very useful for parsing commands like "SET key value"
	let cmd = "SET my key some value with spaces";
	let parts: Vec<&str> = cmd.splitn(3, ' ').collect();
	println!("splitn(3): {:?}", parts);
	// ["SET", "my", "key some value with spaces"]
	// NOTICE: everything after the 2nd space stays together

	// split and check first word
	let input = "echo hello world";
	let mut parts = input.splitn(2, ' ');
	let command = parts.next().unwrap_or("");
	let args = parts.next().unwrap_or("");

	println!("command: '{}', args: '{}'", command, args);

	// PART 7: Case Conversion

	let s = "Hello Universe";
	println!("converted to lowercase: {}", s.to_lowercase());
	println!("converted to uppercase: {}", s.to_uppercase());

	// PART 8: Parsing --> converting strings to other types
	let num: i32 = "42".parse().unwrap();
	println!("parsed i32: {}", num);

	let float: f64 = "3.14".parse().unwrap();
	println!("parsed f64: {}", float);

	// Handle parse failure safely
	match "abc".parse::<i32>() {
		Ok(n) => println!("parsed: {}", n),
		Err(e) => println!("parse failed: {}", e),
		
	}	

	//number --> string
	let n = 42;
	let s = n.to_string();
	println!("number to string: {}", s);

	// PART 9: Joining

	let words = vec!["It's", "Rust Lang", "baby"];

	// join --> connect items with a separator
	let sentence = words.join(" ");
	println!("{}", sentence);

	let csv = words.join(",");
	println!("{}", csv);

	// PART 10: Characters

	let s = String::from("hello");
	for i in s.chars() {
		print!("{} ", i);
	}
	println!();

	// Count characters
	let char_count = s.chars().count();
	println!("char count: {}", char_count);
	
}
