use core::fmt;

use crate::creature;
use creature::status_effect;

pub struct TurnOrder {
    creatures: Vec<creature::Creature>,
    count: usize,
    // This is 0 based but will be displayed as 1 based
    current_turn: usize
}

impl TurnOrder {

    pub fn new() -> TurnOrder {
        TurnOrder {
            creatures: Vec::new(),
            count: 0,
            current_turn: 0
        }
    }

    pub fn add_creature(&mut self, name: String, initiative: isize) {
        self.creatures.push( creature::Creature::new(name, initiative))
    }

    // creature_num is 0 based indexing
    pub fn add_status_effect(&mut self, creature_num: usize, effect_name: String) -> Result<String, String> {
        match self.creatures.get_mut(creature_num) {
            Some(creature) => {
                creature.add_status_effect(effect_name);
                Ok(String::from("Ok"))
            }
            None => Err(String::from("Index out of bounds"))
        }
    }

    pub fn add_status_effect_timed(&mut self, creature_num: usize, effect_name: String, duration: usize, clear_type: status_effect::ClearType) -> Result<String, String> {
        match self.creatures.get_mut(creature_num) {
            Some(creature) => {
                creature.add_status_effect_timed(effect_name, duration, clear_type);
                Ok(String::from("Ok"))
            }
            None => Err(String::from("Index out of bounds"))
        }
    }

    // TODO
    // pub fn next_turn() ->
}

// TODO: NEED TO DO STATUS EFFECTS
impl fmt::Display for TurnOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut turn_order = String::new();

        let mut creature_num: usize = 1;

        for creature in &self.creatures {
            let initiative = creature.initiative();
            let name = creature.name();

            if creature_num == self.current_turn {
                turn_order.push_str(&format!("[{creature_num}] [CURRENT_TURN] I:{initiative} {name}\n"))
            }

            else {
                turn_order.push_str(&format!("[{creature_num}] I:{initiative} {name}\n"))
            }

            creature_num += 1;
        }

        write!(f, "{turn_order}")
    }
}


