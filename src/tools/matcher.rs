use crate::tools::frozem;

pub fn command(command: String) {
    let main: Vec<&str> = command.split_whitespace().collect();
    match main[0] {
        "frozem" => frozem::run(main),
        _ => ()
    }
}
