use std::io;

enum Computer {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl Computer {
    fn new(state: &str) -> Option<Computer> {
        let state = state.to_lowercase();
        match state.as_str() {
            "off" => Some(Computer::Off),
            "sleep" => Some(Computer::Sleep),
            "reboot" => Some(Computer::Reboot),
            "shutdown" => Some(Computer::Shutdown),
            "hibernate" => Some(Computer::Hibernate),
            _ => None
        }
     
    }
}

fn print_computer_option(opt: Computer) {
    use Computer::*;
    match opt {
        Off => println!("Shuting down"),
        Sleep => println!("Sleeping mode"),
        Reboot => println!("Reboot mode"),
        Shutdown => println!("Shutdown mode"),
        _ => println!("Option not found")
    }
}

fn main() {
    let mut buffer = String::new();

    let user_input = io::stdin().read_line(&mut buffer);

    if user_input.is_ok() {
        match Computer::new(&buffer) {
            Some(action) => {
                print_computer_option(action)

            }
            None => println!("Invalid option")
        }

    } else {
        println!("Error reading the input")
    }
}
