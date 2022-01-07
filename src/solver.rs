use crate::game::Game;
use self::full_line::{FullRow, FullCol};

mod full_line;

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
