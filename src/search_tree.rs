use std::fmt;
use std::fmt::Display;

use crate::game::{GameAction, GameState, Player};
use crate::search_node::*;
use crate::tree_policy::TreePolicy;

pub struct SearchTree<S: GameState<A, Pl>, A: GameAction, Pl: Player, Po: TreePolicy<A, Pl>> {
    root: SearchNode<A, Pl>,
    root_game_state: S,
    policy: Po,
}

impl<S: GameState<A, Pl>, A: GameAction, Pl: Player, Po: TreePolicy<A, Pl>> SearchTree<S, A, Pl, Po> {
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

    pub fn run(&mut self, iterations: usize) {
        for _ in 0..iterations {
            self.root.iteration(&mut self.root_game_state.clone(), &self.policy);
        }
    }

    pub fn get_best_action(&mut self) -> Option<A> {
        self.root.children.as_slice().into_iter().reduce(|a, b| if a.visits > b.visits { a } else { b }).map(|n| n.action.expect("Expected node to have action"))
    }
}

impl<S: GameState<A, Pl>, A: GameAction, Pl: Player, Po: TreePolicy<A, Pl>> Display for SearchTree<S, A, Pl, Po> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Node count: {}\n{}", self.root.get_node_count(), self.root)
    }
}