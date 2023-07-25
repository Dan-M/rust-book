#![allow(unused)]
fn main() {
    println!("Hello, world!");
    void_function();

    // statement
    println!("this is a statement");

    // expresssion
    let a = 5;

    let x = {
        let y = 5;
        let z = y + 1;
        // no semi colon here or it will not return
        z
    };

    let squared = square_if_greater_ten(15);
    println!("{squared}");

    // Ownership in scope of string heap memory
    {
        let s1 = String::from("hello");
        // hello exists once on the heap. s1 and s2 point to the same place
        let s2 = s1;

        // ...but to make sure memory deallocation does not happen twice, s1 is not accessable
        // anymore here.

        // value borrowed here after move. Can not use s1 anymore at this point
        // println!("{}, world!", s1);
        println!("{}, world!", s2);

        // cloning string data - both variables stay valid. hello exists twice on the heap
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);

        // Note that primitive data types live on the stack only and are always copied.
        let x = 5;
        let y = x;
        println!("x = {}, y = {}", x, y);
    }
    {
        let s = String::from("hello"); // s comes into scope

        takes_ownership(s); // s's value moves into the function...
                            // ... and so is no longer valid here

        // Not possible because of "borrow of used value"
        // println!("{s}");

        let x = 5; // x comes into scope

        makes_copy(x); // x would move into the function,
                       // but i32 is Copy, so it's okay to still
                       // use x afterward

        // This works because x is copied to be used by makes_copy
        println!("{x}");

        // using references, the called function does not take Ownership
        let s_ref = String::from("hello");
        does_not_take_ownership(&s_ref);
        // s is still mine
        println!("{s_ref}");

        // variable needs to be mutable as well - otherwise can't be passed as mutable ref
        let mut s_ref = String::from("hello");
        does_allow_mutating_ref(&mut s_ref);

        let mut s = String::from("hello");
        let r1 = &mut s;
        let r2 = &mut s;
        // if this print is uncommented it wont compile because 2 mutable borrows of s
        // Note that if the refs are not used, there is no problem ;-)
        // Same rule applies if r1 would be immutable (&s)
        // println!("{}, {}", r1, r2);

        // Slices
        let mut s = String::from("Hello World");
        let hello_slice = &s[0..5];
        let world_slice = &s[6..11];
        println!("{s} - {hello_slice} | {world_slice}");

        // can not clear the s because there are immutable borrows (slices)
        // s.clear();
    }
}

/*
 * function that does not return
 * */
fn void_function() {
    println!("Does not return anything");
}

/*
 * function that returns something
* */
fn square_if_greater_ten(x: i32) -> i32 {
    if x > 10 {
        x * x
    } else {
        x
    }
}

fn takes_ownership(s: String) {
    println!("{s}");
} // s drop is called here because it goes out of scope. The caller is not allowed to access s
  // anymore

fn makes_copy(x: i32) {
    println!("{x}");
}

fn does_not_take_ownership(s: &String) {
    println!("{s}");
    // cant push on a borrowed reference unless s is declared as mut
    // s.push_str("test");
}

fn does_allow_mutating_ref(s: &mut String) {
    println!("{s}");
    // can push because its explicitely mutable
    s.push_str("test");
}

/*
fn dangle() -> &String {
    // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String
                                   // we return a reference to the String, s
    &s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger & compile error!
*/
