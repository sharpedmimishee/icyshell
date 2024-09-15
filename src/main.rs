use std::io::{stdin, stdout, Write};

use self::tools::matcher::command;

mod tools;

fn main() {
    let prefix: String = String::new();
    loop {
        let mut input = String::new();
        print!("{}", prefix);
        let _ = stdout().flush().unwrap();
        let _ = stdin().read_line(&mut input);
        command(input);
    }
}
