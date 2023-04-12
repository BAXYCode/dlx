use crate::matrix::Matrix;
use crate::cells::Cell;

pub trait Solver {
    fn solver(&mut self) -> Option<Vec<Vec<usize>>>;
    fn solve(
        &mut self,
        matrix: &mut Matrix,
        partials: &mut Vec<Cell>,
        ans: &mut Vec<Vec<Vec<usize>>>,
    );
}


