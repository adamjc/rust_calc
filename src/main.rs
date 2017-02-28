use std::env;
use std::collections::HashMap;

struct Operator {
    precedence: i32,
    func: Box<Fn(f32, f32) -> f32>
}

fn main () {
    let args: Vec<_> = env::args().collect();
    let tokens: Vec<&str> = args[1].split(" ").collect();
    let mut output: Vec<i32> = vec![];
    let mut operator: Vec<_> = vec![];

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

    let mut operators: HashMap<char, Operator> = HashMap::new();

    operators.insert('*', multiply);
    operators.insert('/', divide);
    operators.insert('+', add);
    operators.insert('-', subtract);

    for i in 1..tokens.len() {
        let r = tokens[i].parse::<i32>();

        match r {
            Ok(_) => output.push(r.unwrap()),
            Err(_) => {
                if operator.len() > 0 {

                }
                operator.push(&tokens[i])
            }
        }
    }
}
