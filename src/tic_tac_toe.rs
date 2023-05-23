use std::fmt::{Display, Formatter};
use std::fmt;
use crate::{GameAction, GameState, Player};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TicTacToeMove {
    pub pos: u16
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
pub enum TicTacToePlayer {
    X,
    O
}

impl TicTacToePlayer {
    pub(crate) fn other(&self) -> TicTacToePlayer {
        match self {
            TicTacToePlayer::X => TicTacToePlayer::O,
            TicTacToePlayer::O => TicTacToePlayer::X
        }
    }
}

impl Player for TicTacToePlayer {}

impl Display for TicTacToePlayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TicTacToePlayer::X => write!(f, "X"),
            TicTacToePlayer::O => write!(f, "O")
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

pub fn print_board(board: u16, c: char) {
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
pub struct TicTacToePosition {
    pub board_x: u16,
    pub board_o: u16,
    pub turn: TicTacToePlayer,
}

impl TicTacToePosition {
    pub fn new() -> TicTacToePosition {
        TicTacToePosition {
            board_x: 0,
            board_o: 0,
            turn: TicTacToePlayer::X,
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
            TicTacToePlayer::X => self.board_x |= pos,
            TicTacToePlayer::O => self.board_o |= pos
        }
        self.turn = self.turn.other();
    }

    pub fn is_board_full(&self) -> bool {
        self.board_x | self.board_o == BOARD_MASK
    }

    pub fn get_winner(&self) -> Option<TicTacToePlayer> {
        for mask in WIN_MASKS.iter() {
            if self.board_x & *mask == *mask {
                return Some(TicTacToePlayer::X);
            }
            if self.board_o & *mask == *mask {
                return Some(TicTacToePlayer::O);
            }
        }
        None
    }
}

impl Display for TicTacToePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut board_str = String::new();
        // let mut board_str = format!("X: {}\nO: {}\n", self.board_x, self.board_o);
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
            Some(TicTacToePlayer::X) => board_str.push_str("Winner: X\n"),
            Some(TicTacToePlayer::O) => board_str.push_str("Winner: O\n"),
            None => {
                if self.is_board_full() {
                    board_str.push_str("Draw\n");
                }
                else {
                    board_str.push_str("Turn: ");
                    match self.turn {
                        TicTacToePlayer::X => board_str.push('X'),
                        TicTacToePlayer::O => board_str.push('O')
                    }
                    board_str.push('\n');
                }
            }
        }
        write!(f, "{}", board_str)
    }
}

impl GameState<TicTacToeMove, TicTacToePlayer> for TicTacToePosition {
    fn get_actions(&self) -> Vec<TicTacToeMove> {
        self.get_moves().iter().map(|&pos| TicTacToeMove { pos }).collect()
    }

    fn take_action(&mut self, action: &TicTacToeMove) {
        self.make_move(action.pos);
    }

    fn get_turn(&self) -> TicTacToePlayer {
        self.turn
    }

    fn get_reward_for_player(&self, player: TicTacToePlayer) -> f32 {
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
