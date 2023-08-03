fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_lifetime(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// This fails to compile because of missing lifetime spec
// longest return type contains a borrowed value but it's not clear if x or y
/*
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

// Here we specify that all arguments and the return type have the same lifetime
fn longest_with_lifetime<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
