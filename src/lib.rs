//! # mocats
//!
//! A fast, easy-to-use, generalized Monte Carlo Tree Search library.
//! Works for any game, any number of players, and any tree policy (UctPolicy included as a default).
//! As of the current version, the search is single-threaded.
//!
//! ## Usage
//!
//! In the root directory of your project, do
//!
//! ```bash
//! cargo add mocats
//! ```
//!
//! ...or add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! mocats = "0.2.0"
//! ```
//!
//! ### Defining a game
//!
//! To use mocats, you must define a game and a tree policy. A game is defined by three traits:
//!
//! - `GameState`: Represents a game state.
//! - `GameAction`: Represents a legal game action that can be applied to some `GameState`.
//! - `Player`: Represents a player in a game. Should be an enum.
//!
//! A tree policy is defined by one trait:
//!
//! - `TreePolicy`: Represents a tree policy.
//!
//! The `UctPolicy` struct is included as a default tree policy.
//!
//! ### Running the search
//!
//! To run the search, create a `SearchTree` struct with the game and tree policy, then call `run` on it.
//!
//! ```rust
//! use mocats::{tic_tac_toe, UctPolicy};
//!
//! fn foo() {
//!     let game = tic_tac_toe::TicTacToePosition::new();
//!     let tree_policy = UctPolicy::new(2.0);
//!     let mut search_tree = mocats::SearchTree::new(game, tree_policy);
//!     search_tree.run(2000);
//!     let best_action = search_tree.get_best_action();
//!     println!("{}", search_tree);
//!     println!("Best action: {}", best_action.unwrap());
//! }
//! ```
//!
//! # Example
//!
//! ```rust
//! use std::fmt;
//! use std::fmt::{Display, Formatter};
//!
//! #[derive(Debug, Clone, Copy, PartialEq)]
//! pub struct TicTacToeMove {
//!     pub pos: u16
//! }
//!
//! impl mocats::GameAction for TicTacToeMove {}
//!
//! impl Display for TicTacToeMove {
//!     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//!         todo!()
//!     }
//! }
//!
//! #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
//! pub enum TicTacToePlayer {
//!     X,
//!     O
//! }
//!
//! impl TicTacToePlayer {}
//!
//! impl mocats::Player for TicTacToePlayer {}
//!
//! impl Display for TicTacToePlayer {
//!     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//!         todo!()
//!     }
//! }
//!
//! #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
//! pub struct TicTacToePosition {
//!     pub board_x: u16,
//!     pub board_o: u16,
//!     pub turn: TicTacToePlayer,
//! }
//!
//! impl TicTacToePosition {
//!     pub fn new() -> TicTacToePosition {
//!         todo!()
//!     }
//! }
//!
//! impl Display for TicTacToePosition {
//!     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//!         todo!()
//!     }
//! }
//!
//! impl mocats::GameState<TicTacToeMove, TicTacToePlayer> for TicTacToePosition {
//!     fn get_actions(&self) -> Vec<TicTacToeMove> {
//!         todo!()
//!     }
//!
//!     fn apply_action(&mut self, action: &TicTacToeMove) {
//!         todo!()
//!     }
//!
//!     fn get_turn(&self) -> TicTacToePlayer {
//!         todo!()
//!     }
//!
//!     fn get_reward_for_player(&self, player: TicTacToePlayer) -> f32 {
//!         todo!()
//!     }
//! }
//! ```
//!
//! See `mocats::tic_tac_toe` for a full example.
//! You can also import the `tic_tac_toe` to use it in your code.
//!
//! ## License
//!
//! MIT
//!
//! ## Contributing
//!
//! Pull requests are welcome!

mod game;
mod search_node;
mod search_tree;
mod tree_policy;
pub mod tic_tac_toe;

#[cfg(test)]
mod tests;

pub use game::*;
pub use search_node::*;
pub use search_tree::*;
pub use tree_policy::*;