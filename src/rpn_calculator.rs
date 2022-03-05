use std::ops::{Add, Div, Mul, Sub};

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

fn binary_operation(stack: &mut Vec<i32>, f: impl Fn(i32, i32) -> i32) -> Option<i32> {
    stack.pop().and_then(|x| stack.pop().map(|y| f(y, x)))
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    inputs
        .iter()
        .try_fold(vec![], |mut stack, cur| {
            match *cur {
                CalculatorInput::Add => binary_operation(&mut stack, i32::add),
                CalculatorInput::Subtract => binary_operation(&mut stack, i32::sub),
                CalculatorInput::Multiply => binary_operation(&mut stack, i32::mul),
                CalculatorInput::Divide => binary_operation(&mut stack, i32::div),
                CalculatorInput::Value(i) => Some(i),
            }
            .map(|x| {
                stack.push(x);
                stack
            })
        })
        .and_then(|stack| match stack.as_slice() {
            [x] => Some(*x),
            _ => None,
        })
}
