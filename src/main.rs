use std::env;

mod window;

use picross_rs::game::Game;
use picross_rs::solver::SolverBuilder;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        None => {
            println!("usage : picross <filename>")
        }
        Some(filename) => {
            match Game::new(filename) {
                Err(e) => eprintln!("Error initializing game : {:?}", e),
                Ok(game) => {
                    window::show(game);
                    // if play(&mut game) {
                    //     println!("YOU WIN")
                    // } 
                    // else {
                    //     println!("NOT FINISHED")
                    // }
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
