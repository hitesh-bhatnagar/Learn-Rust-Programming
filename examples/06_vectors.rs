use std::string;

fn main() {
    // PART 1 -> Creating a Vec

    // Method 1 - empty Vec, type must be specified
    let mut nums: Vec<i32> = Vec::new();
    println!("empty vec: {:?}", nums);

    // Method 2 - vec! macro, type is inferred
    let scores = vec![10, 20, 30, 40, 50];
    println!("Scores: {:?}", scores);

    // Method 3: vec of strings
    let names = vec![
        String::from("alice"),
        String::from("bob"),
        String::from("charlie"),
    ];

    println!("names: {:?}", names);

    // PART 2: Adding & Removing Items

    let mut fruits = Vec::new();

    //push -- add to the END
    fruits.push("apple");
    fruits.push("banana");
    fruits.push("cranberries");
    println!("after push : {:?}", fruits);

    // pop -- remove from the END, returns Option
    let last = fruits.pop();
    match last {
        Some(f) => println!("popped: {}", f),
        None => println!("vec was empty"),
    }

    println!("after pop: {:?}", fruits);

    fruits.insert(1, "mango");
    println!("after insert at 1: {:?}", fruits);

    // remove --> remove at a specific index, returns the value
    let removed = fruits.remove(0);
    println!("removed index 0: {}", removed);
    println!("after remove: {:?}", fruits);

    // PART 3: Accessing Items
    let numbers = vec![10, 20, 30, 40, 50];

    // Method 1
    println!("index 0: {}", numbers[0]);

    // Method 2
    match numbers.get(2) {
        Some(n) => println!("get(2): {}", n),
        None => println!("index out of bounds"),
    }

    match numbers.get(99) {
        Some(n) => println!("get(99): {}", n),
        none => println!("get(99): index does not exists"),
    }

    println!("first: {:?}", numbers.first());
    println!("last: {:?}", numbers.last());

    // PART 4: Useful Vec Methods

    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];

    println!("len: {}", v.len());
    println!("is_empty: {}", v.is_empty());

    //contains
    println!("contains 5 : {}", v.contains(&5));
    println!("contains 7: {}", v.contains(&7));

    v.sort();
    println!("sorted array : {:?}", v);

    v.reverse();
    println!("reversed array: {:?}", v);

    // dedup => removed consecutive duplicated (sort first)
    v.sort();
    v.dedup();
    println!("deduped: {:?}", v); // [1,2,3,4,5,6,9]

    let mut temp = vec![1, 2, 3];
    temp.clear();
    println!("after clear: {:?}", temp);

    // PART 5: looping over a vec

    let colors = vec!["red", "green", "blue"];

    for i in &colors {
        // colors will still valid becuz we borrowed
        println!("color : {}", i);
    }

    // loop with mut - modify each item
    let mut prices = vec![20, 40, 60];
    for price in &mut prices {
        *price *= 2; // * -> dereferences -- needed to modify
    }

    println!("doubled prices: {:?}", prices);

    // PART 6 : Vec of Structs  -- IMPORTANT

    #[derive(Debug)]
    struct Student {
        name: String,
        grade: u32,
    }

    let students = vec![
        Student {
            name: String::from("Alice"),
            grade: 90,
        },
        Student {
            name: String::from("in"),
            grade: 55,
        },
        Student {
            name: String::from("wonderland"),
            grade: 87,
        },
    ];

    for i in &students {
        println!("{} scored {}", i.name, i.grade);
    }

    // PART 7: Slices -- a window into a Vec
    let nums = vec![1, 2, 3, 4, 5];

    // A slice is a reference to a position of a vec
    let slice = &nums[1..4];
    println!("slice: {:?}", slice);

    let all = &nums[..];
    println!("all: {:?}", all);

    // PART 8: Useful Patterns

    let numbers = vec![1, 2, 3, 4, 5];

    let sum: i32 = numbers.iter().sum();
    println!("sum: {}", sum);

    println!("max: {:?}", numbers.iter().max());
    println!("min: {:?}", numbers.iter().min());

    // count items matching a condition
    let even = numbers.iter().filter(|&x| x % 2 == 0).count();
    println!("even count: {}", even);

    // check if any item matches
    let has_big = numbers.iter().any(|&x| x > 4);
    println!("any > 4: {}", has_big);

    // check if all items match
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("all positive: {}", all_positive);
}
