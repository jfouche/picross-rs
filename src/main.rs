use std::env;

//mod window;

use picross_rs::Game;
use picross_rs::SolverBuilder;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        None => {
            println!("usage : picross <filename>")
        }
        Some(filename) => {
            match Game::new(filename) {
                Err(e) => eprintln!("Error initializing game \"{}\"\n{}", filename, e),
                Ok(mut game) => {
                    //window::show(game);
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
        if let Some(proposition) = solver.solve(game) {
            proposition.merge(&mut game.board);
        } else {
            return false;
        }
        println!("BOARD");
        println!("{}", game.board);
    }
    true
}
