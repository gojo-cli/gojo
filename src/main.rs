mod commands;
mod packages;
mod plaintext;
mod templates;

// TODO
// * how to edit CMakeLists.txt file from rust?

fn main() {
    let args: std::vec::Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("{}", plaintext::WIN);
        commands::help();
        return;
    }

    let command = args[1].as_str();
    let mut command_args: &[String] = &[];
    if args.len() > 2 {
        command_args = args[2..].iter().as_slice();
    }

    let mut result: std::io::Result<()> = std::io::Result::Ok(());

    match command {
        "init" => {
            result = commands::init(command_args);
        }
        "build" => {
            result = commands::build(command_args);
        }
        "run" => {
            result = commands::run(command_args);
        }
        "test" => {
            result = commands::test();
        }
        "clean" => {
            result = commands::clean();
        }
        "fmt" => {
            result = commands::fmt(command_args);
        }
        "lint" => {
            println!("\x1b[31mincorrect usage:\x1b[0m \n\tcommand not recognized: {command}");
        }
        "check" => {
            result = commands::check();
        }
        "branch" => {
            result = commands::branch(command_args);
        }
        "install" => {
            result = commands::install(command_args);
        }
        "uninstall" => {
            println!("\x1b[31mincorrect usage:\x1b[0m \n\tcommand not recognized: {command}");
        }
        "add" => {
            // class or header
            println!("\x1b[31mincorrect usage:\x1b[0m \n\tcommand not recognized: {command}");
        }
        "help" | "--help" | "-h" => {
            commands::help();
        }
        _ => {
            println!("\x1b[31mincorrect usage:\x1b[0m \n\tcommand not recognized: {command}");
        }
    }
    if result.is_err() {
        eprintln!("{}", result.unwrap_err())
    }
}
