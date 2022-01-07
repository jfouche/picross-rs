use crate::{board::Pixel, picross_image::WHITE, game::Game};

pub trait SolverAlgo {
    fn solve(&self, game: &mut Game) -> bool;
}

pub struct Solver {
    algos: Vec<Box<dyn SolverAlgo>>,
}

pub struct SolverBuilder {
    algos: Vec<Box<dyn SolverAlgo>>,
}

impl SolverBuilder {
    pub fn new() -> Self {
        SolverBuilder { 
            algos: vec![Box::new(FullRow {}), Box::new(FullCol {})] 
        }
    }

    #[allow(dead_code)]
    pub fn add(mut self, algo: Box<dyn SolverAlgo>) -> SolverBuilder {
        self.algos.push(algo);
        self
    }

    pub fn build(self) -> Solver {
        Solver { algos: self.algos }
    }
}

impl Solver {
    pub fn solve(&self, game: &mut Game) -> bool {
        for algo in &self.algos {
            if algo.solve(game) {
                return true;
            }
        }
        return false;
    }
}

pub struct FullRow;

pub struct FullCol;

impl SolverAlgo for FullRow {
    fn solve(&self, game: &mut Game) -> bool {
        let image = &game.image;
        let board = &mut game.board;
        for (y, row) in image.rows.iter().enumerate() {
            let mut current_color = WHITE;
            let mut counter = 0;
            for clue in row {
                if clue.color.eq(&current_color) {
                    counter += 1;
                }
                counter += clue.count;
                current_color = clue.color;
            }
            if counter == image.width {
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
    fn solve(&self, game: &mut Game) -> bool {
        let image = &game.image;
        let board = &mut game.board;
        for (x, col) in image.cols.iter().enumerate() {
            let mut current_color = WHITE;
            let mut counter = 0;
            for clue in col {
                if clue.color.eq(&current_color) {
                    counter += 1;
                }
                counter += clue.count;
                current_color = clue.color;
            }
            if counter == image.height {
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
    use image::Rgb;

    use super::*;

    const BLACK: Pixel = Pixel::Color(Rgb([0, 0, 0]));

    #[test]
    fn it_solves_full_lines() {
        let game_res = Game::new("test/4x4-shuriken.png");
        assert!(&game_res.is_ok());
        let mut game = game_res.unwrap();
        let solver = Solver { 
            algos: vec![Box::new(FullRow {})]
        };
        assert!(solver.solve(&mut game));
        assert!(solver.solve(&mut game));

        // ██X█
        //     
        //     
        // █X██

        assert_eq!(game.board.get_pixel(0, 0), &BLACK);
        assert_eq!(game.board.get_pixel(1, 0), &BLACK);
        assert_eq!(game.board.get_pixel(2, 0), &Pixel::Cross);
        assert_eq!(game.board.get_pixel(3, 0), &BLACK);

        assert_eq!(game.board.get_pixel(0, 3), &BLACK);
        assert_eq!(game.board.get_pixel(1, 3), &Pixel::Cross);
        assert_eq!(game.board.get_pixel(2, 3), &BLACK);
        assert_eq!(game.board.get_pixel(3, 3), &BLACK);
    }

    #[test]
    fn it_solves_full_cols() {
        let game_res = Game::new("test/4x4-shuriken.png");
        assert!(&game_res.is_ok());
        let mut game = game_res.unwrap();
        let solver = Solver { 
            algos: vec![Box::new(FullCol {})]
        };
        assert!(solver.solve(&mut game));
        assert!(solver.solve(&mut game));

        // █  █
        // X  █
        // █  X
        // █  █
        assert_eq!(game.board.get_pixel(0, 0), &BLACK);
        assert_eq!(game.board.get_pixel(0, 1), &Pixel::Cross);
        assert_eq!(game.board.get_pixel(0, 2), &BLACK);
        assert_eq!(game.board.get_pixel(0, 3), &BLACK);

        assert_eq!(game.board.get_pixel(3, 3), &BLACK);
        assert_eq!(game.board.get_pixel(3, 0), &BLACK);
        assert_eq!(game.board.get_pixel(3, 0), &Pixel::Cross);
        assert_eq!(game.board.get_pixel(3, 3), &BLACK);
    }
}
