//! Contains the implementation of the MCTS search tree.

use std::fmt;
use std::fmt::Display;

use crate::game::{GameAction, GameState, Player};
use crate::search_node::*;
use crate::tree_policy::TreePolicy;

/// Represents a MCTS search tree.
pub struct SearchTree<S: GameState<A, Pl>, A: GameAction, Pl: Player, Po: TreePolicy<A, Pl>> {
    /// The root node of the search tree.
    root: SearchNode<A, Pl>,
    /// The initial game state.
    root_game_state: S,
    /// The tree policy to use.
    policy: Po,
}

impl<S, A, Pl, Po> SearchTree<S, A, Pl, Po> where S: GameState<A, Pl>, A: GameAction, Pl: Player, Po: TreePolicy<A, Pl> {
    pub fn new(game: S, tree_policy: Po) -> SearchTree<S, A, Pl, Po> {
        SearchTree {
            root: SearchNode {
                action: None,
                children: Vec::new(),
                root_player: game.get_turn(),
                state: NodeState::ExpandableLeaf,
                visits: 0,
                total_value: 0.0
            },
            root_game_state: game,
            policy: tree_policy
        }
    }

    /// Runs the MCTS algorithm for the given number of iterations.
    pub fn run(&mut self, iterations: usize) {
        for _ in 0..iterations {
            self.root.run_iteration(&mut self.root_game_state.clone(), &self.policy);
        }
    }

    /// Returns the best action according to the MCTS algorithm.
    pub fn get_best_action(&mut self) -> Option<A> {
        self.root.children.as_slice().into_iter().reduce(|a, b| if a.visits > b.visits { a } else { b }).map(|n| n.action.expect("Expected node to have action"))
    }
}

impl<S, A, Pl, Po> Display for SearchTree<S, A, Pl, Po> where S: GameState<A, Pl>, A: GameAction, Pl: Player, Po: TreePolicy<A, Pl> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Node count: {}\n{}", self.root.get_node_count(), self.root)
    }
}