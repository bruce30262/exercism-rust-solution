#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack:Vec<i32> = vec![];

    for i in inputs {
        match i {
            CalculatorInput::Value(x) => stack.push(*x), 
            _ => {
                    if stack.len() < 2 {
                        return None;
                    }

                    let (y, x) = ( stack.pop().unwrap(), stack.pop().unwrap() );
                    match i {
                        CalculatorInput::Add => stack.push(x+y), 
                        CalculatorInput::Subtract => stack.push(x-y), 
                        CalculatorInput::Multiply => stack.push(x*y),
                        CalculatorInput::Divide => stack.push(x/y), 
                        _ => return None,
                    }
            }
        }
    }

    if stack.len() == 1 {
        Some(stack.pop().unwrap())
    } else {
        None
    }
}
