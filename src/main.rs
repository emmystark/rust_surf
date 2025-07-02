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

    array_();
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


fn array_() {
    let fruits = ["banana", "mango", "pineapple", "egg"];

    println!("{} is not a fruit", fruits[3].to_uppercase());
}
