use std::env;

mod game;
mod picross_image;
mod board;
mod solver;

use picross_image::Image;
use solver::{FullRow, SolverBuilder, FullCol};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        None => {
            println!("usage : picross <filename>")
        }
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
