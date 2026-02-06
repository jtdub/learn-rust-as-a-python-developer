use std::io::{self, Write};

fn parse_expression(input: &str) -> Option<(f64, &str, f64)> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() != 3 {
        println!("Usage: <number> <operator> <number>");
        return None;
    }

    let left: f64 = match parts[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number: {}", parts[0]);
            return None;
        }
    };

    let right: f64 = match parts[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number: {}", parts[2]);
            return None;
        }
    };

    Some((left, parts[1], right))
}

fn calculate(left: f64, operator: &str, right: f64) -> Option<f64> {
    match operator {
        "+" => Some(left + right),
        "-" => Some(left - right),
        "*" => Some(left * right),
        "/" => {
            if right == 0.0 {
                println!("Error: Division by zero");
                None
            } else {
                Some(left / right)
            }
        }
        "^" => Some(left.powf(right)),
        "%" => Some(left % right),
        _ => {
            println!("Unknown operator: {operator}");
            println!("Supported operators: + - * / ^ %");
            None
        }
    }
}

fn main() {
    println!("Simple Calculator — type an expression or 'quit' to exit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading input: {e}");
                break;
            }
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "quit" || input == "exit" {
            println!("Goodbye!");
            break;
        }

        if let Some((left, op, right)) = parse_expression(input) {
            if let Some(result) = calculate(left, op, right) {
                // Display as integer if it's a whole number
                if result.fract() == 0.0 && result.abs() < i64::MAX as f64 {
                    println!("= {}", result as i64);
                } else {
                    println!("= {result}");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(calculate(2.0, "+", 3.0), Some(5.0));
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(calculate(10.0, "-", 4.0), Some(6.0));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculate(3.0, "*", 7.0), Some(21.0));
    }

    #[test]
    fn test_division() {
        assert_eq!(calculate(10.0, "/", 4.0), Some(2.5));
    }

    #[test]
    fn test_division_by_zero() {
        assert_eq!(calculate(10.0, "/", 0.0), None);
    }

    #[test]
    fn test_power() {
        assert_eq!(calculate(2.0, "^", 10.0), Some(1024.0));
    }

    #[test]
    fn test_modulo() {
        assert_eq!(calculate(15.0, "%", 4.0), Some(3.0));
    }

    #[test]
    fn test_unknown_operator() {
        assert_eq!(calculate(1.0, "&", 2.0), None);
    }

    #[test]
    fn test_parse_valid() {
        let result = parse_expression("5 + 3");
        assert!(result.is_some());
        let (left, op, right) = result.unwrap();
        assert_eq!(left, 5.0);
        assert_eq!(op, "+");
        assert_eq!(right, 3.0);
    }

    #[test]
    fn test_parse_invalid_number() {
        assert!(parse_expression("abc + 3").is_none());
    }

    #[test]
    fn test_parse_wrong_parts() {
        assert!(parse_expression("5 +").is_none());
    }
}
