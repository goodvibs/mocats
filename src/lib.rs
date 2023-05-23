/// A fast, easy-to-use, generalized Monte Carlo Tree Search library.
/// Works for any game, any number of players, and any tree policy. UctPolicy included as a default.
/// As of the current version, the search is single-threaded.

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