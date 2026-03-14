// RUN THE CODE - cargo run --example 03_ownership_and_borrowing

use std::io;

fn get_length(s: &String) -> usize {
    s.len()
}

// Takes ownership - s1 is moved in and dropped here
fn takes_ownership(s: String) {
    println!("got ownership of: {}", s);
} // s is dropped here

// Borrows only - caller keeps ownership
fn borrows_only(s: &String) {
    println!("Just borrowing: {}", s);
}

fn main() -> io::Result<()> {
    // PART 1 : Ownership and Moves

    let s1 = String::from("hello");
    let s2 = s1; // ownership moves to s2  & s1 is now invalid

    println!("s2 = {}", s2);

    // PART 2 : Copy Types -- NOTE : small types are copied not moved

    let x = 5;
    let y = x;
    println!("X = {}, y = {}", x, y);

    let is_active = true;
    let also_active = is_active;
    println!("is_active = {}, also_active = {}", is_active, also_active);

    // PART 3: Scope - values are dropped when owner leaves scope

    {
        let s = String::from("I only live here");
        println!("inside scope: {}", s);
    }; // s is dropped here - memory freed automatically

    // println!("s is not valid outside scope {}", s);

    // PART 4: Borrowing with &

    let s1 = String::from("hello Universe");
    let length = get_length(&s1); // lend s1 to the function - we keep ownership
    println!("s1 = {}, length = {}", s1, length);

    // PART 5 : Immutable vs Mutable Borrows

    // Multiple immutable borrows are fine
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("r1 = {}, r2 = {}", r1, r2);

    // One Mutable borrow - let's you change the value
    let mut s2 = String::from("Universe");
    let r3 = &mut s2;
    r3.push_str(" is big");
    println!("r3 = {}", r3);

    /*  NOTE : you CANNOT mix mutable & immutable borrows at the same time
     *
     * let r4 = &s2;
     * let r5 = &mut s2,
     * println!("{} {}", r4, r5);
     */

    // PRAT 6 : Ownership through Functions

    let s1 = String::from("Lo Siento");

    // Without borrowing -- ownership moves INTO the function and s1 is gone after
    takes_ownership(s1);
    // println!("{}", s1);      // printing s1 will show error

    // With borrowing - we keep ownership
    let s2 = String::from("Wilson");
    borrows_only(&s2);
    println!("s2 still valid : {}", s2);
    Ok(())
}
