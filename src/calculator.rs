/// A simple math expression evaluator using the shunting-yard algorithm.
/// Supports: +, -, *, /, %, parentheses, unary minus, and sqrt.

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Percent,
    Power,
    LeftParen,
    RightParen,
    Sqrt,
    UnaryMinus,
}

fn precedence(token: &Token) -> u8 {
    match token {
        Token::Plus | Token::Minus => 1,
        Token::Multiply | Token::Divide | Token::Percent => 2,
        Token::Power => 3,
        Token::UnaryMinus | Token::Sqrt => 4,
        _ => 0,
    }
}

fn is_right_associative(token: &Token) -> bool {
    matches!(token, Token::Power | Token::UnaryMinus | Token::Sqrt)
}

fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = expr.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' => {
                i += 1;
            }
            '0'..='9' | '.' => {
                let start = i;
                let mut has_dot = chars[i] == '.';
                i += 1;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    if chars[i] == '.' {
                        if has_dot {
                            return Err("Invalid number: multiple decimal points".into());
                        }
                        has_dot = true;
                    }
                    i += 1;
                }
                let num_str: String = chars[start..i].iter().collect();
                let num = num_str
                    .parse::<f64>()
                    .map_err(|_| format!("Invalid number: {}", num_str))?;
                tokens.push(Token::Number(num));
            }
            '+' => {
                tokens.push(Token::Plus);
                i += 1;
            }
            '-' => {
                // Determine if this is unary minus
                let is_unary = tokens.is_empty()
                    || matches!(
                        tokens.last(),
                        Some(Token::Plus)
                            | Some(Token::Minus)
                            | Some(Token::Multiply)
                            | Some(Token::Divide)
                            | Some(Token::Percent)
                            | Some(Token::Power)
                            | Some(Token::LeftParen)
                            | Some(Token::Sqrt)
                    );
                if is_unary {
                    tokens.push(Token::UnaryMinus);
                } else {
                    tokens.push(Token::Minus);
                }
                i += 1;
            }
            '*' | '\u{00D7}' => {
                tokens.push(Token::Multiply);
                i += 1;
            }
            '/' | '\u{00F7}' => {
                tokens.push(Token::Divide);
                i += 1;
            }
            '%' => {
                tokens.push(Token::Percent);
                i += 1;
            }
            '^' => {
                tokens.push(Token::Power);
                i += 1;
            }
            '(' => {
                tokens.push(Token::LeftParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RightParen);
                i += 1;
            }
            's' if i + 4 <= chars.len() => {
                let word: String = chars[i..i + 4].iter().collect();
                if word == "sqrt" {
                    tokens.push(Token::Sqrt);
                    i += 4;
                } else {
                    return Err(format!("Unknown character: {}", chars[i]));
                }
            }
            c => {
                return Err(format!("Unknown character: {}", c));
            }
        }
    }

    Ok(tokens)
}

fn shunting_yard(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut output: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token),
            Token::Sqrt | Token::UnaryMinus => {
                operator_stack.push(token);
            }
            Token::Plus
            | Token::Minus
            | Token::Multiply
            | Token::Divide
            | Token::Percent
            | Token::Power => {
                while let Some(top) = operator_stack.last() {
                    if *top == Token::LeftParen {
                        break;
                    }
                    let top_prec = precedence(top);
                    let cur_prec = precedence(&token);
                    if top_prec > cur_prec
                        || (top_prec == cur_prec && !is_right_associative(&token))
                    {
                        output.push(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operator_stack.push(token);
            }
            Token::LeftParen => {
                operator_stack.push(token);
            }
            Token::RightParen => {
                let mut found_left_paren = false;
                while let Some(top) = operator_stack.pop() {
                    if top == Token::LeftParen {
                        found_left_paren = true;
                        break;
                    }
                    output.push(top);
                }
                if !found_left_paren {
                    return Err("Mismatched parentheses".into());
                }
                // If there's a function (sqrt) on top, pop it
                if let Some(top) = operator_stack.last() {
                    if matches!(top, Token::Sqrt) {
                        output.push(operator_stack.pop().unwrap());
                    }
                }
            }
        }
    }

    while let Some(op) = operator_stack.pop() {
        if op == Token::LeftParen {
            return Err("Mismatched parentheses".into());
        }
        output.push(op);
    }

    Ok(output)
}

