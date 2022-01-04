use game::Game;
use solver::{FullRow, SolverBuilder, FullCol};

mod game;
mod solver;

fn main() {
    match Game::from_image("test/4x4-square.png") {
        Err(e) => eprintln!("{:?}", e),
        Ok(game) => {
            if play(&game) {
                println!("YOU WIN")
            } 
            else {
                println!("NOT FINISHED")
            }
        }
    }
}

fn play(game: &Game) -> bool {
    let mut board = game.new_board();
    let solver = SolverBuilder::new().add(Box::new(FullRow {})).add(Box::new(FullCol {})).build();
    while !game.is_finished(&board) {
        if solver.solve(&game, &mut board) == false {
            return false;
        }
        println!("BOARD");
        println!("{}", board);
    }
    true
}
