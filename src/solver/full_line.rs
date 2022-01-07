use crate::board::{Board, Pixel};
use crate::game::Game;
use crate::picross_image::{Clue, Image, WHITE};

use super::SolverAlgo;

pub struct FullRow;

pub struct FullCol;

impl SolverAlgo for FullRow {
    fn solve(&self, game: &mut Game) -> bool {
        let image = &game.image;
        let board = &mut game.board;
        for (y, row) in image.rows.iter().enumerate() {
            let mut line = row::Row::new(image, board, y);
            if solve_line(&mut line, row) {
                return true;
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
            let mut line = col::Col::new(image, board, x);
            if solve_line(&mut line, col) {
                return true;
            }
        }
        false
    }
}

trait Line {
    fn size(&self) -> usize;
    fn get_pixel(&self, index: usize) -> &Pixel;
    fn set_pixel(&mut self, index: usize, pixel: Pixel);
}

fn solve_line(line: &mut dyn Line, clues: &Vec<Clue>) -> bool {
    let mut current_color = WHITE;
    let mut counter = 0 as usize;

    // count how many pixel are there can be if they are all collapsed
    for clue in clues {
        if clue.color.eq(&current_color) {
            counter += 1;
        }
        counter += clue.count as usize;
        current_color = clue.color;
    }

    if counter == line.size() {
        // a full line is available
        let mut current_color = WHITE;
        let mut index = 0;
        let mut modified = false;
        for clue in clues {
            if clue.color.eq(&current_color) {
                line.set_pixel(index, Pixel::Cross);
                index += 1;
            }
            for _ in 0..clue.count {
                match line.get_pixel(index) {
                    Pixel::Color(color) => {
                        if color.eq(&WHITE) {
                            modified = true;
                        }
                    }
                    Pixel::Cross => {
                        modified = true;
                    }
                }
                line.set_pixel(index, Pixel::Color(clue.color));
                index += 1;
            }
            current_color = clue.color;
        }
        if modified {
            return true;
        }
    }
    false
}

mod row {
    use super::Line;
    use super::{Board, Image, Pixel};

    pub struct Row<'a> {
        image: &'a Image,
        board: &'a mut Board,
        nb_rows: usize,
    }

    impl<'a> Row<'a> {
        pub fn new(image: &'a Image, board: &'a mut Board, nb_rows: usize) -> Self {
            Row {
                image,
                board,
                nb_rows,
            }
        }
    }

    impl<'a> Line for Row<'a> {
        fn size(&self) -> usize {
            self.image.width as usize
        }

        fn get_pixel(&self, index: usize) -> &Pixel {
            &self.board.get_pixel(index, self.nb_rows)
        }

        fn set_pixel(&mut self, index: usize, pixel: Pixel) {
            self.board.set_pixel(index, self.nb_rows, &pixel);
        }
    }
}

mod col {
    use super::Line;
    use super::{Board, Image, Pixel};

    pub struct Col<'a> {
        image: &'a Image,
        board: &'a mut Board,
        nb_cols: usize,
    }

    impl<'a> Col<'a> {
        pub fn new(image: &'a Image, board: &'a mut Board, nb_cols: usize) -> Self {
            Col {
                image,
                board,
                nb_cols,
            }
        }
    }

    impl<'a> Line for Col<'a> {
        fn size(&self) -> usize {
            self.image.height as usize
        }

        fn get_pixel(&self, index: usize) -> &Pixel {
            &self.board.get_pixel(self.nb_cols, index)
        }

        fn set_pixel(&mut self, index: usize, pixel: Pixel) {
            self.board.set_pixel(self.nb_cols, index, &pixel);
        }
    }
}

#[cfg(test)]
mod tests {
    use image::Rgb;

    use crate::solver::Solver;

    use super::*;

    const BLACK: Pixel = Pixel::Color(Rgb([0, 0, 0]));

    #[test]
    fn it_solves_full_lines() {
        let game_res = Game::new("test/4x4-shuriken.png");
        assert!(&game_res.is_ok());
        let mut game = game_res.unwrap();
        let solver = Solver {
            algos: vec![Box::new(FullRow {})],
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
            algos: vec![Box::new(FullCol {})],
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

        assert_eq!(game.board.get_pixel(3, 0), &BLACK);
        assert_eq!(game.board.get_pixel(3, 1), &BLACK);
        assert_eq!(game.board.get_pixel(3, 2), &Pixel::Cross);
        assert_eq!(game.board.get_pixel(3, 3), &BLACK);
    }
}
