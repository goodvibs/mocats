#[cfg(test)]

use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::game::{GameAction, GameState};
use crate::game;
use crate::search_tree::SearchTree;
use crate::tree_policy::UctPolicy;


#[test]
fn test() {
    let mut tablebase: HashMap<TicTacToePosition, Option<Player>> = HashMap::new();
    fn test_helper(game: &mut TicTacToePosition, tablebase: &mut HashMap<TicTacToePosition, Option<Player>>) -> Option<Player> {
        let moves = game.get_moves();
        if moves.len() == 0 {
            return game.get_winner();
        }
        let mut results = Vec::new();
        for m in moves {
            let mut game = game.clone();
            game.make_move(m);
            results.push(test_helper(&mut game, tablebase));
        }
        let mut contains_draw = false;
        let mut contains_player_win = false;
        let mut contains_other_win = false;
        let res;
        for r in results {
            match r {
                None => contains_draw = true,
                Some(winner) => {
                    if winner == game.get_turn() {
                        contains_player_win = true;
                        break;
                    } else {
                        contains_other_win = true;
                    }
                }
            }
        }
        if contains_player_win {
            res = Some(game.get_turn());
        } else if contains_draw {
            res = None;
        } else if contains_other_win {
            res = Some(game.get_turn().other());
        } else {
            panic!("No results");
        }
        tablebase.insert(game.clone(), res);
        for action in game.get_actions() {
            let m = action.pos;
            let mut game = game.clone();
            game.make_move(m);
            if game.get_actions().is_empty() {
                continue;
            }
            let mut mcts = SearchTree::<TicTacToePosition, TicTacToeMove, Player, UctPolicy>::new(game, UctPolicy::new(2.));
            mcts.run(1000);
            let best = mcts.get_best_action().unwrap().pos;
            game.make_move(best);
            assert_eq!(tablebase.get(&game).unwrap(), &res, "Game: {}\nMove: {}\nBest: {}\n", game, m, best);
        }
        res
    }
    let mut game = TicTacToePosition::new();
    test_helper(&mut game, &mut tablebase);
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TicTacToeMove {
    pos: u16
}

impl GameAction for TicTacToeMove {}

impl Display for TicTacToeMove {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut board_str = String::new();
        let mut pos: u16 = 1;
        for _i in 0..3 {
            for _j in 0..3 {
                match self.pos & pos {
                    0 => board_str.push('.'),
                    _ => board_str.push('?')
                }
                pos <<= 1;
            }
            board_str.push('\n');
        }
        write!(f, "{}", board_str)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Player {
    X,
    O
}

impl Player {
    fn other(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X
        }
    }
}

impl game::Player for Player {}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O")
        }
    }
}

const BOARD_MASK: u16 = 0b111111111;

const WIN_MASKS: [u16; 8] = [
    0b111000000,
    0b000111000,
    0b000000111,
    0b100100100,
    0b010010010,
    0b001001001,
    0b100010001,
    0b001010100
];

fn print_board(board: u16, c: char) {
    let mut board_str = String::new();
    let mut pos: u16 = 1;
    for _i in 0..3 {
        for _j in 0..3 {
            match board & pos {
                0 => board_str.push('.'),
                _ => board_str.push(c)
            }
            pos <<= 1;
        }
        board_str.push('\n');
    }
    println!("{}", board_str);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct TicTacToePosition {
    pub board_x: u16,
    pub board_o: u16,
    pub turn: Player,
}

impl TicTacToePosition {
    pub fn new() -> TicTacToePosition {
        TicTacToePosition {
            board_x: 0,
            board_o: 0,
            turn: Player::X,
        }
    }

    pub fn get_moves_mask(&self) -> u16 {
        !(self.board_x | self.board_o) & BOARD_MASK
    }

    pub fn get_moves(&self) -> Vec<u16> {
        if self.is_board_full() | self.get_winner().is_some() {
            return Vec::new();
        }
        unpack(self.get_moves_mask())
    }

    pub fn make_move(&mut self, pos: u16) {
        match self.turn {
            Player::X => self.board_x |= pos,
            Player::O => self.board_o |= pos
        }
        self.turn = self.turn.other();
    }

    pub fn is_board_full(&self) -> bool {
        self.board_x | self.board_o == BOARD_MASK
    }

    pub fn get_winner(&self) -> Option<Player> {
        for mask in WIN_MASKS.iter() {
            if self.board_x & *mask == *mask {
                return Some(Player::X);
            }
            if self.board_o & *mask == *mask {
                return Some(Player::O);
            }
        }
        None
    }
}

impl Display for TicTacToePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut board_str = String::new();
        let mut pos: u16 = 1;
        for _i in 0..3 {
            for _j in 0..3 {
                match self.board_x & pos {
                    0 => match self.board_o & pos {
                        0 => board_str.push('.'),
                        _ => board_str.push('O')
                    },
                    _ => board_str.push('X')
                }
                pos <<= 1;
            }
            board_str.push('\n');
        }
        let winner = self.get_winner();
        match winner {
            Some(Player::X) => board_str.push_str("Winner: X\n"),
            Some(Player::O) => board_str.push_str("Winner: O\n"),
            None => {
                if self.is_board_full() {
                    board_str.push_str("Draw\n");
                }
                else {
                    board_str.push_str("Turn: ");
                    match self.turn {
                        Player::X => board_str.push('X'),
                        Player::O => board_str.push('O')
                    }
                    board_str.push('\n');
                }
            }
        }
        write!(f, "{}", board_str)
    }
}

impl GameState<TicTacToeMove, Player> for TicTacToePosition {
    fn get_actions(&self) -> Vec<TicTacToeMove> {
        self.get_moves().iter().map(|&pos| TicTacToeMove { pos }).collect()
    }

    fn take_action(&mut self, action: &TicTacToeMove) {
        self.make_move(action.pos);
    }

    fn get_turn(&self) -> Player {
        self.turn
    }

    fn get_reward_for_player(&self, player: Player) -> f32 {
        match self.get_winner() {
            Some(winner) => {
                return if winner == player {
                    1.
                } else {
                    -1.
                }
            },
            None => 0.
        }
    }
}

fn unpack(mut board: u16) -> Vec<u16> {
    let mut res: Vec<u16> = Vec::new();
    while board != 0 {
        let lsb = board & board.wrapping_neg();
        res.push(lsb);
        board ^= lsb;
    }
    res
}
