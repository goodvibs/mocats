use std::collections::HashMap;
use crate::tic_tac_toe::{TicTacToeMove, TicTacToePlayer, TicTacToePosition};
use crate::{GameState, SearchTree, UctPolicy};

#[test]
fn test_all_ttt_positions() {
    let mut tablebase: HashMap<TicTacToePosition, Option<TicTacToePlayer>> = HashMap::new();
    fn helper(init_game_state: &mut TicTacToePosition, tablebase: &mut HashMap<TicTacToePosition, Option<TicTacToePlayer>>) -> Option<TicTacToePlayer> {
        let moves = init_game_state.get_moves();
        if moves.is_empty() {
            return init_game_state.get_winner();
        }
        let mut minimax_results_for_moves = Vec::new();
        for m in moves {
            let mut game = init_game_state.clone();
            game.make_move(m);
            minimax_results_for_moves.push(helper(&mut game, tablebase));
        }
        let mut contains_player_win = false;
        let mut contains_draw = false;
        let mut contains_other_win = false;
        let res;
        for r in minimax_results_for_moves {
            match r {
                None => contains_draw = true,
                Some(winner) => {
                    if winner == init_game_state.get_turn() {
                        contains_player_win = true;
                        break;
                    } else {
                        contains_other_win = true;
                    }
                }
            }
        }
        if contains_player_win {
            res = Some(init_game_state.get_turn());
        } else if contains_draw {
            res = None;
        } else if contains_other_win {
            res = Some(init_game_state.get_turn().other());
        } else {
            panic!("No results");
        }
        if tablebase.contains_key(init_game_state) {
            assert_eq!(*tablebase.get(init_game_state).unwrap(), res, "{}", init_game_state);
        }
        for m in init_game_state.get_moves() {
            let mut game = init_game_state.clone();
            game.make_move(m);
            if game.get_actions().is_empty() {
                continue;
            }
            let mut mcts = SearchTree::<TicTacToePosition, TicTacToeMove, TicTacToePlayer, UctPolicy>::new(game, UctPolicy::new(2.));
            mcts.run(2000);
            let best = mcts.get_best_action().unwrap().pos;
            game.make_move(best);
            tablebase.insert(game, res);
        }
        res
    }
    let mut game = TicTacToePosition::new();
    helper(&mut game, &mut tablebase);
}
