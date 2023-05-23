use std::fmt::{Debug, Display};


pub trait GameState<A: GameAction, P: Player> : Clone {
    fn get_actions(&self) -> Vec<A>;
    fn apply_action(&mut self, action: &A);
    fn get_turn(&self) -> P;
    fn get_reward_for_player(&self, player: P) -> f32;
}

pub trait GameAction: Debug+PartialEq+Copy+Display {}

pub trait Player: Debug+Eq+Copy {}
