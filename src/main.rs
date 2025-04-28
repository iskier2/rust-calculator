use std::io;

fn main() {
    println!("Simple calculator in Rust!");
    loop{
        let mut operation = String::new();
        loop {
            println!("Choose an operation: [+, -, *, /]");
            io::stdin().read_line(&mut operation).expect("Error while reading!");
            operation = operation.trim().to_string();

            if operation == "+" || operation == "-" || operation == "*" || operation == "/" {
                break;
            } else {
                println!("Invalid operation! Please choose again.");
            }
        }

        match operation.as_str() {
            "+" => add(),
            "-" => subtract(),
            "*" => multiply(),
            "/" => divide(),
            _ => unreachable!(), 
        }

        println!("Do you want to perform another calculation? (y/N)");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Error while reading!");
        if answer.trim().to_lowercase() != "y" {
            println!("Thank you for using the calculator!");
            break;
        }
    }
}

fn get_number(prompt: &str) -> f64 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Error while reading!");
        let trimmed_input = input.trim();
        match trimmed_input.parse::<f64>() {
            Ok(number) => return number,
            Err(_) => {
                println!("Invalid input! Please enter a valid number.");
            }
        }
    }
}

fn add() {
    let a = get_number("Enter first number:");
    let b = get_number("Enter second number:");
    let result = a + b;
    println!("Result: {}", result);
}

fn subtract() {
    let a = get_number("Enter first number:");
    let b = get_number("Enter second number:");
    let result = a - b;
    println!("Result: {}", result);
}

fn multiply() {
    let a = get_number("Enter first number:");
    let b = get_number("Enter second number:");
    let result = a * b;
    println!("Result: {}", result);
}

fn divide() {
    let a = get_number("Enter first number:");
    let b = get_number("Enter second number:");
    if b == 0.0 {
        println!("Cannot divide by zero!");
        return;
    }
    let result = a / b;
    println!("Result: {}", result);
}
