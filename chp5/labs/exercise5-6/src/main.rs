enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn str_to_operation(s: &str) -> Operation {
    match s.to_lowercase().as_str() {
        "add" => Operation::Add,
        "subtract" => Operation::Subtract,
        "multiply" => Operation::Multiply,
        "divide" => Operation::Divide,
        _ => {
            println!("Unknown operation: {}. Defaulting to addition.", s);
            Operation::Add
        }
    }
}

fn calculate(op: Operation, a: i32, b: i32) -> i32 {
    match op {
        Operation::Add => a + b,
        Operation::Subtract => a - b,
        Operation::Multiply => a * b,
        Operation::Divide => {
            if b != 0 {
                a / b
            } else {
                println!("Cannot divide by zero! Defaulting to 0.");
                0
            }
        }
    }
}

fn main() {
   let operation_str = "add";
    let x = 5;
    let y = 3;

    println!("Result of {}ing {} and {}: {}", operation_str, x, y, calculate(str_to_operation(operation_str), x, y));

    let operation_str = "subtract";
    let x = 5;
    let y = 3;
    let op = str_to_operation(operation_str);
    let result = calculate(op, x, y);
    println!("Result of {}ing {} and {}: {}", operation_str, x, y, result);
}
