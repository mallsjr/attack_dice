use crate::actions::{Action, ActionType};
pub struct Player {
    pub hp: isize,
    pub roll_over: RollOver,
}

impl Player {
    pub fn new() -> Self {
        Player {
            hp: 30,
            roll_over: RollOver::default(),
        }
    }
}

#[derive(Default, Debug)]
pub struct RollOver {
    pub magical: isize,
    pub physical: isize,
}

pub type PlayerRolls = (isize, isize, isize);

pub struct PlayerAction {
    pub action: Action,
    pub action_type: ActionType,
    pub damage: isize,
    pub magical: isize,
    pub physical: isize,
}
