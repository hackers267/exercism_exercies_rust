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

    #[test]
    fn test_intermediate_error_returns_none() {
        let inputs = [
            CalculatorInput::Add,
            CalculatorInput::Value(2),
            CalculatorInput::Value(2),
            CalculatorInput::Multiply,
        ];
        let results = evaluate(&inputs);
        assert_eq!(results, None);
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
    for input in inputs {
        match input {
            CalculatorInput::Value(number) => number_stack.push(*number),
            CalculatorInput::Add => {
                let first = number_stack.pop()?;
                let second = number_stack.pop()?;
                number_stack.push(second + first)
            }
            CalculatorInput::Subtract => {
                let first = number_stack.pop()?;
                let second = number_stack.pop()?;
                number_stack.push(second - first)
            }
            CalculatorInput::Multiply => {
                let first = number_stack.pop()?;
                let second = number_stack.pop()?;
                number_stack.push(second * first)
            }
            CalculatorInput::Divide => {
                let first = number_stack.pop()?;
                let second = number_stack.pop()?;
                number_stack.push(second / first)
            }
        }
    }
    let result = number_stack.pop();
    if number_stack.len() == 0 {
        return result;
    }
    return None;
}
