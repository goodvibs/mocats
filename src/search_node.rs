use std::fmt;
use rand::seq::SliceRandom;
use crate::game::{GameAction, GameState, Player};
use crate::tree_policy::TreePolicy;

/// Represents the state of a node in the search tree.
pub struct SearchNode<A, Pl> where A: GameAction, Pl: Player {
    /// The action that this node represents. Only None for the root node.
    pub action: Option<A>,
    /// The children of this node.
    pub children: Vec<SearchNode<A, Pl>>,
    /// The player whose turn it was at the root node (initial game position).
    pub root_player: Pl,
    /// The state of this node.
    pub state: NodeState,
    /// The number of times this node has been visited.
    pub visits: u32,
    /// The total value of this node as a result of rollouts.
    pub total_value: f32
}

impl<A, Pl> SearchNode<A, Pl> where A: GameAction, Pl: Player {
    /// Constructs a new search node with the given action and root_player.
    pub fn new(action: Option<A>, root_player: Pl) -> SearchNode<A, Pl> {
        SearchNode::<A, Pl> {
            action: action,
            children: Vec::new(),
            root_player: root_player,
            state: NodeState::ExpandableLeaf,
            visits: 0,
            total_value: 0.0
        }
    }

    /// Runs a single iteration of the MCTS algorithm.
    /// Returns the reward for the player whose turn it was at the root node (initial game position).
    pub fn run_iteration<S, Po>(&mut self, game: &mut S, tree_policy: &Po) -> f32 where S: GameState<A, Pl>, Po: TreePolicy<A, Pl> {
        let delta = match self.state {
            NodeState::ExpandableLeaf => {
                let root_player = self.root_player;
                match self.expand(game) {
                    Some(best_child) =>  {
                        game.apply_action(&best_child.action.expect("Expected child node to have action"));
                        let mut available = game.get_actions();
                        while available.len() > 0 {
                            let action = available.choose(&mut rand::thread_rng()).expect("Expected available actions to be non-empty");
                            game.apply_action(&action);
                            available = game.get_actions();
                        }
                        let reward = game.get_reward_for_player(root_player);
                        best_child.visits += 1;
                        best_child.total_value += reward;
                        reward
                    },
                    None => return game.get_reward_for_player(root_player)
                }
            },
            NodeState::TerminalLeaf => {
                game.get_reward_for_player(self.root_player)
            },
            NodeState::Expanded => {
                let child = tree_policy.select_child(self, game.get_turn() == self.root_player);
                game.apply_action(&child.action.expect("Expected child node to have action"));
                child.run_iteration(game, tree_policy)
            }
        };
        self.visits += 1;
        self.total_value += delta;
        delta
    }

    /// Adds a child node to this leaf node if it is expandable, using a random legal action.
    /// If it is not, marks this node as a TerminalLeaf.
    /// If there is only one allowed action, this node is marked as a TerminalLeaf after expansion.
    pub fn expand<S>(&mut self, game: &S) -> Option<&mut SearchNode<A, Pl>> where S: GameState<A, Pl> {
        let allowed_actions = game.get_actions();
        if allowed_actions.is_empty() {
            self.state = NodeState::TerminalLeaf;
            return None;
        }
        let mut child_actions : Vec<A> = Vec::new();
        for child in &self.children {
            child_actions.push(child.action.expect("Child node without action"));
        }
        let mut candidate_actions = Vec::new();
        for action in allowed_actions {
            if !child_actions.contains(&action) {
                candidate_actions.push(action);
            }
        }
        assert!(candidate_actions.len() > 0, "Expected at least one candidate action");
        if candidate_actions.len() == 1 {
            self.children.push(SearchNode::new(Some(candidate_actions[0]), self.root_player));
            self.state = NodeState::Expanded;
        }
        else {
            let rand_action = *candidate_actions.choose(&mut rand::thread_rng()).expect("Expected candidate actions to be non-empty");
            let node = SearchNode::new(Some(rand_action), self.root_player);
            self.children.push(node);
            // self.children.push(SearchNode::new(Some(*candidate_actions.choose(&mut rand::thread_rng()).expect("Expected candidate actions to be non-empty"))));
        }
        self.children.last_mut()
    }

    pub fn get_node_count(&self) -> u32 {
        let mut count: u32 = 1;
        for child in &self.children {
            count += child.get_node_count();
        }
        count
    }
}

impl<A, Pl> fmt::Display for SearchNode<A, Pl> where A: GameAction, Pl: Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn fmt_subtree<A: GameAction, Pl: Player>(f: &mut fmt::Formatter, node: &SearchNode<A, Pl>, indent_level :i32) -> fmt::Result {
            for _ in 0..indent_level {
                f.write_str("|    ")?;
            }
            match node.action {
                Some(a) => {

                    let state = match node.state {
                        NodeState::ExpandableLeaf => "EXPANDABLE_LEAF",
                        NodeState::TerminalLeaf => "TERMINAL_LEAF",
                        NodeState::Expanded => "EXPANDED"
                    };
                    writeln!(f, "[{}] {:?} total={} visits={}", state, a, node.total_value, node.visits)?;
                    format!("{}", a).split("\n").for_each(|line| {
                        for _ in 0..(indent_level + 1) {
                            f.write_str("|    ").unwrap();
                        }
                        writeln!(f, "{}", line).unwrap();
                    });
                },
                None => writeln!(f, "[ROOT] total={} visits={}", node.total_value, node.visits)?
            }
            for child in &node.children {
                fmt_subtree(f, child, indent_level + 1)?;
            }
            write!(f, "")
        }
        fmt_subtree(f, self, 0)
    }
}

/// Represents the state of a node in the search tree.
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum NodeState {
    /// No children, but might have available actions
    ExpandableLeaf,
    /// No children, no available actions
    TerminalLeaf,
    /// Has children (already expanded)
    Expanded
}
