use crate::utils;
use crate::{
    solver::Solver,
    sudoku::{self, Sudoku},
};
use rand::{seq::SliceRandom, thread_rng, Rng};
pub(crate) struct SudokuGenerator {
    generated_sudokus: Vec<String>,
    blank: Sudoku,
    dimension: usize,
}

impl SudokuGenerator {
    pub fn new(size: usize) -> Self {
        Self {
            generated_sudokus: Vec::new(),
            blank: {
                let sudoku = {
                    let sudoku: String =
                        utils::vec_to_sudoku_string(&vec![0; size * size * size * size]);
                    let returned_sudoku = Sudoku::new(sudoku, size);
                    returned_sudoku
                };
                sudoku
            },
            dimension: size,
        }
    }
    pub fn generate(&mut self) -> Option<Vec<Vec<usize>>> {
        let sudokus = self.blank.solver(None);
        if let Some(sudokus) = sudokus {
            return Some(sudokus);
        }
        None
    }
    pub fn generate_one_full(&mut self) -> Option<String> {
        return Some(
            "987654312312987645654213987798362451145798263263145790879536124421879536536421879"
                .to_owned(),
        );
    }
    //    pub fn generate_one_full(&mut self) -> Option<String> {
    //        let sudokus = self.generate();
    //        let mut rng = thread_rng();
    //        if let Some(mut sudokus) = sudokus {
    //            let picked_board = sudokus.remove(rng.gen_range(0..sudokus.len()));
    //            return Some(utils::vec_to_sudoku_string(&picked_board));
    //        }
    //        return None;
    //    }

    pub fn generate_solvable(&mut self) -> String {
        let posibilities = self.dimension * self.dimension;
        let mut picks: Vec<usize> = (0usize..posibilities * posibilities as usize).collect();
        let mut rng = thread_rng();
        let sudoku = self.generate_one_full().unwrap();
        let mut last_sudoku = "".to_owned();
        let mut sudoku_to_return = sudoku::Sudoku::new(sudoku.clone(), self.dimension);
        picks.shuffle(&mut rng);

        let mut curr_sudoku = sudoku;
        while let Some(pick) = picks.pop() {
            if curr_sudoku.chars().filter(|v| *v == '0').count() >= 64usize {
                return curr_sudoku;
            }

            curr_sudoku.replace_range(pick..pick + 1, "0");
            picks.shuffle(&mut rng);
            sudoku_to_return.set_new_sudoku(&curr_sudoku);

            let result = sudoku_to_return.solver(Some(2usize));
            if let Some(result) = result {
                curr_sudoku = sudoku_to_return.sudoku.clone();
                if result.len() == 1 {
                    last_sudoku = curr_sudoku.clone();
                } else {
                    curr_sudoku = last_sudoku.clone();
                }
            }
        }
        curr_sudoku
    }
}
