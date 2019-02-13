mod parse;

fn main() {
    use std::io::{stdin, stdout, Write};
    let mut stack = parse::Stack::new();
    loop {
        let mut input = String::new();
        print!("$ ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter correct string!");

        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }

        if let Some('\t') = input.chars().next_back() {
            input.pop();
        }

        let commands = parse::parse(&input);
        match commands {
            Ok(comm) => {
                let res = parse::eval(comm, &mut stack);
                match res {
                    Ok(_) => println!("len: {}, last: {:?}", stack.len(), stack.last()),
                    Err(err) => println!("{}", err),
                }
            }
            Err(err) => println!("{}", err),
        }
    }
}
