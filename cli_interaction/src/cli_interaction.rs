//use turn_order::turn_order::{self, TurnOrder, creature::status_effect};
use turn_order::turn_order::{TurnOrder, creature::status_effect};
use std::{io, str::FromStr};

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
    turn_order: TurnOrder
}

impl TextBased {
    pub fn start(&mut self) -> Result<(), String> {
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
    
    
    
        Ok(())
    }
    
    fn process_command(&mut self, user_input: &str) -> CommandResult {
        match user_input.to_lowercase().as_str() {
            "add" => { self.add_creature(); },
            "remove" => self.remove_creature(),
            "bulk" => self.bulk_add(),
            "n" => self.next_turn(),
            "next" => self.next_turn(),
            "status" => self.add_status_effect(),
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
            UserInputResult::Cancel => return CommandResult::End,
            UserInputResult::Err(e) => {
                println!("Error in name input: {e}");
                return CommandResult::End;
            }
        }

        println!("Enter the initiative.");
        let initiative: isize;
        match get_input_from_user::<isize>() {
            UserInputResult::Ok(input_init) => initiative = input_init,
            UserInputResult::Cancel => return CommandResult::End,
            UserInputResult::Err(e) => {
                println!("Error in initiative input: {e}");
                return CommandResult::End;
            }

        }

        self.turn_order.add_creature(name, initiative);
        CommandResult::Continue
    }

    fn remove_creature(&mut self) {
        println!("Enter the number of the creature.");
        let creature_num: usize;
        match get_input_from_user::<usize>() {
            UserInputResult::Ok(input_num) => creature_num = input_num - 1,
            UserInputResult::Cancel => return,
            UserInputResult::Err(e) => {
                println!("Error in creature num input: {e}");
                return;
            }
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

    fn next_turn(&mut self) {
        match self.turn_order.next_turn() {
            Ok(updates) => {
                for update in &updates {
                    println!("{update}");
                }
            }
            Err(e) => println!("Error advancing turn: {e}")
        }
    }

    fn add_status_effect(&mut self) {
        println!("Enter the number of the creature.");
        let creature_num: usize;
        match get_input_from_user::<usize>() {
            UserInputResult::Ok(input_num) => {
                creature_num = input_num - 1;
                match self.turn_order.creature_num_valid(creature_num) {
                    Ok(_) => (),
                    Err(e) => println!("Error: invalid creature index.")
                }
            },
            UserInputResult::Cancel => return,
            UserInputResult::Err(e) => {
                println!("Error in creature num input: {e}");
                return;
            }
        }
        println!("Enter the name of the status effect.");
        let effect_name: String;
        match get_input_from_user::<String>() {
            UserInputResult::Ok(name) => effect_name = name,
            UserInputResult::Cancel => return,
            UserInputResult::Err(e) => {
                println!("Error in creature num input: {e}");
                return;
            }
        }

        /*
            Any cancels will be interpreted as default statuses, but
            depending on which point it's at, default values may be 
            used for a more complete status

            Just use basic add status if cancelled at this point
         */

        println!("Enter clear type (\"start\" or \"end\"), or press enter again for indefinite clear");
        let clear_type: status_effect::ClearType;

        loop {
            match get_input_from_user::<String>() {
                UserInputResult::Ok(user_input) => {
                    match user_input.to_lowercase().as_str() {
                        "start" => {
                            clear_type = status_effect::ClearType::BeginningOfTurn;
                            break;
                        }
                        "end" => {
                            clear_type = status_effect::ClearType::EndOfTurn;
                            break;
                        }
                        _ => {
                            println!("Unrecognized clear type. Try again.");
                        }
                    }
                }
                UserInputResult::Cancel => {
                    self.turn_order.add_status_effect(creature_num, effect_name);
                    return;
                }
                UserInputResult::Err(e) => {
                    println!("Error in Clear Type input: {e}");
                    return;
                }
            }
        }

        println!("Enter duration in number of turns, or press enter again for indefinite clear.");
        let duration: usize;
        match get_input_from_user::<usize>() {
            UserInputResult::Ok(user_num) => duration = user_num,
            UserInputResult::Cancel => {
                self.turn_order.add_status_effect(creature_num, effect_name);
                return;
            }
            UserInputResult::Err(e) => {
                println!("Error in duration input: {e}");
                return;
            }
        }

        match self.turn_order.add_status_effect_timed(creature_num, effect_name, duration, clear_type) {
            Ok(_) => (),
            Err(e) => println!("Error adding completed status effect: {e}")
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

pub fn start() {
    let mut cli = TextBased { turn_order: TurnOrder::new() };
    cli.start();
}

