use std::env;
use std::collections::HashMap;

struct Operator {
    precedence: i32,
    func: Box<Fn(f32, f32) -> f32>
}

fn get_operators () -> HashMap<char, Operator> {
    let multiply = Operator {
        precedence: 3,
        func: Box::new(move |a: f32, b:f32| a * b)
    };

    let divide = Operator {
        precedence: 3,
        func: Box::new(move |a: f32, b:f32| a / b)
    };

    let add = Operator {
        precedence: 4,
        func: Box::new(move |a: f32, b:f32| a + b)
    };

    let subtract = Operator {
        precedence: 4,
        func: Box::new(move |a: f32, b:f32| a - b)
    };

    let mut operators = HashMap::new();

    operators.insert('*', multiply);
    operators.insert('/', divide);
    operators.insert('+', add);
    operators.insert('-', subtract);

    operators
}

fn main () {
    let args: Vec<_> = env::args().collect();
    let tokens: Vec<&str> = args[1].split(" ").collect();
    let mut operator_stack: Vec<char> = vec![];
    let mut output: Vec<String> = vec![];
    let operators = get_operators();

    for token in tokens {
        let maybe_int = token.parse::<i32>();

        match maybe_int {
            Ok(_) => output.push(token.to_string()),
            Err(_) => {
                let o1_char = token.to_string().pop().unwrap();
                let o1 = operators.get(&o1_char).unwrap();

                while !operator_stack.is_empty() {
                    let o2_char = operator_stack[operator_stack.len() - 1];
                    let o2 = operators.get(&o2_char).unwrap();
                    if o1.precedence >= o2.precedence {
                        let popped_o2_char = operator_stack.pop().unwrap().to_string();
                        output.push(popped_o2_char);
                    } else {
                        break;
                    }
                }

                operator_stack.push(o1_char);
            }
        }
    }

    while !operator_stack.is_empty() {
        let op = operator_stack.pop().unwrap().to_string();
        output.push(op);
    }

    output.reverse();

    let mut stack: Vec<f32> = vec![];
    while !output.is_empty() {        
        let element = output.pop().unwrap();
        let maybe_int = element.parse::<f32>();
        match maybe_int {
            Ok(x) => stack.push(x),
            Err(_) => {
                let op_char = element.to_string().pop().unwrap();
                let op = operators.get(&op_char).unwrap();

                let b: f32 = stack.pop().unwrap();
                let a: f32 = stack.pop().unwrap();
                let r: f32 = (op.func)(a, b);
                stack.push(r);
            }
        }
    }

    println!("{:?}", stack.pop().unwrap());
}
