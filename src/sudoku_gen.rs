use crate::{sudoku::Sudoku, solver::Solver};
use rand::{thread_rng, Rng};
use crate::utils;
pub(crate) struct SudokuGenerator{
    
    generated_sudokus: Vec<String>,
    blank: Sudoku
}

impl SudokuGenerator {
   
    pub fn new(size : usize) -> Self{
        Self{
            generated_sudokus:Vec::new(),
            blank: {
                let sudoku= {
                        let sudoku: String = utils::vec_to_sudoku_string(vec![0;size*size*size*size]);
                       let  returned_sudoku = Sudoku::new(sudoku.as_str(),size);
                        returned_sudoku
                    };
                sudoku
            }
        }
    }
    #[allow(dead_code)]
    pub(crate) fn generate(&mut self) ->Option<String>{
       
        let sudokus = self.blank.solver(None);
        let mut rng = thread_rng();
        if let Some(mut sudokus) = sudokus{
            
            let picked_board = sudokus.remove(rng.gen_range(0..sudokus.len()));
            return Some(utils::vec_to_sudoku_string(picked_board))
        }
        return None 
    }
    

}
