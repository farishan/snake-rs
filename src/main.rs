mod constants;
mod game;
mod game_context;
mod point;
mod renderer;

use game::Game;

fn main() {
    Game::run_game();
}
