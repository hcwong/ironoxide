fn main() {
    let number = 3;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Because blocks are evaluated to the last line of code
    // The different conditional branches have to evaluate to the same type
    // Wow can use break to return the value evaluated in a while loop
    // Completed rustlings chapter on functions
}
