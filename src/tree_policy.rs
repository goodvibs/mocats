//! Contains the trait that defines a tree policy, as well as a preset implementation (UctPolicy).

use crate::game::{GameAction, Player};
use crate::search_node::SearchNode;

/// A trait that defines a tree policy.
pub trait TreePolicy<A, Pl>: Sized where A: GameAction, Pl: Player {
    /// Selects a child node to explore.
    fn select_child<'a>(&self, node: &'a mut SearchNode<A, Pl>, is_max_player_turn: bool) -> &'a mut SearchNode<A, Pl>;
}

/// A preset implementation of a tree policy, using the UCT formula.
pub struct UctPolicy {
    /// The exploration constant to use.
    pub exploration_constant: f32
}

impl UctPolicy {
    pub fn new(exploration_constant: f32) -> Self {
        assert!(exploration_constant > 0.0, "Exploration constant must be positive");
        Self {
            exploration_constant: exploration_constant
        }
    }
}

impl<A, Pl> TreePolicy<A, Pl> for UctPolicy where A: GameAction, Pl: Player {
    fn select_child<'a>(&self, node: &'a mut SearchNode<A, Pl>, is_root_player_turn: bool) -> &'a mut SearchNode<A, Pl> {
        match is_root_player_turn {
            true => {
                let mut highest_ucb: f32 = f32::NEG_INFINITY;
                let mut best_child : Option<&'a mut SearchNode<A, Pl>> = None;
                let parent_visits_ln = (node.visits as f32).ln();
                for child in node.children.iter_mut() {
                    if child.visits == 0 {
                        return child;
                    }
                    let child_ucb = child.total_value / child.visits as f32 + self.exploration_constant*(parent_visits_ln/child.visits as f32).sqrt();
                    if child_ucb > highest_ucb {
                        highest_ucb = child_ucb;
                        best_child = Some(child);
                    }
                }
                best_child.expect("No best child found")
            }
            false => {
                let mut lowest_ucb: f32 = f32::INFINITY;
                let mut best_child : Option<&'a mut SearchNode<A, Pl>> = None;
                let parent_visits_ln = (node.visits as f32).ln();
                for child in node.children.iter_mut() {
                    if child.visits == 0 {
                        return child;
                    }
                    let child_ucb = child.total_value / child.visits as f32 - self.exploration_constant*(parent_visits_ln/child.visits as f32).sqrt();
                    if child_ucb < lowest_ucb {
                        lowest_ucb = child_ucb;
                        best_child = Some(child);
                    }
                }
                best_child.expect("No best child found")
            }
        }
    }
}