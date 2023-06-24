use rand::Rng;

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

    pub fn roll_dice(&self) -> isize {
        let mut rng = rand::thread_rng();
        let random_number: isize = rng.gen_range(1..=6);
        random_number
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
