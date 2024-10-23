pub mod status_effect;
use status_effect::{StatusEffect, TurnsLeft, ClearType, DurationStatus};


pub struct Creature {
    name: String,
    initiative: isize,
    status_effects: Vec<status_effect::StatusEffect>,
    next_effect_id: usize
}

pub enum CreatureUpdate {
    Updates(Vec<String>),
    NoUpdate
}

impl Creature {
    pub fn new(name: String, initiative: isize) -> Creature {
        Creature {
            name, initiative, status_effects: Vec::new(), next_effect_id: 0
        }
    }

    // Public facing status effect adders, just requires name and an optional time limit
    pub fn add_status_effect(&mut self, name: String) {
        let effect = StatusEffect::builder(self.get_status_id(), name, status_effect::TurnsLeft::Indefinite).build();

        self.add_status_effect_to_list(effect);
    }

    pub fn add_status_effect_timed(&mut self, name: String, turn_duration: usize, clear_type: ClearType) {
        let duration = TurnsLeft::Finite(turn_duration);
        let effect = StatusEffect::builder(
            self.get_status_id(),
            name,
            duration
        ).clear_type(clear_type)
            .build();

        self.add_status_effect_to_list(effect);
    }

    pub fn begin_turn(&mut self) -> CreatureUpdate {
        // List of indices of items to remove from the status effects Vec
        let mut effects_to_remove: Vec<usize> = Vec::new();


        for effect in &mut self.status_effects {
            match effect.begin_turn() {
                DurationStatus::NonExpired => (),
                DurationStatus::Expired => {
                    effects_to_remove.push(effect.id());
                }
            }
        }


        self.return_creature_updates(&effects_to_remove)
    }

    pub fn end_turn(&mut self) -> CreatureUpdate {
        let mut effects_to_remove: Vec<usize> = Vec::new();

        for effect in &mut self.status_effects {
            match effect.end_turn() {
                DurationStatus::NonExpired => (),
                DurationStatus::Expired => {
                    effects_to_remove.push(effect.id());
                }
            }
        }

        self.return_creature_updates(&effects_to_remove)
    }

    pub fn initiative(&self) -> isize {
        self.initiative
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn return_creature_updates(&mut self, effects_to_remove: &Vec<usize>) -> CreatureUpdate {
        let mut updates: Vec<String> = Vec::new();

        for effect_id in effects_to_remove {
            if let Some(index) = self.status_effects.iter().position(|effect| effect.id() == *effect_id) {

                match self.status_effects.get(index) {
                    Some(effect) => {
                        let name = effect.name().to_owned();
                        updates.push(format!("Status effect {name} has expired."));
                        self.status_effects.remove(index);
                    }
                    None => panic!("This should never happen")
                }
            }
        }

        match updates.len() {
            0 => CreatureUpdate::NoUpdate,
            _ => CreatureUpdate::Updates(updates)
        }
    }

    fn get_status_id(&mut self) -> usize {
        self.next_effect_id += 1;

        self.next_effect_id - 1
    }

    fn add_status_effect_to_list(&mut self, effect: status_effect::StatusEffect) {
        self.status_effects.push(effect);
    }
}