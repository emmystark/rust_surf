fn main() {
    println!("Hello, world!");
    print!("This is a simple Rust program.\n");
    print!(
        "Not starting a new line... Opps check the code again there's a  that's why there's a new line."
    );
    println!("\nLet's calculate the sum of two numbers:");
    calculate_sum();
    constants_();
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

fn constants_() {
    // Constants are immutable and must have a type annotation
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are {}", MAX_POINTS);
}