use learning_rust::read_input;

pub fn calculator() {
    let number1 = match read_input("Enter the first number:") {
        Ok(number) => match number.trim().parse::<f64>() {
            Ok(number) => number,
            Err(e) => {
                eprintln!("Error reading number: {}", e);
                return;
            }
        },
        Err(e) => {
            eprintln!("Error reading number: {}", e);
            return;
        }
    };

    let operation = match read_input("Enter the operation (+, -, *, /, %):") {
        Ok(operation) => operation.trim().to_string(),
        Err(e) => {
            eprintln!("Error reading operation: {}", e);
            return;
        }
    };

    let number2 = match read_input("Enter the second number:") {
        Ok(number) => match number.trim().parse::<f64>() {
            Ok(number) => number,
            Err(e) => {
                eprintln!("Error reading number: {}", e);
                return;
            }
        },
        Err(e) => {
            eprintln!("Error reading number: {}", e);
            return;
        }
    };

    let result: f64 = match operation.as_str() {
        "+" => number1 + number2,
        "-" => number1 - number2,
        "*" => number1 * number2,
        "/" => {
            if number2 == 0.0 {
                eprintln!("Cannot divide by zero");
                return;
            }
            number1 / number2
        }
        "%" => number1 % number2,
        _ => {
            eprintln!("Invalid operation");
            0.0
        }
    };

    println!("The result is: {}", result);
}
