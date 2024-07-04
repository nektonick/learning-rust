mod game;
mod render;

fn main() {
    let mut game = game::Game::new();
    game.play()
}
