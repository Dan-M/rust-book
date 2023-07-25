#![allow(unused)]

enum IpAddrKind {
    // enum value with data - could also be V4(u8,u8,u8,u8)
    V4(String),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // this match is exhaustive. Try commenting out Dime...
    // match expressions can by assigned (x here)
    let x = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
    x
}

fn smaller_equal_five(coin: Coin) -> bool {
    match coin {
        Coin::Penny => true,
        Coin::Nickel => true,
        // catch all.
        _ => false,
        // other => do_something_with_other(other),
        // _ => (), // do nothing at all
    }
}

// enums can have impls as well
impl IpAddrKind {
    fn route(&self) {}
}

fn main() {
    let home_v4 = "127.0.0.1";
    let home_v6 = "::1";
    let four = IpAddrKind::V4(String::from(home_v4));
    let six = IpAddrKind::V6(String::from(home_v6));

    four.route();
    six.route();

    let penny_in_cents = value_in_cents(Coin::Penny);
    println!("Penny in cents: {}", penny_in_cents);

    let smaller = smaller_equal_five(Coin::Penny);
    let bigger = smaller_equal_five(Coin::Dime);
    println!("{} {}", smaller, bigger);

    // if let syntax:
    // no need to handle the None case here if we don't want to do anything with it.
    let config_val = Some(3);
    if let Some(val) = config_val {
        println!("Config val is defined as {}", val);
    }
}
