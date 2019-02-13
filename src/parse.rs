pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Parsed {
    Num(f64),
    Operator(Operator),
}

pub type CommandStack = std::vec::Vec<Parsed>;
pub type Stack = std::vec::Vec<f64>;

pub fn parse(expr: &str) -> Result<CommandStack, String> {
    expr.split_whitespace()
        .map(|elem| match elem {
            "+" => Ok(Parsed::Operator(Operator::Add)),
            "-" => Ok(Parsed::Operator(Operator::Sub)),
            "*" => Ok(Parsed::Operator(Operator::Mul)),
            "/" => Ok(Parsed::Operator(Operator::Div)),
            other => match other.parse::<f64>() {
                Ok(val) => Ok(Parsed::Num(val)),
                Err(_) => Err(format!("Cannot parse \"{}\"", other)),
            },
        })
        .into_iter()
        .collect()
}

pub fn eval(commands: CommandStack, stack: &mut Stack) -> Result<(), String> {
    for command in commands {
        match command {
            Parsed::Num(num) => stack.push(num),
            Parsed::Operator(op) => {
                if stack.len() < 2 {
                    return Err(format!("Too few arguments on stack"));
                } else {
                    // Did implicit check before
                    // so the values should be correct
                    let arg2 = stack.pop().unwrap();
                    let arg1 = stack.pop().unwrap();
                    match op {
                        Operator::Add => stack.push(arg1 + arg2),
                        Operator::Sub => stack.push(arg1 - arg2),
                        Operator::Mul => stack.push(arg1 * arg2),
                        Operator::Div => stack.push(arg1 / arg2),
                    }
                }
            }
        }
    }
    Ok(())
}
