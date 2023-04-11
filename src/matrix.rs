use rand::{thread_rng,Rng};
use crate::cells::Cell;
use crate::cells::CERO;
use crate::cursor;
use crate::cursor::Cursor;
use crate::linked::Linked;

pub struct Matrix {
    pub(crate) horizontal: Linked,
    pub(crate) vertical: Linked,

    pub(crate) access: Vec<Cell>,

    pub(crate) sizes: Vec<usize>,
    pub(crate) covered: Vec<usize>,
    pub(crate) total_covered: usize,
    pub(crate) total_uncovered: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        let mut matrix = Matrix {
            horizontal: crate::linked::Linked {
                links: Vec::with_capacity(cols + 1),
            },
            vertical: crate::linked::Linked {
                links: Vec::with_capacity(rows + 1),
            },
            sizes: Vec::new(),
            access: Vec::with_capacity(cols + 1),
            covered: Vec::new(),
            total_covered: 0usize,
            total_uncovered: 0usize,
        };
        assert_eq!(matrix.allocate_column(), CERO);
        for _ in 0..cols {
            matrix.add_column();
        }
        matrix
    }

    fn allocate_cell(&mut self, cell: Cell) -> Cell {
        self.access.push(cell);
        let x_cell = self.horizontal.new_cell();
        let y_cell = self.vertical.new_cell();
        assert_eq!(x_cell, y_cell);
        x_cell
    }
    fn allocate_column(&mut self) -> Cell {
        let cell = self.allocate_cell(CERO);
        self.access[cell] = cell;
        self.sizes.push(0);
        self.covered.push(0);
        cell
    }

    fn add_column(&mut self) -> Cell {
        let cell = self.allocate_column();

        // add column in last position
        self.horizontal.insert(self.horizontal[CERO].prev, cell);

        cell
    }
    pub fn add_row(&mut self, row: &Vec<usize>) {
        let mut col = CERO;

        let mut prev = None;
        for val in row {
            col = self.horizontal[col].next;
            if *val == 1usize {
                self.sizes[col] += 1;
                let new_cell = self.allocate_cell(col);
                //fetch column and add new cell as cols new previous cell
                self.vertical.insert(self.vertical[col].prev, new_cell);
                if let Some(prev) = prev {
                    self.horizontal.insert(prev, new_cell);
                }
                prev = Some(new_cell);
            }
        }
    }
    pub(crate) fn cover(&mut self, cell: Cell) {
        //remove acces cell front x axis list
        // println!("covering column: {:?}",cell);
        //self.total_covered+=1;
        self.horizontal.remove(cell);
        self.covered[cell] = 1;
        //get y axis cursor to iterate on rows to cover
        let mut cur = self.vertical.cursor(cell);
        //here we only cover the rows but we leave the covered columns nodes
        //for us to cover later outside of this function
        while let Some(c_axis_cell) = cur.next(&self.vertical) {
            let mut curr = self.horizontal.cursor(c_axis_cell);
            while let Some(r_axis_cell) = curr.next(&self.horizontal) {
                self.vertical.remove(r_axis_cell);
                self.sizes[self.access[r_axis_cell]] -= 1;
                //self.total_covered+=1;
                // println!("size for column {:?} reduced to : {}",self.access[r_axis_cell],self.sizes[self.access[r_axis_cell]]);
            }
        }
    }
    pub(crate) fn cover_all_row(&mut self, col: Cell) {
        let mut cursor = self.vertical.cursor(col);
        while let Some(cell) = cursor.next(&self.horizontal) {
            let header = self.access[cell];
            self.cover(header);
        }
    }

    pub(crate) fn uncover(&mut self, cell: Cell) {
        let mut cur = self.vertical.cursor(cell);
        while let Some(c_axis_cell) = cur.prev(&self.vertical) {
            let mut r_axis_cell = self.horizontal.cursor(c_axis_cell);
            while let Some(current_cell) = r_axis_cell.prev(&self.horizontal) {
                //self.total_uncovered +=1;
                self.sizes[self.access[current_cell]] += 1;
                self.vertical.add_back(current_cell);
            }
        } //self.total_uncovered+=1;
        self.covered[cell] = 0;
        self.horizontal.add_back(cell);
    }
    pub(crate) fn uncover_all_row(&mut self, cell: Cell) {
        let mut cursor = self.vertical.cursor(cell);
        while let Some(cell) = cursor.prev(&self.horizontal) {
            let header = self.access[cell];
            self.uncover(header)
        }
    }

    pub(crate) fn get_row(&self, cell: Cell) -> Vec<usize> {
        let mut cols_ind = Vec::new();

        let mut curr = self.horizontal.cursor(cell);
        cols_ind.push(self.access[cell].0);
        while let Some(current) = curr.next(&self.horizontal) {
            cols_ind.push(self.access[current].0);
        }
        cols_ind.sort();
        cols_ind
    }
    pub(crate) fn get_smallest(&self) -> Cell {
        let mut cursor = self.horizontal.cursor(CERO);
        let mut smallest_column = CERO;
        let mut changed:bool = false;
        let mut current_smallest_size = self.sizes.clone().into_iter().max().unwrap()+1;
        while let Some(node) = cursor.next(&self.horizontal) {
            if self.sizes[self.access[node]] < current_smallest_size {
                current_smallest_size = self.sizes[self.access[node]];
                smallest_column = node;
                if current_smallest_size == 1 {
                    return smallest_column;
                }
                changed = true;
            }if changed == false {
                let index: usize = rand::thread_rng().gen_range(1..self.sizes.len());
                return self.access[index]
            }
        }

        smallest_column
    }
}
