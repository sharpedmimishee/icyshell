use std::env;
use std::io::stdout;
use std::process::Command;
trait Change {
    // add code here
    fn change(&self) -> ();
}

impl Change for &str {
    fn change(&self) {
        exec(self.to_string());
    }
}
impl Change for Vec<&str> {
    fn change(&self) {
        for codes in self {
            exec(codes.to_string());
        }
    }
}
pub fn proc<T: Change>(codes: T) {
    codes.change();
}

fn exec(code: String) {
    let args: Vec<&str> = code.split_whitespace().collect();
    let main = &args[0].to_string();
    let mut subcommands: Vec<&str> = Vec::new();
    for n in 1..args.len() {
        subcommands.push(args[n]);
    }
    let homedir = env::home_dir().expect("").display().to_string();
    let mut resultmain = String::new();
    if main.contains("~/") {
        resultmain = main.replace("~/", &(homedir.to_string()+"/"));
    } else {
        resultmain = main.replace("~/", &(homedir.to_string()+"/"));
    }
    let process = Command::new(&resultmain)
        .args(subcommands)
        .output()
        .expect("failed to execute process");

    let output = process.stdout;
    println!("{}", std::str::from_utf8(&output).unwrap());
}

pub fn execute(code: &str) -> String {
    let args: Vec<&str> = code.split_whitespace().collect();
    let main = &args[0];
    let mut subcommands: Vec<&str> = Vec::new();
    for n in 1..args.len() {
        subcommands.push(args[n]);
    }
    let homedir = env::home_dir().expect("").display().to_string();
    let mut resultmain = String::new();
    if main.contains("~/") {
        resultmain = main.replace("~/", &(homedir.to_string()+"/"));
    } else {
        resultmain = main.replace("~/", &(homedir.to_string()+"/"));
    }
    let process = Command::new(&resultmain)
        .args(subcommands)
        .output()
        .expect("failed to execute process");

    let output = process.stdout;
    return std::str::from_utf8(&output).unwrap().to_string();
}
