use std::env;
use std::collections::HashMap;
#[allow(dead_code)]
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
                if let Some(o1_char) = token.chars().last() {
                    if let Some(o1) = operators.get(&o1_char) {
                        while !operator_stack.is_empty() {
                            if let Some(o2_char) = operator_stack.pop() {
                                if let Some(o2) = operators.get(&o2_char) {
                                    if o1.precedence >= o2.precedence {
                                        output.push(o2_char.to_string());
                                    } else {
                                        operator_stack.push(o2_char);
                                        break;
                                    }
                                }
                            }
                        }

                        operator_stack.push(o1_char);
                    }
                }
            }
        }
    }

    while !operator_stack.is_empty() {
        let op = operator_stack.pop().unwrap().to_string();
        output.push(op);
    }

    println!("{:?}", output);
}
