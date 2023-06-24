use std::fmt;

#[derive(PartialEq, Eq)]
pub enum ActionType {
    Magical,
    Physical,
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ActionType::Magical => write!(f, "Magical"),
            ActionType::Physical => write!(f, "Physical"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Action {
    Attack,
    Defend,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Attack => write!(f, "Attack"),
            Action::Defend => write!(f, "Defend"),
        }
    }
}
