use crate::tic_tac_toe::{TicTacToeMove, TicTacToePlayer, TicTacToePosition};
use crate::{SearchTree, UctPolicy};

#[test]
fn weak_test() {
    let init_game_state = TicTacToePosition::new();
    for m in init_game_state.get_moves() {
        let mut game_state = init_game_state.clone();
        game_state.make_move(m);
        let opt_result = get_opt_result_by_mocats(game_state);
        assert_eq!(opt_result, None);
    }
}

fn get_opt_result_by_mocats(pos: TicTacToePosition) -> Option<TicTacToePlayer> {
    let mut game = pos.clone();
    let mut moves = game.get_moves();
    while !moves.is_empty() {
        let mut mcts = SearchTree::<TicTacToePosition, TicTacToeMove, TicTacToePlayer, UctPolicy>::new(game, UctPolicy::new(2.));
        mcts.run(2000);
        let best = mcts.get_best_action().unwrap().pos;
        game.make_move(best);
        moves = game.get_moves();
    }
    game.get_winner()
}

