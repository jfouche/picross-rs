mod game;


fn main() {
    match game::Game::from_image("test/4x4-square.png") {
        Ok(game) => {},
        Err(e) => eprintln!("{:?}", e)
    }
}
