use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    println!("empty map: {:?}, scores");

    // PART 2: Insert, Get, Remove

    let mut map: HashMap<String, String> = HashMap::new();

    // insert - add a key-value pair
    map.insert(String::from("name"), String::from("alice"));
    map.insert(String::from("city"), String::from("Belarus"));
    map.insert(String::from("lang"), String::from("Rust"));
    println!("after inserts: {:?}", map);

    // insert same key- OVERWRITES the old value
    map.insert(String::from("name"), String::from("Bheem"));
    println!("after overwrite: {:?}", map);

    // get -- returns Option<&Value>
    match map.get("name") {
        Some(v) => println!("name = {}", v),
        None => println!("key not found"),
    }

    // get a key that doesn't exist
    match map.get("age") {
        Some(v) => println!("age = {}", v),
        None => println!("age key does not exist"),
    }

    // contains_key - check if key exists
    println!("has city: {}", map.contains_key("city"));
    println!("has age: {}", map.contains_key("age"));

    // remove - delete a key, returns Option<Value>
    let removed = map.remove("city");
    println!("removed: {:?}", removed);
    println!("after remove: {:?}", map);

    // PART 3: entry() - insert only if key doesn't exist

    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(String::from("alice"), 10);

    // or_insert - only inserts if key is NOT already there
    map.entry(String::from("alice")).or_insert(99); // alice exists -- no change
    map.entry(String::from("bob")).or_insert(50); // bob missing -- inserts 50 

    println!("{:?}", map); // alice = 10, bob = 50

    // or_insert returns a mutable reference -- you can modify it
    let alice_score = map.entry(String::from("alice")).or_insert(0);
    *alice_score += 5; // add 5 to alice's score

    println!("{:?}", map);

    // PART 4: Looping over a hashmap

    let mut ages: HashMap<&str, u32> = HashMap::new();
    ages.insert("alice", 25);
    ages.insert("bob", 30);
    ages.insert("carol", 22);

    // loop over all key-value pairs
    // Note: Hashmap has NO guranteed order

    for (name, age) in &ages {
        println!("{} is {}", name, age)
    }

    // Loop over just keys
    println!("keys");
    for key in ages.keys() {
        println!("	{}", key);
    }

    // Loop over just values
    prontln!("values: ");
    for value in ages.values() {
        println!("	{}", value);
    }

    // PART 5 : Useful HashMap Methods

    let mut map: HashMap<&str, i32> = HashMap::new();

    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    println!("length: {}", map.len());

    println!("is_empty ? : {}", map.is_empty());

    let mut temp = map.clone();
    temp.clear();
    println!("after clear length: {}", temp.len());

    // PART 6: Word counter -- classic HashMap patter

    let text = "hello world hello rust world hello";
    let mut word_count: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        // get the count for this word, or start at 0
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_count);
}
