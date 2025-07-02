// Using W3 schools

fn main() {
    println!("Hello, world!");
    print!("This is a simple Rust program.\n");
    print!(
        "Not starting a new line... Opps check the code again there's a  that's why there's a new line."
    );
    println!("\nLet's calculate the sum of two numbers:");
    calculate_sum();
    constants_();
    match_();
    add();
    borrowing();

    data_structure();

    using_struct();

    _enum();
}

fn calculate_sum() {
    // 'mut' allows the variable to be mutable
    // 'let' is used to declare a variable
    // 'x' is mutable and can be changed later
    let mut x = 5;
    // 'y' is immutable and cannot be changed after declaration
    let y = 10;
    x = 20;
    let sum = x + y;
    println!("The sum of {} and {} is {}", x, y, sum);
}

/// This function prints out the maximum number of points allowed
/// and the result of the && and || operators
fn constants_() {
    // Constants are immutable and must have a type annotation
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are {}", MAX_POINTS);

    // Using of && and || operators
    let a = true;
    let b = false;
    let c = a && b; // Logical AND
    let d = a || b; // Logical OR
    println!("a && b = {}, a || b = {}", c, d);

    let age = 15;
    let can_vote = age >= 18;

    println!("Can vote? {}", can_vote);

    if 120 < 102 {
        println!("90 is less than 102");
    } else if 100 < 102 {
        println!("100 is less than 102");
    } else {
        println!("90 is greater than 102");
    }
}

fn match_() {
    let day = 60;
    // match day {
    //     1 => println!("Monday"),
    //     2 => println!("Tuesday"),
    //     3 => println!("Wednesday"),
    //     4 => println!("Thursday"),
    //     5 => println!("Friday"),
    //     6 => println!("Saturday"),
    //     7 => println!("Sunday"),
    //     _ => println!("Invalid day."),
    // }

    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday!"),
        6 | 7 => println!("Weekend!"),
        _ => println!("Invalid day"),
    }

    loop {
        println!("Looping");
        break;
    }

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    while count < 10 {
        println!("count = {}", count);
        count += 1;
    }
    println!("End count = {}", count);

    let mut num = 1;

    while num <= 10 {
        if num == 6 {
            num += 1;
            continue;
        }

        println!("Number: {}", num);
        num += 1;
    }

    // for i in 1..6 {
    for i in 1..=6 {
        println!("i is: {}", i);
    }
}

fn add() {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let sum = add(3, 4);
    println!("Sum is: {}", sum);
}

fn borrowing() {
    let a = String::from("Hello");
    let b = &a;

    println!("a = {}", a);
    println!("b = {}", b);
}

fn data_structure() {
    // array

    // An array in Rust is a fixed-size list of values, all of the same type.

    // You cannot grow or shrink an array after it's created.

    // To access an array element, refer to its index number.

    // Array indexes start with 0: [0] is the first element, [1] is the second element, etc.

    let mut fruits = ["banana", "mango", "pineapple", "egg"];
    println!("{} is not a fruit", fruits[3].to_uppercase());
    fruits[3] = "apple";
    println!(
        "{} is a fruit, all fixed now 🫠, and the length of the array is {}",
        fruits[3],
        fruits.len()
    );
    println!("This is all of the array {:?}", fruits);

    // vector

    // A vector is a resizable array. Unlike regular arrays, vectors can grow or shrink in size.

    let mut universities = vec!["Uniben", "Unilag", "Lasu", "Unizik", "Uniuyo"];
    universities.push("Uni-Ibadan");

    println!("These is a list of universities in Nigeria ");

    let mut count = 1;

    for university in &universities {
        println!("{}. {}", count, university);
        count += 1;
    }

    universities.remove(4);
    println!("{:?} is the new list", universities);

    // Tuple
    // A tuple is a collection of values of different types.
    let person = ("Stark", 50, true);

    println!("Name: {} ", person.0);
    println!("Age: {} ", person.1);
    println!("Is Alive: {}", person.2);

    // Hash Map, it is similar to dictionary in python

    // A HashMap stores key-value pairs. It lets you look up a value using a key.

    // To use HashMap, you must import it from the standard library.

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert("Stark", 98);
    scores.insert("Scot", 99);
    scores.insert("Sammy", 90);

    println!("Stark's score is: {}", scores["Stark"]);
}

fn using_struct() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user1 = User {
        username: String::from("Stark"),
        email: String::from("vXxh7@example.com"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("stark@example.com");

    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Sign in count: {}", user1.sign_in_count);
    println!("Active: {}", user1.active);
}

    // An enum (short for "enumeration") is a way to define a type that can be one of a few     different values.

    // Each value in the enum is called a variant.

    // Enums are useful when you want to represent a value that can only be one of a set of options - like days of the week, directions, or results like success and error.
    enum Direction {
        Up,
        Down,
        Left,
        Right,
      }
fn _enum() {


    let my_direction = Direction::Left;

    match my_direction {
        Direction::Up => println!("Going up"),
        Direction::Down => println!("Going down"),
        Direction::Left => println!("Going left"),
        Direction::Right => println!("Going right"),
}
}