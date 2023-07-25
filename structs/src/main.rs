#![allow(unused)]

// This derive add the Debug trait to User
#[derive(Debug)]
struct User {
    active: bool,
    email: String,
    user_name: String,
    login_count: u32,
}

// Tuple struct without names for fields
struct Color(i32, i32, i32);

// a struct without fields (like a marker inferfacec)
struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        email: String::from("test@example.ch"),
        user_name: String::from("test"),
        login_count: 1,
    };

    println!(
        "UserName: {}, Email: {}, isActive: {}, loginCount: {}",
        user1.user_name, user1.email, user1.active, user1.login_count
    );
    // use the debug trait to print user (:?)
    println!("user is {:?}", user1);

    // whole struct must be mut. Single fields mut don't work
    // reusing email / user name -> would imply ownership rules for user1 (they are moved from
    // user1 to user2)
    let mut user2 = User {
        email: String::from("test@example.ch"),
        user_name: String::from("test"),
        ..user1
    };
    user2.active = false;

    // init and usage of color tuple struct
    let color = Color(0, 0, 0);
    println!("{} {} {}", color.0, color.1, color.2);

    let user3 = create_user(String::from("user"), String::from("email"));
    println!("{} {}", user3.email, user3.user_name);

    let always_qual = AlwaysEqual;
}

fn create_user(user_name: String, email: String) -> User {
    // use short hand for email / user_name
    User {
        active: true,
        email,
        user_name,
        login_count: 1,
    }
}