fn evaluate_rpn(rpn: Vec<Token>) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    for token in rpn {
        match token {
            Token::Number(n) => stack.push(n),
            Token::UnaryMinus => {
                let a = stack.pop().ok_or("Invalid expression")?;
                stack.push(-a);
            }
            Token::Sqrt => {
                let a = stack.pop().ok_or("Invalid expression")?;
                if a < 0.0 {
                    return Err("Cannot take square root of negative number".into());
                }
                stack.push(a.sqrt());
            }
            Token::Plus => {
                let b = stack.pop().ok_or("Invalid expression")?;
                let a = stack.pop().ok_or("Invalid expression")?;
                stack.push(a + b);
            }
            Token::Minus => {
                let b = stack.pop().ok_or("Invalid expression")?;
                let a = stack.pop().ok_or("Invalid expression")?;
                stack.push(a - b);
            }
            Token::Multiply => {
                let b = stack.pop().ok_or("Invalid expression")?;
                let a = stack.pop().ok_or("Invalid expression")?;
                stack.push(a * b);
            }
            Token::Divide => {
                let b = stack.pop().ok_or("Invalid expression")?;
                let a = stack.pop().ok_or("Invalid expression")?;
                if b == 0.0 {
                    return Err("Division by zero".into());
                }
                stack.push(a / b);
            }
            Token::Percent => {
                let b = stack.pop().ok_or("Invalid expression")?;
                let a = stack.pop().ok_or("Invalid expression")?;
                stack.push(a % b);
            }
            Token::Power => {
                let b = stack.pop().ok_or("Invalid expression")?;
                let a = stack.pop().ok_or("Invalid expression")?;
                stack.push(a.powf(b));
            }
            _ => return Err("Unexpected token in RPN".into()),
        }
    }

    if stack.len() != 1 {
        return Err("Invalid expression".into());
    }

    Ok(stack[0])
}

/// Evaluate a mathematical expression string and return the result.
pub fn evaluate(expr: &str) -> Result<f64, String> {
    if expr.trim().is_empty() {
        return Err("Empty expression".into());
    }
    let tokens = tokenize(expr)?;
    let rpn = shunting_yard(tokens)?;
    evaluate_rpn(rpn)
}

/// Format a result for display, removing unnecessary trailing zeros.
pub fn format_result(value: f64) -> String {
    if value.is_infinite() {
        return "Infinity".into();
    }
    if value.is_nan() {
        return "Error".into();
    }

    // If it's an integer value, show without decimal
    if value == value.floor() && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        // Show up to 10 significant digits, trim trailing zeros
        let s = format!("{:.10}", value);
        let s = s.trim_end_matches('0');
        let s = s.trim_end_matches('.');
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        assert_eq!(evaluate("2+3").unwrap(), 5.0);
        assert_eq!(evaluate("10-4").unwrap(), 6.0);
        assert_eq!(evaluate("3*7").unwrap(), 21.0);
        assert_eq!(evaluate("20/4").unwrap(), 5.0);
    }

    #[test]
    fn test_precedence() {
        assert_eq!(evaluate("2+3*4").unwrap(), 14.0);
        assert_eq!(evaluate("(2+3)*4").unwrap(), 20.0);
    }

    #[test]
    fn test_unary_minus() {
        assert_eq!(evaluate("-5").unwrap(), -5.0);
        assert_eq!(evaluate("-5+3").unwrap(), -2.0);
        assert_eq!(evaluate("3*-2").unwrap(), -6.0);
    }

    #[test]
    fn test_decimal() {
        let result = evaluate("3.14+1").unwrap();
        assert!((result - 4.14).abs() < 1e-10);
        assert_eq!(evaluate("0.1+0.2").unwrap(), 0.1 + 0.2);
    }

    #[test]
    fn test_power() {
        assert_eq!(evaluate("2^3").unwrap(), 8.0);
        assert_eq!(evaluate("3^2+1").unwrap(), 10.0);
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(evaluate("sqrt(9)").unwrap(), 3.0);
        assert_eq!(evaluate("sqrt(16)+1").unwrap(), 5.0);
    }

    #[test]
    fn test_division_by_zero() {
        assert!(evaluate("1/0").is_err());
    }

    #[test]
    fn test_format_result() {
        assert_eq!(format_result(5.0), "5");
        assert_eq!(format_result(3.14), "3.14");
        assert_eq!(format_result(-7.0), "-7");
    }
}
