use crate::board::{Pixel};
use crate::game::Game;
use crate::picross_image::{WHITE};

use super::{GameLine, Proposition, SolverAlgo};

pub struct FullLine;

impl SolverAlgo for FullLine {
    fn solve(&self, _game: &mut Game) -> bool {
        false
    }

    fn get_proposition(&self, game_line: &GameLine) -> Option<super::Proposition> {
        let mut current_color = WHITE;
        let mut counter = 0 as usize;

        // count how many pixel are there can be if they are all collapsed
        for clue in game_line.clues {
            if clue.color.eq(&current_color) {
                counter += 1;
            }
            counter += clue.count as usize;
            current_color = clue.color;
        }

        if counter != game_line.board_line.len() {
            return None;
        }

        // a full line is available
        let mut current_color = WHITE;
        let mut index = 0;
        let mut proposition = vec![Some(Pixel::Cross); game_line.board_line.len()];
        let mut changes = false;
        for clue in game_line.clues {
            if clue.color.eq(&current_color) {
                // 2 consecutive colors, allow a space between them
                index += 1;
            }
            for _ in 0..clue.count {
                // Check if the board already contains this color
                let board_pixel = game_line.board_line[index];
                if let Pixel::Color(color) = board_pixel {
                    if !color.eq(&clue.color) {
                        changes = true;
                    }
                }
                // Add the color
                proposition[index] = Some(Pixel::Color(clue.color));
                index += 1;
            }
            current_color = clue.color;
        }
        if changes {
            Some(Proposition {
                view: game_line.view,
                line: proposition,
                index: game_line.index,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use image::Rgb;

    use crate::solver::{Solver, GameView};

    use super::*;

    const BLACK: Pixel = Pixel::Color(Rgb([0, 0, 0]));

    fn proposition_as_str(proposition: &Proposition) -> String {
        let mut str = String::new();
        for p in &proposition.line {
            match p {
                Some(Pixel::Cross) => {
                    str.push('X');
                },
                Some(Pixel::Color(color)) => {
                    if color.eq(&WHITE) {
                        str.push(' ');
                    }
                    else {
                        str.push('█');
                    }
                },
                None => {
                    str.push(' ');
                }
            }
        }
        str
    }

    #[test]
    fn it_solves_full_lines() {
        let game_res = Game::new("test/4x4-shuriken.png");
        assert!(&game_res.is_ok());
        let mut game = game_res.unwrap();
        let solver = Solver {
            algos: vec![Box::new(FullLine {})],
        };

        // Should return the 1st row
        let proposition = solver.solve(&mut game);
        assert!(proposition.is_some());
        let proposition = proposition.unwrap();
        assert_eq!(proposition.view, GameView::Row);
        assert_eq!(proposition.index, 0);
        assert_eq!(proposition_as_str(&proposition), "██X█");

        // partialy fill the 1st row
        game.board.set_pixel(0, 0, &BLACK);
        game.board.set_pixel(1, 0, &BLACK);

        // Should return the 1st row
        let proposition = solver.solve(&mut game);
        assert!(proposition.is_some());
        let proposition = proposition.unwrap();
        assert_eq!(proposition.view, GameView::Row);
        assert_eq!(proposition.index, 0);
        assert_eq!(proposition_as_str(&proposition), "██X█");

        // finish filling the 1st row
        game.board.set_pixel(3, 0, &BLACK);

        // Should return the 2nd row
        let proposition = solver.solve(&mut game);
        assert!(proposition.is_some());
        let proposition = proposition.unwrap();
        assert_eq!(proposition.view, GameView::Row);
        assert_eq!(proposition.index, 3);
        assert_eq!(proposition_as_str(&proposition), "█X██");
        
        // Fill the last row
        game.board.set_pixel(0, 3, &BLACK);
        game.board.set_pixel(2, 3, &BLACK);
        game.board.set_pixel(3, 3, &BLACK);

        // Should return the 1st col
        let proposition = solver.solve(&mut game);
        assert!(proposition.is_some());
        let proposition = proposition.unwrap();
        assert_eq!(proposition.view, GameView::Column);
        assert_eq!(proposition.index, 0);
        assert_eq!(proposition_as_str(&proposition), "█X██");

        // Fill de first col
        game.board.set_pixel(0, 2, &BLACK);

        // Should return the last col
        let proposition = solver.solve(&mut game);
        assert!(proposition.is_some());
        let proposition = proposition.unwrap();
        assert_eq!(proposition.view, GameView::Column);
        assert_eq!(proposition.index, 3);
        assert_eq!(proposition_as_str(&proposition), "██X█");

        // Fill de last col
        game.board.set_pixel(3, 1, &BLACK);

        // No more proposition
        assert!(solver.solve(&mut game).is_none());
    }
}
