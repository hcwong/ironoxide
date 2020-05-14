use std::io;

fn main() {
    run_fibonacci();
}

fn run_fibonacci() {
    // Use the fact that loops can be evaluated here with the coolass break feature
    let iterations: i32 = loop {
        println!("Input the number of iterations");
        let mut iterations = String::new();

        io::stdin()
            .read_line(&mut iterations)
            .expect("Failed to read");

        // Hmmm is there a better way of doing this??
        let iterations: i32 = match iterations.trim().parse() {
            Ok(num) => num, 
            Err(_) => {
                println!("Positive integers only boss");
                continue;
            },
        };

        if iterations <= 0 {
            println!("Positive integers only boss");
            continue;
        }
        break iterations;
    };

    println!("Fibonacci number is {} for {} iterations", fibonacci(iterations), iterations)
        
}

fn fibonacci(iterations: i32) -> i32 {
    if iterations <= 2 {
        return 1;
    }
    let mut iterations = iterations;
    let mut prev = 1;
    let mut current = 1;
    while iterations > 2 {
        let new_current = prev + current;
        prev = current;
        current = new_current;
        iterations -= 1;
    }
    return current;
}
