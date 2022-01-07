use std::env;

use game::Image;
use solver::{FullRow, SolverBuilder, FullCol};

mod game;
mod solver;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    match args.get(1) {
        Some(filename) => {
            match Image::from_image(filename) {
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
        },
        None => {
            println!("usage : picross <filename>")
        }
    }
    

}

fn play(game: &Image) -> bool {
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
