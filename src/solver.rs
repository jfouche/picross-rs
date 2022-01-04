use crate::game::{Board, Game, WHITE};

pub trait SolverAlgo {
    fn solve(&self, game: &Game, board: &mut Board) -> bool;
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
    pub fn solve(&self, game: &Game, board: &mut Board) -> bool {
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
    fn solve(&self, game: &Game, board: &mut Board) -> bool {
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
                        x += 1;
                    }
                    for _ in 0..clue.count {
                        if board.get_pixel(x, y as u32).eq(&WHITE) {
                            modified = true;
                        }
                        board.set_pixel(x, y as u32, clue.color);
                        x += 1;
                    }
                    current_color = clue.color;
                }
                if modified {
                    return true
                }
            }
        }
        false
    }
}

impl SolverAlgo for FullCol {
    fn solve(&self, game: &Game, board: &mut Board) -> bool {
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
                        y += 1;
                    }
                    for _ in 0..clue.count {
                        if board.get_pixel(x as u32, y).eq(&WHITE) {
                            modified = true;
                        }
                        board.set_pixel(x as u32, y, clue.color);
                        y += 1;
                    }
                    current_color = clue.color;
                }
                if modified {
                    return true
                }
            }
        }
        false
    }
}
