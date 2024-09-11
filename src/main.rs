use std::env;
use std::fs::File;
use std::io::{self, stdin, stdout, BufRead, Read, Write};

use self::cmd::proc::execute;
mod cmd;

fn main() {
    let home_dir = env::home_dir().expect(""); //warning
    let path = home_dir.join(".icyshrc"); //
    println!("{}", path.display());
    let mut file = match File::open(&path) {
        Err(_) => panic!(r#"Couldn't open "~/.icyshrc". you should make a it."#),
        Ok(v) => v,
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("reading file error");
    let mut prefix = String::new();
    let mut str_msgs: Vec<&str> = Vec::new();
    let mut codes: Vec<&str> = Vec::new();
    let mut codewait = false;
    let mut strwait = false;
    let mut log_level: u32 = 1;
    let mut varbool = false;
    let mut var = String::new();
    let mut attachwait = false;
    for line in contents.lines() {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let keyword: Option<&str> = Some(words[0]);
        if strwait == true {
            if line.contains(r#"'''"#) {
                strwait = false;
            } else {
                str_msgs.push(line);
            }
        } else if codewait == true {
            if line.contains(r#"'''"#) {
                cmd::proc::proc(codes.clone());
                codewait = false;
            } else {
                codes.push(line);
            }
        } else if words[0].starts_with("//") {
            //comment
        } else if words.len() >= 2 && words[1] == "=" {
            match keyword {
                // Something = ?
                Some("code") | Some("cd") => {
                    if words[2].starts_with(r#"'''"#) {
                        codewait = true; //code block
                    } else {
                        let mut args = String::new();
                        for n in 2..words.len() {
                            match n {
                                2 => args = format!("{}", words[2]),
                                _ => args = format!("{} {}", args, words[n]),
                            }
                        }
                        if varbool == false {
                            cmd::proc::proc(&*args);
                        } else {
                            var = cmd::proc::execute(&*args);
                            varbool = false;
                            attachwait = true;
                        }
                    }
                }
                Some("var") => varbool = true,
                Some("log_level") | Some("ll") => log_level = words[2].parse().unwrap(),
                Some("prefix") | Some("pf") => {
                    let mut word = String::new();
                
                    if words.len() >= 4 {for n in 2..words.len()-1 { word = format!("{} {}",words[n], words[n+1]); }} else {word = words[2].to_string()}
                    if words.len() <= 3 && words[2] == "var" && attachwait == true {
                        prefix = var;
                        attachwait = false;
                        var = String::new();
                    } else if word.starts_with(r#"code(""#) && word.ends_with(r#"")"#) {
                        let args = word.to_string().replace(r#"code(""#, "").replace(r#"")"#, "");
                        let result: &str = &args.to_string();
                        prefix = execute(result.trim());
                    } else {
                        if words.len() >= 4 {
                            prefix = word.to_string();
                        } else {
                            prefix = words[2].to_string();
                        }
                    }
                }
                Some("start_text") | Some("st") => {
                    if words[2].starts_with(r#"'''"#) {
                        strwait = true;
                    }
                }
                Some(str) => println!("error! {} is not a keyword.", str),
                _ => (),
            }
        } else {
            match keyword {
                // Something
                Some(str) => println!("error! {} is not a keyword.", str),
                _ => (),
            }
        }
    }
    for msg in &str_msgs {
        println!("{}", msg);
    }
    loop {
        print!("{}", prefix);
        stdout().flush().unwrap();
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
    }
}
