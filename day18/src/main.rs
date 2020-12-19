fn main() {
    let (p1, p2): (Vec<u64>, Vec<u64>) = include_str!("big")
        .lines()
        .map(|expression| {
            let expression = expression.replace("(", "( ").replace(")", " )");
            let tokens: Vec<&str> = expression.split(' ').collect();

            // for part1 one all the ops on the stack has higher precedence then the current
            let part1 = eval_expression(&tokens, |_, _| true);
            // for part2 "+" has highest precedence
            let part2 = eval_expression(&tokens, |op1, _| op1 == "+");

            (part1, part2)
        })
        .unzip();

    println!("Part 1: {:?}", p1.iter().sum::<u64>());
    println!("Part 2: {:?}", p2.iter().sum::<u64>());
}

fn eval_expression<F>(expression: &[&str], higher_precedence: F) -> u64
where
    // check if left has higher precedence than right
    F: Fn(&str, &str) -> bool,
{
    let mut operators = Vec::new();
    let mut output = Vec::new();

    for token in expression {
        match *token {
            "+" | "*" => {
                // Eval first the ops on the stack that has higher precedence than the current op
                while let Some(op) = operators.pop() {
                    if op == "(" || !higher_precedence(op, token) {
                        operators.push(op);
                        break;
                    }

                    let l = output.pop().unwrap();
                    let r = output.pop().unwrap();

                    output.push(eval(l, r, op));
                }

                operators.push(token)
            }
            "(" => operators.push(token),
            ")" => {
                while let Some(op) = operators.pop() {
                    if op == "(" {
                        break;
                    }

                    let l = output.pop().unwrap();
                    let r = output.pop().unwrap();

                    output.push(eval(l, r, op));
                }
            }

            _ => output.push(token.parse::<u64>().unwrap()),
        }
    }

    while let Some(op) = operators.pop() {
        let l = output.pop().unwrap();
        let r = output.pop().unwrap();

        output.push(eval(l, r, op));
    }

    // there has to be one
    output[0]
}

fn eval(left: u64, right: u64, op: &str) -> u64 {
    match op {
        "+" => left + right,
        "*" => left * right,
        _ => unreachable!("Boom!"),
    }
}
