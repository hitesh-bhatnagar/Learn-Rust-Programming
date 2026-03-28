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
	prontln!("borrowed: {}", borrowed);

	// PART 2: Building Strings

	// push_str --> appedn a &str
	let mut s =String::from("hello");
	s.push_str(" Universe");
	println!("{} ", s);

	// push --> append a single character
	s.push('!');
	println!("{}", S);

	// + operator --> concatenates
	// NOTE: Consumes first string!!
	let s1 = String::from("Is this a real life ");
	let s2 = String::from("or just fantasy ");
	
	let s3 = s1+s2;		// s1 is MOVED here --> can't use s1 after this

	println!("{}", s3);

	// format! macro --> joins w/o consuming (most flexible)
	let first = String::from("Caught in a landscape, ");
	let sec = String::from("no escape from reality");
	let combined = format!("{} {}!", first, second);

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
    	Some(i) => println!("'Rust' found at index {}", i);
    	None => println!("not found");
    }

    // PART 5 : Replacing

    let s = String::from("I love Java and Java was my first programming language");
	
    // replace --> replaces ALL occurences
	
		
}
