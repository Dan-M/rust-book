/*
* const
* - can be declared in global scope
* - are immutable
* - require type annotation
* - must be set to a constant expression
**/
const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

fn main() {
    println!("Hello, world!");
    // Variables by default immutable
    let x = 2;
    // x = 5;

    // ... but can be opt in mutable
    let mut y = 6;
    println!("{x} {y}");
    y = 7;
    println!("{x} {y} {THREE_HOURS_IN_SECONDS}");

    /*
     * Shadowing x in scope
     * */
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // Shadowning allows to change the type of a variable
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");
}
