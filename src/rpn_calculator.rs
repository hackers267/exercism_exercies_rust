#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_inputs() {
        let inputs = [
            CalculatorInput::Value(2),
            CalculatorInput::Value(3),
            CalculatorInput::Add,
        ];
        let result = evaluate(&inputs);
        assert_eq!(result, Option::Some(5));
    }

    #[test]
    fn complex_inputs() {
        let inputs = [
            CalculatorInput::Value(4),
            CalculatorInput::Value(8),
            CalculatorInput::Add,
            CalculatorInput::Value(7),
            CalculatorInput::Value(5),
            CalculatorInput::Subtract,
            CalculatorInput::Divide,
        ];
        let result = evaluate(&inputs);
        assert_eq!(result, Option::Some(6));
    }

    #[test]
    fn test_zero_operands_returns_none() {
        let inputs = [CalculatorInput::Value(3), CalculatorInput::Value(5)];
        let result = evaluate(&inputs);
        assert_eq!(result, None);
    }

    #[test]
    fn test_too_few_operands_returns_none() {
        let inputs = [
            CalculatorInput::Add,
            CalculatorInput::Value(3),
            CalculatorInput::Value(5),
            CalculatorInput::Value(7),
        ];
        let result = evaluate(&inputs);
        assert_eq!(result, None);
    }

    #[test]
    fn test_too_many_operands_return_none() {
        let inputs = [
            CalculatorInput::Value(5),
            CalculatorInput::Value(3),
            CalculatorInput::Multiply,
            CalculatorInput::Add,
        ];
        let result = evaluate(&inputs);
        assert_eq!(result, None);
    }
}

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut number_stack: Vec<i32> = Vec::new();
    let len = inputs.len();
    for i in 0..len {
        let input = inputs.get(i);
        match input {
            Some(in_input) => match in_input {
                CalculatorInput::Value(j) => number_stack.push(*j),
                CalculatorInput::Add => {
                    let first_option = number_stack.pop();
                    let second_option = number_stack.pop();
                    if first_option.is_some() && second_option.is_some() {
                        let x = first_option.unwrap() + second_option.unwrap();
                        number_stack.push(x)
                    }
                }
                CalculatorInput::Subtract => {
                    let first_option = number_stack.pop();
                    let second_option = number_stack.pop();
                    if first_option.is_some() && second_option.is_some() {
                        let second = second_option.unwrap();
                        let first = first_option.unwrap();
                        let x = second - first;
                        number_stack.push(x)
                    }
                }
                CalculatorInput::Divide => {
                    let first_option = number_stack.pop();
                    let second_option = number_stack.pop();
                    if first_option.is_some() && second_option.is_some() {
                        let first = first_option.unwrap();
                        let second = second_option.unwrap();
                        let x = second / first;
                        number_stack.push(x)
                    }
                }
                CalculatorInput::Multiply => {
                    let first_option = number_stack.pop();
                    let second_option = number_stack.pop();
                    if first_option.is_some() && second_option.is_some() {
                        let first = first_option.unwrap();
                        let second = second_option.unwrap();
                        let x = first * second;
                        number_stack.push(x)
                    }
                }
            },
            None => (),
        }
    }
    let result = number_stack.pop();
    if number_stack.len() == 0 {
        return result;
    }
    return None;
}
