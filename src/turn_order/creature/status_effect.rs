pub struct StatusEffect {
    id: usize,
    effect_name: String,
    turns_left: TurnsLeft,
    clear_type: ClearType
}

pub struct StatusEffectBuilder {
    id: usize,
    effect_name: String,
    turns_left: TurnsLeft,
    clear_type: Option<ClearType>
}

pub enum TurnsLeft {
    Indefinite,
    Finite(usize)
}

pub enum DurationStatus {
    Expired,
    NonExpired
}

pub enum ClearType {
    BeginningOfTurn,
    EndOfTurn
}

impl StatusEffect {
    pub fn builder(id: usize, effect_name: String, turns_left: TurnsLeft) -> StatusEffectBuilder {
        StatusEffectBuilder {
            id, effect_name, turns_left, clear_type: None
        }
    }

    pub fn name(&self) -> &str {
        &self.effect_name
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn clear_type(&self) -> &ClearType {
        &self.clear_type
    }

    pub fn turns_left(&self) -> &TurnsLeft {
        &self.turns_left
    }

    /*
        Expectation is that no action will be taken for statuses
        that are cleared on end of turn if beginning of turn, and
        vice versa.
     */
    pub fn begin_turn(&mut self) -> DurationStatus {
        match self.turns_left {
            TurnsLeft::Indefinite => DurationStatus::NonExpired,
            TurnsLeft::Finite(ref mut turns) => { 
                match self.clear_type {
                    ClearType::BeginningOfTurn => {
                        if *turns <= 1 {
                            return DurationStatus::Expired
                        }

                        *turns -= 1;
                        DurationStatus::NonExpired
                    }

                    ClearType::EndOfTurn => DurationStatus::NonExpired
                }
            } 
        }
    }

    pub fn end_turn(&mut self) -> DurationStatus {
        match self.turns_left {
            TurnsLeft::Indefinite => DurationStatus::NonExpired,
            TurnsLeft::Finite(ref mut turns) => {
                match self.clear_type {
                    ClearType::BeginningOfTurn => DurationStatus::NonExpired,

                    ClearType::EndOfTurn => {
                        if *turns <= 1 {
                            return DurationStatus::Expired
                        }

                        *turns -= 1;
                        DurationStatus::NonExpired
                    }
                }
            }
        }
    }
}

impl StatusEffectBuilder {
    pub fn clear_type(mut self, clear_type: ClearType) -> Self {
        self.clear_type = Some(clear_type);
        self
    }

    pub fn build (self) -> StatusEffect {
        StatusEffect {
            id: self.id,
            effect_name: self.effect_name,
            turns_left: self.turns_left,
            clear_type: self.clear_type.unwrap_or_default()
        }
    }
}

impl Default for ClearType {
    fn default() -> Self {
        ClearType::BeginningOfTurn
    }
}