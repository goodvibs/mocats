<!-- cargo-rdme start -->

# mocats

[![Crates.io](https://img.shields.io/crates/v/mocats)](https://crates.io/crates/mocats)
[![License](https://img.shields.io/crates/l/mocats)](./LICENSE)

A fast, easy-to-use, generalized Monte Carlo Tree Search library.
Works for any game, any number of players, and any tree policy (UctPolicy included as a default).
As of the current version, the search is single-threaded.

## Features
- Fast and efficient Monte Carlo Tree Search implementation
- Easy-to-use API
- Customizable number of players (uses paranoid approach for more than 2 players)
- Customizable tree policies
- Nicely formatted display output for debugging

## Usage

In the root directory of your project, add the `mocats` dependency to your `Cargo.toml` file:

```bash
cargo add mocats
```

...or add this to your `Cargo.toml`:

```toml
[dependencies]
mocats = "0.3.0"
```

### Defining a game

To use `mocats`, you must define a game and a tree policy. A game is defined by three traits:

- `GameState`: Represents a game state.
- `GameAction`: Represents a legal game action that can be applied to some `GameState`.
- `Player`: Represents a player in a game. Should be an enum.

A tree policy is defined by one trait:

- `TreePolicy`: Represents a tree policy.

The `UctPolicy` struct is included as a default tree policy.

### Running the search

To run the search, create a `SearchTree` struct with the game and tree policy, then call `run` on it.

```rust
use mocats::{tic_tac_toe, UctPolicy};

fn foo() {
    let game = tic_tac_toe::TicTacToePosition::new();
    let tree_policy = UctPolicy::new(2.0);
    let mut search_tree = mocats::SearchTree::new(game, tree_policy);
    search_tree.run(2000);
    let best_action = search_tree.get_best_action();
    println!("{}", search_tree);
    println!("Best action: {}", best_action.unwrap());
}
```

## Example

See the `mocats::tic_tac_toe module` for a full example of implementing Tic Tac Toe using `mocats`.
You can import `tic_tac_toe` to use it in your code.

```rust
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TicTacToeMove {
    pub pos: u16
}

impl mocats::GameAction for TicTacToeMove {}

impl Display for TicTacToeMove {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TicTacToePlayer {
    X,
    O
}

impl TicTacToePlayer {}

impl mocats::Player for TicTacToePlayer {}

impl Display for TicTacToePlayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct TicTacToePosition {
    pub board_x: u16,
    pub board_o: u16,
    pub turn: TicTacToePlayer,
}

impl TicTacToePosition {
    pub fn new() -> TicTacToePosition {
        todo!()
    }
}

impl Display for TicTacToePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl mocats::GameState<TicTacToeMove, TicTacToePlayer> for TicTacToePosition {
    fn get_actions(&self) -> Vec<TicTacToeMove> {
        todo!()
    }

    fn apply_action(&mut self, action: &TicTacToeMove) {
        todo!()
    }

    fn get_turn(&self) -> TicTacToePlayer {
        todo!()
    }

    fn get_reward_for_player(&self, player: TicTacToePlayer) -> f32 {
        todo!()
    }
}
```

## Documentation

For more detailed documentation and usage examples, refer to the [API documentation](https://docs.rs/mocats/0.2.1/mocats/).

## Contributing

Contributions in the form of pull requests are welcome!
If you encounter any issues or have suggestions for improvements, please open an issue on the GitHub repository.

## License

This project is licensed under the MIT License. See the [LICENSE file](./LICENSE) for details.

<!-- cargo-rdme end -->
