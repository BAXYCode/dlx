use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell(pub usize);

pub const CERO: Cell = Cell(0);

impl ops::Index<Cell> for Vec<Cell> {
    type Output = Cell;

    fn index(&self, cell: Cell) -> &Cell {
        &self[cell.0]
    }
}
impl ops::IndexMut<Cell> for Vec<Cell> {
    fn index_mut(&mut self, cell: Cell) -> &mut Cell {
        &mut self[cell.0]
    }
}
impl ops::Index<Cell> for Vec<usize> {
    type Output = usize;
    fn index(&self, index: Cell) -> &usize {
        &self[index.0]
    }
}

impl ops::IndexMut<Cell> for Vec<usize> {
    fn index_mut(&mut self, index: Cell) -> &mut usize {
        &mut self[index.0]
    }
}
