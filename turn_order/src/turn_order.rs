use core::fmt;

pub mod creature;
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
        self.creatures.push( creature::Creature::new(name, initiative));
        self.reorder_creatures();
    }

    /*
        TODO: Add error handling
        Remove the creature using 0-based indexing
     */
    pub fn remove_creature(&mut self, creature_index: usize) {
        self.creatures.remove(creature_index);
        self.reorder_creatures();
    }

    pub fn creatures(&self) -> &Vec<creature::Creature> {
        &self.creatures
    }

    pub fn creature_num_valid(&self, index: usize) -> Result<(), &'static str> {
        if index >= self.creatures.len() {
            return Err("Creature index out of range.");
        }

        Ok(())
    }

    // creature_num is 0 based indexing
    pub fn add_status_effect(&mut self, creature_num: usize, effect_name: String) -> Result<(), &'static str> {
        match self.creatures.get_mut(creature_num) {
            Some(creature) => {
                creature.add_status_effect(effect_name);
                Ok(())
            }
            None => Err("Index out of bounds")
        }
    }

    pub fn add_status_effect_timed(&mut self, creature_num: usize, effect_name: String, duration: usize, clear_type: status_effect::ClearType) -> Result<(), &'static str> {
        match self.creatures.get_mut(creature_num) {
            Some(creature) => {
                creature.add_status_effect_timed(effect_name, duration, clear_type);
                Ok(())
            }
            None => Err("Index out of bounds")
        }
    }

    /*
        If operation Ok, will return a vec of strings that
        represent creature updates.
     */
    pub fn next_turn(&mut self) -> Result<Vec<String>, String> {
        let mut all_updates: Vec<String> = Vec::new();
        // Call end turn on current creature and begin turn on the next
        match self.creatures.get_mut(self.current_turn) {
            Some(creature) => {
                match creature.end_turn() {
                    creature::CreatureUpdate::Updates(ref mut creature_updates) => {
                        all_updates.append(creature_updates);
                    }
                    creature::CreatureUpdate::NoUpdate => ()
                }
            }
            None => {
                let index = self.current_turn;
                return Err(format!("Error advancing turn: no creature found at index {index}"));

            }
        }

        self.increase_turn_counter();

        match self.creatures.get_mut(self.current_turn) {
            Some(creature) => {
                match creature.begin_turn() {
                    creature::CreatureUpdate::Updates(ref mut creature_updates) => {
                        all_updates.append(creature_updates);
                    }
                    creature::CreatureUpdate::NoUpdate => ()
                }
            }
            None => {
                let index = self.current_turn;
                return Err(format!("Error advancing turn: no creature found at index {index}"));
            }
        }


        return Ok(all_updates);
    }

    fn reorder_creatures(&mut self) {
        self.creatures.sort_by(|a, b| b.initiative().cmp(&a.initiative()));
    }

    fn increase_turn_counter(&mut self) {
        self.current_turn += 1;
        if self.current_turn >= self.creatures.len() {
            self.current_turn = 0;
        }
    }

    
}

impl fmt::Display for TurnOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut turn_order_str = String::new();

        let mut creature_num: usize = 1;

        for creature in &self.creatures {
            let initiative = creature.initiative();

            if creature_num - 1 == self.current_turn {
                turn_order_str.push_str(&format!("[{creature_num}] [CURRENT TURN] I:{initiative} {creature}\n"))
            }

            else {
                turn_order_str.push_str(&format!("[{creature_num}] I:{initiative} {creature}\n"))
            }

            creature_num += 1;
        }

        write!(f, "{turn_order_str}")
    }
}


