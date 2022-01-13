use crate::{board::Pixel, picross_image::Clue, Game, Board};

mod full_line;
use self::full_line::FullLine;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum GameView {
    Row,
    Column,
}

pub struct GameLine<'a> {
    view: GameView,
    board_line: Vec<&'a Pixel>,
    clues: &'a Vec<Clue>,
    index: usize,
}

pub struct Proposition {
    view: GameView,
    line: Vec<Option<Pixel>>,
    index: usize,
}

impl Proposition {
    pub fn new(pixels: Vec<Option<Pixel>>, line: &GameLine) -> Self {
        Proposition {
            view: line.view,
            line: pixels,
            index: line.index,
        }
    }

    pub fn count_pixel(&self) -> usize {
        self.line.iter().fold(0, |acc, p| {
            match p {
                Some(_) => acc + 1,
                None => acc
            }
        })
    }

    fn get_position(&self, index: usize) -> (usize, usize) {
        match self.view {
            GameView::Row => (index, self.index),
            GameView::Column => (self.index, index)
        }
    }

    pub fn merge(&self, board: &mut Board) {
        for (i, pixel_opt) in self.line.iter().enumerate() {
            if let Some(pixel) = pixel_opt {
                let (x, y) = self.get_position(i);
                board.set_pixel(x, y, pixel);
            }
        }
    }
}

pub trait SolverAlgo {
    #[deprecated]
    fn solve(&self, game: &mut Game) -> bool;

    fn get_proposition(&self, game_line: &GameLine) -> Option<Proposition>;
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
            algos: vec![Box::new(FullLine {})],
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

struct RowIterator<'a> {
    game: &'a Game,
    y: usize,
}

impl<'a> RowIterator<'a> {
    fn new(game: &'a Game) -> Self {
        RowIterator { game, y: 0 }
    }
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = GameLine<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.game.board.height() {
            return None;
        }
        let mut line = vec![];
        for x in 0..self.game.board.width() {
            line.push(self.game.board.get_pixel(x, self.y))
        }
        let clues = &self.game.image.rows[self.y];
        let index = self.y;
        self.y += 1;
        Some(GameLine {
            view: GameView::Row,
            board_line: line,
            clues,
            index,
        })
    }
}

struct ColumnIterator<'a> {
    game: &'a Game,
    x: usize,
}

impl<'a> ColumnIterator<'a> {
    fn new(game: &'a Game) -> Self {
        ColumnIterator { game, x: 0 }
    }
}

impl<'a> Iterator for ColumnIterator<'a> {
    type Item = GameLine<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.game.board.width() {
            return None;
        }
        let mut line = vec![];
        for y in 0..self.game.board.height() {
            line.push(self.game.board.get_pixel(self.x, y))
        }
        let clues = &self.game.image.cols[self.x];
        let index = self.x;
        self.x += 1;
        Some(GameLine {
            view: GameView::Column,
            board_line: line,
            clues,
            index,
        })
    }
}

impl Solver {
    pub fn solve(&self, game: &mut Game) -> Option<Proposition> {
        for algo in &self.algos {
            for row in RowIterator::new(game) {
                if let Some(proposition) = algo.get_proposition(&row) {
                    return Some(proposition);
                }
            }
            for col in ColumnIterator::new(game) {
                if let Some(proposition) = algo.get_proposition(&col) {
                    return Some(proposition);
                }
            }
        }
        None
    }
}
