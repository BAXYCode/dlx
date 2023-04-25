use crate::solver::Solver;
use crate::sudoku_gen::SudokuGenerator;
use crate::{cells::Cell, cells::CERO, matrix::Matrix};
use std::time::{Instant, SystemTime};
use radsort::sort_by_key;
const HARD_CODED_MAX: usize = 100_000;
pub struct Sudoku {
    pub(crate) sudoku: String,
    pub(crate) dimension: usize,
    pub(crate) solutions: usize,
    pub(crate) recursion_depth: usize,
    pub(crate) wanted_ans_num: WantedSolutions,
}

pub(crate) enum WantedSolutions {
    MaxWanted(usize),
    None,
}

impl Sudoku {
    pub fn new(sudoku: String, dimension: usize) -> Self {
        Self {
            sudoku,
            dimension,
            solutions: 0usize,
            recursion_depth: 0usize,
            wanted_ans_num: WantedSolutions::None,
        }
    }

    fn small_sudoku(&self, sparse: &mut Vec<Vec<usize>>) {
        let n = self.dimension * self.dimension;
        for (ind, mut val) in self.sudoku.char_indices() {
            if val == '.' {
                val = '0';
            }
            let val_copy = val.clone();
            let val_dig = val_copy.to_digit(10u32).unwrap();
            if (val.to_digit(10u32).unwrap() as usize) == 0usize {
                let mut temp = self.build_n_rows(n, ind);
                while let Some(individual) = temp.pop() {
                    sparse.push(individual);
                }
            } else {
                let temp = self.build_one_row(ind, val_dig as usize);

                let tempp: Vec<usize> = temp.into_iter().flatten().collect();
                sparse.push(tempp);
            }
        }
    }
    fn big_sudoku(&self, sparse: &mut Vec<Vec<usize>>) {
        let n = self.dimension * self.dimension;
        let sudoku_split: Vec<&str> = self.sudoku.split(".").collect();
        let mut sudoku_vec = Vec::with_capacity(n*n);
        for x in sudoku_split{
            sudoku_vec.push(x.parse::<usize>().unwrap());
        }
        for (ind, val) in sudoku_vec.iter().enumerate() {
            if *val == 0usize {
                let mut temp = self.build_n_rows(n, ind);
                while let Some(row) = temp.pop() {
                    sparse.push(row);
                }
            } else {
                let row = self.build_one_row(ind, *val);
                let mut temp: Vec<usize> = Vec::with_capacity(row[0].len() * 4);
                let flat_row = row.into_iter().flatten();
                temp.extend(flat_row);
                sparse.push(temp)
            }
        }
    }
    pub fn set_new_sudoku(&mut self, sudoku: &str) {
        self.solutions = 0usize;
        self.sudoku = sudoku.to_owned();
    }
    fn sudoku_to_sparse(&self) -> Vec<Vec<usize>> {
        let mut sparse: Vec<Vec<usize>> = Vec::new();
        if self.dimension < 4 {
            self.small_sudoku(&mut sparse);
        } else {
            self.big_sudoku(&mut sparse);
        }
        sparse
    }

    fn push_to_constraint_vec(&self, vect: &mut Vec<usize>, comparor: &usize, compare_to: &usize) {
        if comparor == compare_to {
            vect.push(1usize);
        } else {
            vect.push(0usize)
        }
    }

    fn get_index(&self, dim: usize, val: usize, constraint_ind: usize) -> usize {
        dim * (constraint_ind) + val
    }

    fn build_one_row(&self, ind: usize, val: usize) -> Vec<Vec<usize>> {
        //total number of cells in the sudoku puzzle
        let dimension = self.dimension * self.dimension * self.dimension * self.dimension;
        let n = self.dimension * self.dimension;
        let mut completed_row = Vec::new();
        //these are the "constraint indices" i.e. in which block of 1-9
        //the 1 should be placed to represent respecting a constraint
        //(let's say we have the first block of 81 columns represent the row
        //constraint, index 56 would mean that our possibility is located
        //in the 7th row)
        let col = ind % (n);
        let row = ind.div_floor(n);
        let box_con = (row - (row % self.dimension)) + (col.div_floor(self.dimension));
        //-----------------------------------------------------------------------------

        //this is the index of the 1 in inside of the specific block inside
        //the constraint.
        let row_index = self.get_index(n, val, row) - 1;
        let col_index = self.get_index(n, val, col) - 1;
        let box_index = self.get_index(n, val, box_con) - 1;
        let digit_index = ind;
        //-----------------------------------------------------------------------------
        let mut row_vec = Vec::with_capacity(dimension);
        let mut col_vec = Vec::with_capacity(dimension);
        let mut box_vec = Vec::with_capacity(dimension);
        let mut digit_vec = Vec::with_capacity(dimension);

        for i in 0..dimension {
            self.push_to_constraint_vec(&mut digit_vec, &i, &(&digit_index));
            self.push_to_constraint_vec(&mut row_vec, &i, &(&row_index));
            self.push_to_constraint_vec(&mut col_vec, &i, &(&col_index));
            self.push_to_constraint_vec(&mut box_vec, &i, &(&box_index));
        }

        completed_row.push(digit_vec);
        completed_row.push(row_vec);
        completed_row.push(col_vec);
        completed_row.push(box_vec);
        completed_row
    }

