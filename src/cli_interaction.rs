use crate::turn_order;
use std::io;

enum Command {
    Add,
    Remove,
    Bulk,
    Advance,
    Exit
}

struct TextBased {
    turn_order: turn_order::TurnOrder
}

impl TextBased {
    pub fn start(&mut self) -> Result<String, String> {
        println!(r#"Available commands: ["Add", "Remove", "Bulk", "(N)ext, Status, Exit]"#);
        println!(r#"Commands are not case sensitive, if a command has a letter in paretheses, that is its abbreviation."#);
        println!();
    
        loop {
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("failed to read");
            let command_type = self.get_command_type(&user_input);
        }
    
    
    
        Ok(String::from("Finished Okay!"))
    }
    
    fn get_command_type(&mut self, user_input: &str) {
        match user_input.to_lowercase().as_str() {
            "add" => (),
            "remove" => (),
            "bulk" => (),
            "n" => (),
            "next" => (),
            "status" => (),
            "exit" => (),
            _ => ()
        }
    }

    fn add_creature(&mut self) {
        println!("Please enter the name of the creature.");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("failed to read");

    }
}


pub fn test() -> () {
    let mut turn_order = turn_order::TurnOrder::new();
    turn_order.add_creature(String::from("Jeremy"), 10);

    turn_order.add_status_effect(0, String::from("Poisoned"));
    turn_order.add_status_effect(0, String::from("Weakened"));


    println!("{turn_order}");
}

