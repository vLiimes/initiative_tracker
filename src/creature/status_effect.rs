pub struct StatusEffect {
    effect_name: String,
    turns_left: TurnsLeft,
}

pub enum TurnsLeft {
    Indefinite,
    Finite(usize)
}

impl StatusEffect {
    pub fn new(effect_name: String, turns_left: TurnsLeft) -> StatusEffect {
        StatusEffect {
            effect_name, turns_left
        }
    }

    pub fn name(&self) -> &str {
        self.effect_name
    }
}