    fn build_n_rows(&self, dim: usize, cell_num: usize) -> Vec<Vec<usize>> {
        let mut result = Vec::new();

        for i in 1..=dim {
            let row = self.build_one_row(cell_num, i);
            let mut temp: Vec<usize> = Vec::with_capacity(row[0].len() * 4);
            let flat_row = row.into_iter().flatten();
            temp.extend(flat_row);
            result.push(temp);
        }
        result
    }

    fn decoder(&self, row: &Vec<usize>) -> (usize, usize) {
        let possibilities = self.dimension * self.dimension;
        let cell = row[0];
        let value = {
            let ind = row[1] - (possibilities * possibilities);
            let mut i = ind % possibilities;
            if i == 0 {
                i = possibilities
            }
            i
        };
        (cell, value)
    }
    fn ans_to_sudoku_ans(&self, answers: &Vec<Vec<Vec<usize>>>, length: usize) -> Vec<String> {
        let mut sudokus: Vec<String> = Vec::new();
        for answer in answers {
            let mut sudoku = String::new();
            let mut decoded: Vec<(usize, usize)> = answer
                .into_iter()
                .map(|row| self.decoder(row))
                .collect::<Vec<(usize, usize)>>();
            radsort::sort_by_key(&mut decoded, |vals| vals.0);
            for (i,x) in decoded.into_iter().enumerate(){
                
                sudoku.insert(i,char::from_digit(x.1 as u32,10).unwrap());
            }
            sudokus.push(sudoku);
        }
        sudokus
    }
    pub(crate) fn time_to_solve(
        &mut self,
        f: fn(&mut Sudoku, Option<usize>) -> Option<Vec<String>>,
        ans_wanted: Option<usize>,
    ) -> Option<Vec<String>> {
        let start = SystemTime::now();
        let res = f(self, ans_wanted);

        println!(
            "sovlvable {}",
            SudokuGenerator::new(3usize).generate_solvable()
        );
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        println!("execution time in microseconds: {}", duration.as_micros());
        res
    }
}
impl Solver for Sudoku {
    fn solver(&mut self, answers_wanted: Option<usize>) -> Option<Vec<String>> {
        let length = self.sudoku.len() as f64;

        let sparse = self.sudoku_to_sparse();

        let possibility = self.dimension * self.dimension;
        let rows = possibility * possibility * possibility;
        let cols = possibility * possibility * 4;
        let mut matrix = Matrix::new(rows, cols);

        for x in sparse.iter() {
            // println!("this is row#{}: {:?}",i, x);
            matrix.add_row(&x);
        }
        if let Some(val) = answers_wanted {
            self.wanted_ans_num = WantedSolutions::MaxWanted(val);
        }

        let mut partial_ans = Vec::new();
        let mut answers = Vec::new();
        self.solve(&mut matrix, &mut partial_ans, &mut answers);
        if answers.len() == 0 {
            println!("no answers");
            return None;
        }
        let result: Vec<String> = self.ans_to_sudoku_ans(&answers, length as usize);
        return Some(result);
    }
    fn solve(
        &mut self,
        matrix: &mut Matrix,
        partials: &mut Vec<Cell>,
        ans: &mut Vec<Vec<Vec<usize>>>,
    ) {
        //println!("recursion depth: {}", self.recursion_depth);
        self.recursion_depth += 1;
        //check if matrix is empty
        let mut curr = matrix.horizontal.cursor(CERO);
        if curr.next(&matrix.horizontal) == None {
            let answer = partials
                .iter()
                .map(|cell| matrix.get_row(*cell))
                .collect::<Vec<Vec<usize>>>();
            ans.push(answer);
            self.solutions += 1;
            return;
        }
        if let WantedSolutions::MaxWanted(val) = self.wanted_ans_num {
            if val < self.solutions {
                return;
            }
        }
        if self.solutions > HARD_CODED_MAX {
            return;
        }
        // println!("covered: {:?}",matrix.covered);
        // println!("partials: {:?}", partials);
        // println!("sizes : {:?}", matrix.sizes);
        //find the column with the lowest amount of 1s

        let col = matrix.get_smallest();
        // println!("cell: {:?} covered",col);
        matrix.cover(col);
        let mut curr = matrix.vertical.cursor(col);
        while let Some(curr) = curr.next(&matrix.vertical) {
            partials.push(curr);
            // println!("first cell of row to cover: {:?} cell in col:{:?} ",curr,matrix.access[curr]);
            matrix.cover_all_row(curr);
            // println!("total nodes covered {}",matrix.total_covered);
            self.solve(matrix, partials, ans);
            matrix.uncover_all_row(curr);
            partials.pop();
        }
        matrix.uncover(col);
        //println!(" total uncover ops: {}",matrix.total_uncovered);
        // println!("total operations: {}",matrix.total_covered+matrix.total_uncovered);
    }
}
