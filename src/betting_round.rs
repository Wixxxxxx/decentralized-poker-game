use crate::action::Action;

pub enum BettingRound {
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}

// Need to modify to correct actions, this is just placeholder for now!
impl BettingRound {
    pub fn available_actions(&self) -> Vec<Action> {
        match self {
            Self::PreFlop => vec![Action::Call, Action::Raise],
            Self::Flop => vec![Action::Call, Action::Raise, Action::Raise, Action::Fold],
            _ => vec![
                Action::Call,
                Action::Raise,
                Action::Check,
                Action::Fold,
                Action::Bet,
            ],
        }
    }
}
