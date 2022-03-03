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
                    let x = number_stack.pop().unwrap() + number_stack.pop().unwrap();
                    number_stack.push(x)
                }
                CalculatorInput::Subtract => {
                    let x = number_stack.pop().unwrap() + number_stack.pop().unwrap();
                    number_stack.push(x)
                }
                CalculatorInput::Divide => {
                    let x = number_stack.pop().unwrap() / number_stack.pop().unwrap();
                    number_stack.push(x)
                }
                CalculatorInput::Multiply => {
                    let x = number_stack.pop().unwrap() * number_stack.pop().unwrap();
                    number_stack.push(x)
                }
            },
            None => (),
        }
    }
    return number_stack.pop();
}
