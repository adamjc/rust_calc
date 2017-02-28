use std::env;

fn multiply () {
    
}

fn main () {
    let args: Vec<_> = env::args().collect();
    let tokens: Vec<&str> = args[1].split(" ").collect();
    let mut output: Vec<i32> = vec![];
    let mut operator: Vec<_> = vec![];

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
