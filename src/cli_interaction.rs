use crate::turn_order::{self, TurnOrder};
use std::{io, str::FromStr};

enum Command {
    Add,
    Remove,
    Bulk,
    Advance,
    Exit
}

enum CommandResult {
    End,
    Continue
}

enum UserInputResult<T, E> {
    Ok(T),
    Err(E),
    Cancel
}

struct TextBased {
    turn_order: turn_order::TurnOrder
}

impl TextBased {
    pub fn start(&mut self) -> Result<String, String> {
        println!();
        println!(r#"Available commands: ["Add", "Remove", "Bulk", "(N)ext, Status, Exit]"#);
        println!(r#"Commands are not case sensitive, if a command has a letter in paretheses, that is its abbreviation."#);
        println!(r#"Additionally, press enter again after (or during in most cases) commands to cancel."#);
        println!();
    
        loop {
            let turn_order = &self.turn_order;
            println!("\n{turn_order}");
            println!("Enter a command.");

            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("failed to read");
            let command_result = self.process_command(&user_input.trim());
            match command_result {
                CommandResult::Continue => (),
                CommandResult::End => break
            }
        }
    
    
    
        Ok(String::from("Finished Okay!"))
    }
    
    fn process_command(&mut self, user_input: &str) -> CommandResult {
        match user_input.to_lowercase().as_str() {
            "add" => { self.add_creature(); },
            "remove" => self.remove_creature(),
            "bulk" => self.bulk_add(),
            "n" => (),
            "next" => (),
            "status" => (),
            "exit" => { return CommandResult::End },
            _ => ()
        }

        CommandResult::Continue
    }

    fn add_creature(&mut self) -> CommandResult {
        println!("Please enter the name of the creature.");
        let name: String;
        match get_input_from_user::<String>() {
            UserInputResult::Ok(input_name) => name = input_name,
            UserInputResult::Err(e) => {
                println!("Error in name input: {e}");
                return CommandResult::End;
            }
            UserInputResult::Cancel => return CommandResult::End
        }

        println!("Enter the initiative.");
        let initiative: isize;
        match get_input_from_user::<isize>() {
            UserInputResult::Ok(input_init) => initiative = input_init,
            UserInputResult::Err(e) => {
                println!("Error in initiative input: {e}");
                return CommandResult::End;
            }
            UserInputResult::Cancel => return CommandResult::End

        }

        self.turn_order.add_creature(name, initiative);
        CommandResult::Continue
    }

    fn remove_creature(&mut self) {
        println!("Enter the number of the creature.");
        let creature_num: usize;
        match get_input_from_user::<usize>() {
            UserInputResult::Ok(input_num) => creature_num = input_num - 1,
            UserInputResult::Err(e) => {
                println!("Error in creature num input: {e}");
                return;
            }
            UserInputResult::Cancel => return
        }

        self.turn_order.remove_creature(creature_num);
    }

    fn bulk_add(&mut self) {
        loop {
            match self.add_creature() {
                CommandResult::Continue => (),
                CommandResult::End => return
            }
        }
    }
}

/*
    Generic function to capture input from user.
    Trims input before returning it.
*/
fn get_input_from_user<T>() -> UserInputResult<T, String> 
where
    T: FromStr,
    <T as FromStr>::Err: ToString
{
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        io::Result::Ok(_) => (),
        io::Result::Err(e) => return UserInputResult::Err(e.to_string())
    }

    if user_input.trim().is_empty() {
        return UserInputResult::Cancel;
    }

    match user_input.trim().parse::<T>() {
        Ok(user_num) => UserInputResult::Ok(user_num),
        Err(e) => UserInputResult::Err(e.to_string())
    }
}

//fn get_num_from_user() -> Result

pub fn test() -> () {
    let mut turn_order = turn_order::TurnOrder::new();
    turn_order.add_creature(String::from("Jeremy"), 10);

    turn_order.add_status_effect(0, String::from("Poisoned"));
    turn_order.add_status_effect(0, String::from("Weakened"));


    println!("{turn_order}");
}

pub fn start() {
    let mut cli = TextBased { turn_order: TurnOrder::new() };
    cli.start();
}

