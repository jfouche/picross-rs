use crate::{Image, board::{Board, Pixel}, picross_image::WHITE};

pub trait SolverAlgo {
    fn solve(&self, game: &Image, board: &mut Board) -> bool;
}

pub struct Solver {
    algos: Vec<Box<dyn SolverAlgo>>,
}

pub struct SolverBuilder {
    algos: Vec<Box<dyn SolverAlgo>>,
}

impl SolverBuilder {
    pub fn new() -> Self {
        SolverBuilder { algos: vec![] }
    }

    pub fn add(mut self, algo: Box<dyn SolverAlgo>) -> SolverBuilder {
        self.algos.push(algo);
        self
    }

    pub fn build(self) -> Solver {
        Solver { algos: self.algos }
    }
}

impl Solver {
    pub fn solve(&self, game: &Image, board: &mut Board) -> bool {
        for algo in &self.algos {
            if algo.solve(game, board) {
                return true;
            }
        }
        return false;
    }
}

pub struct FullRow;

pub struct FullCol;

impl SolverAlgo for FullRow {
    fn solve(&self, game: &Image, board: &mut Board) -> bool {
        for (y, row) in game.rows.iter().enumerate() {
            let mut current_color = WHITE;
            let mut counter = 0;
            for clue in row {
                if clue.color.eq(&current_color) {
                    counter += 1;
                }
                counter += clue.count;
                current_color = clue.color;
            }
            if counter == game.width {
                // a full line is available
                let mut current_color = WHITE;
                let mut x = 0;
                let mut modified = false;
                for clue in row {
                    if clue.color.eq(&current_color) {
                        board.set_pixel(x, y, &Pixel::Cross);
                        x += 1;
                    }
                    for _ in 0..clue.count {
                        match board.get_pixel(x, y) {
                            Pixel::Color(color) => {
                                if color.eq(&WHITE) {
                                    modified = true;
                                }
                            }
                            Pixel::Cross => {
                                modified = true;
                            }
                        }
                        board.set_pixel(x, y, &Pixel::Color(clue.color));
                        x += 1;
                    }
                    current_color = clue.color;
                }
                if modified {
                    return true;
                }
            }
        }
        false
    }
}

impl SolverAlgo for FullCol {
    fn solve(&self, game: &Image, board: &mut Board) -> bool {
        for (x, col) in game.cols.iter().enumerate() {
            let mut current_color = WHITE;
            let mut counter = 0;
            for clue in col {
                if clue.color.eq(&current_color) {
                    counter += 1;
                }
                counter += clue.count;
                current_color = clue.color;
            }
            if counter == game.height {
                // a full line is available
                let mut current_color = WHITE;
                let mut y = 0;
                let mut modified = false;
                for clue in col {
                    if clue.color.eq(&current_color) {
                        board.set_pixel(x, y, &Pixel::Cross);
                        y += 1;
                    }
                    for _ in 0..clue.count {
                        match board.get_pixel(x, y) {
                            Pixel::Color(color) => {
                                if color.eq(&WHITE) {
                                    modified = true;
                                }
                            }
                            Pixel::Cross => {
                                modified = true;
                            }
                        }
                        board.set_pixel(x, y, &Pixel::Color(clue.color));                        y += 1;
                    }
                    current_color = clue.color;
                }
                if modified {
                    return true;
                }
            }
        }
        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_full_lines() {
        let game_res = Image::from_image("test/4x4-shuriken.png");
        assert!(&game_res.is_ok());
        let game = game_res.unwrap();

        let mut board = game.new_board();
        let solver = SolverBuilder::new().add(Box::new(FullRow {})).add(Box::new(FullCol {})).build();
        let mut failed = false;
        while !game.is_finished(&board) {
            if solver.solve(&game, &mut board) == false {
                failed = true;
            }
            println!("BOARD");
            println!("{}", board);
        }
    
        assert!(!failed);
    }
}
