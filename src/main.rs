use mocats::{SearchTree, UctPolicy};
use mocats::tic_tac_toe::{TicTacToePlayer, TicTacToeMove, TicTacToePosition};

fn main() {
    play_as_x();
}

fn play_as_x() {
    let mut game = TicTacToePosition::new();
    loop {
        println!("Current position:");
        println!("{}", game);
        println!("Print moves as tl, tm, tr, ml, mm, mr, bl, bm, br");
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
        if game.get_moves().is_empty() {
            break;
        }
        let mut mcts = SearchTree::<TicTacToePosition, TicTacToeMove, TicTacToePlayer, UctPolicy>::new(game, UctPolicy::new(2.));
        mcts.run(2000);
        let best = mcts.get_best_action().unwrap().pos;
        game.make_move(best);
        println!("Bot played a move!\n");
    }
    println!("Final position:");
    println!("{}", game);
    match game.get_winner() {
        Some(TicTacToePlayer::X) => panic!("The bot is trash!"),
        Some(TicTacToePlayer::O) => println!("You lose!"),
        None => println!("Draw!")
    }
}