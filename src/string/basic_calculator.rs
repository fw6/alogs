pub fn calculate(s: String) -> i32 {
    let mut op_sign = 1;
    let mut op_stack = vec![1];

    let mut res = 0_i32;

    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '+' => {
                op_sign = op_stack.last().unwrap().clone();
            }
            '-' => {
                op_sign = -op_stack.last().unwrap().clone();
            }
            '(' => {
                op_stack.push(op_sign);
            }
            ')' => {
                op_stack.pop();
            }
            '0'..='9' => {
                let mut num = c.to_digit(10).unwrap() as i32;
                while let Some(c) = chars.peek() {
                    match c {
                        '0'..='9' => {
                            num = num * 10 + c.to_digit(10).unwrap() as i32;
                            chars.next();
                        }
                        _ => {
                            break;
                        }
                    }
                }

                res += op_sign * num;
            }
            _ => {}
        }
    }

    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_calculator_it_works() {
        use super::calculate;
        assert_eq!(calculate("1 + 1".to_string()), 2);
        assert_eq!(calculate(" 2-1 + 2 ".to_string()), 3);
        assert_eq!(calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
        assert_eq!(calculate("1-(     -2)".to_string()), 3);
        assert_eq!(calculate("1+(     -2)+-3-(-2)".to_string()), -2);
        assert_eq!(calculate("-2+ 1".to_string()), -1);
    }
}
