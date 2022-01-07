use std::env;

mod game;
mod picross_image;
mod board;
mod solver;

use game::Game;
use solver::SolverBuilder;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        None => {
            println!("usage : picross <filename>")
        }
        Some(filename) => {
            match Game::new(filename) {
                Err(e) => eprintln!("{:?}", e),
                Ok(mut game) => {
                    if play(&mut game) {
                        println!("YOU WIN")
                    } 
                    else {
                        println!("NOT FINISHED")
                    }
                }
            }
        },
    }
}

fn play(game: &mut Game) -> bool {
    let solver = SolverBuilder::new().build();
    while !game.is_finished() {
        if solver.solve(game) == false {
            return false;
        }
        println!("BOARD");
        println!("{}", game.board);
    }
    true
}
