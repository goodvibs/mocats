use std::fmt::{Debug, Display};


/// Represents a game state.
pub trait GameState<A: GameAction, P: Player> : Clone {
    /// Returns the actions that can be taken from this state.
    fn get_actions(&self) -> Vec<A>;
    /// Applies the given action to this state.
    fn apply_action(&mut self, action: &A);
    /// Returns the player whose turn it is.
    fn get_turn(&self) -> P;
    /// Returns whether the game is over.
    fn get_reward_for_player(&self, player: P) -> f32;
}

/// Represents a legal game action that can be applied to some GameState.
pub trait GameAction: Debug+PartialEq+Copy+Display {}

/// Represents a player in a game. Should be an enum.
pub trait Player: Debug+Eq+Copy {}
