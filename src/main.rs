use mocats::{SearchTree, UctPolicy};
use mocats::tic_tac_toe::{TicTacToePlayer, TicTacToeMove, TicTacToePosition};

fn main() {
    println!("Welcome to mocats Tic-Tac-Toe!");
    println!();
    println!("Enter a legal position according the following scheme:");
    println!("    '.' for empty squares");
    println!("    'X' for X squares");
    println!("    'O' for O squares");
    println!("    Spaces are ignored");
    println!();
    println!("For example, the following position:");
    println!("    .X. ..O .X.");
    println!("...indicates the following board:");
    println!("    . X .");
    println!("    . . O");
    println!("    . X .");
    println!();
    let mut input = String::new();
    println!("Enter a legal position:");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    println!();
    let mut board_x: u16 = 0;
    let mut board_o: u16 = 0;
    let mut pos: u16 = 1;
    for c in input.chars() {
        match c {
            'X'|'x' => board_x |= pos,
            'O'|'o' => board_o |= pos,
            ' ' => continue,
            '.'|'\n' => (),
            _ => panic!("Invalid input")
        }
        pos <<= 1;
    }
    let num_x = board_x.count_ones();
    let num_o = board_o.count_ones();
    assert!(num_x == num_o || num_x == num_o + 1, "Invalid position");
    assert_eq!(board_x & board_o, 0, "Invalid position");
    assert!(board_x | board_o <= 0b111111111, "Invalid position");
    let mut game = TicTacToePosition {
        board_x,
        board_o,
        turn: if num_x == num_o { TicTacToePlayer::X } else { TicTacToePlayer::O }
    };
    input = String::new();
    println!("Is it your turn? (y/n)");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    println!();
    let user_turn = match input.trim() {
        "y"|"Y" => true,
        "n"|"N" => false,
        _ => panic!("Invalid input")
    };
    if !user_turn {
        play_best_move(&mut game);
    }
    assert!(game.get_moves().len() > 0, "Game is already over");
    println!("Print moves as tl, tm, tr, ml, mm, mr, bl, bm, br");
    println!();
    loop {
        println!("Current position:");
        println!("{}", game);
        println!("Enter your move: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        println!();
        input = input.trim().parse().unwrap();
        let m = match input.as_str() {
            "tl" => 0b1,
            "tm" => 0b10,
            "tr" => 0b100,
            "ml" => 0b1000,
            "mm" => 0b10000,
            "mr" => 0b100000,
            "bl" => 0b1000000,
            "bm" => 0b10000000,
            "br" => 0b100000000,
            _ => panic!("Invalid input")
        };
        let possible_moves = game.get_moves();
        if !possible_moves.contains(&m) {
            panic!("Invalid move");
        }
        game.make_move(m);
        println!("Current position:");
        println!("{}", game);
        if game.get_moves().is_empty() {
            break;
        }
        play_best_move(&mut game);
    }
    println!("Final position:");
    println!("{}", game);
    match game.get_winner() {
        Some(TicTacToePlayer::X) => println!("You win!"),
        Some(TicTacToePlayer::O) => println!("You lose!"),
        None => println!("Draw!")
    }
}

fn play_best_move(position: &mut TicTacToePosition) {
    let mut mcts = SearchTree::<TicTacToePosition, TicTacToeMove, TicTacToePlayer, UctPolicy>::new(*position, UctPolicy::new(2.));
    mcts.run(2000);
    let best = mcts.get_best_action().unwrap().pos;
    position.make_move(best);
    println!("Bot played a move!\n");
}