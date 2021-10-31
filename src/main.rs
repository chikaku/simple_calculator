use simple_calculator::VM;
use std::io;

const HELP: &'static str = "\
:help    show help
:quit    quit calculator
:debug   open debug
:nodebug clsoe debug\
";

fn main() -> Result<(), io::Error> {
    println!("simple calculator like bc in Rust");
    println!("input :help for help information");

    let mut input = String::new();
    let mut vm = VM::new(true);

    loop {
        input.clear();
        io::stdin().read_line(&mut input)?;
        match input.trim() {
            "" => println!("input some arithmetic expression"),
            ":help" => println!("{}", HELP),
            ":quit" => return Ok(()),
            ":debug" => vm.set_debug(true),
            ":nodebug" => vm.set_debug(false),

            _ => match vm.exec_line(input.trim()) {
                Ok(res) => println!("{}", res),
                Err(err) => println!("{}", err),
            },
        }
    }
}
