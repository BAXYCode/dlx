use crate::{sudoku::{Sudoku, self}, solver::Solver};
use rand::{thread_rng, Rng, seq::SliceRandom};
use crate::utils;
pub(crate) struct SudokuGenerator{
    
    generated_sudokus: Vec<String>,
    blank: Sudoku,
    dimension: usize
}

impl SudokuGenerator {
   
    pub fn new(size : usize) -> Self{
        Self{
            generated_sudokus:Vec::new(),
            blank: {
                let sudoku= {
                        let sudoku: String = utils::vec_to_sudoku_string(&vec![0;size*size*size*size]);
                       let  returned_sudoku = Sudoku::new(sudoku.as_str(),size);
                        returned_sudoku
                    };
                sudoku
            },dimension:size
        }
    }
    pub fn generate(&mut self) ->Option<Vec<Vec<usize>>>{
       
        let sudokus = self.blank.solver(None);
        if let Some(sudokus) = sudokus{
        return Some(sudokus)
        }
        None
    }

    pub fn generate_one_full(&mut self) -> Option<String>{
        let sudokus = self.generate();
        let mut rng = thread_rng();
        if let Some(mut sudokus) = sudokus{
            
            let picked_board = sudokus.remove(rng.gen_range(0..sudokus.len()));
            return Some(utils::vec_to_sudoku_string(&picked_board))
        }
        return None 
    }


    pub fn generate_solvable(&mut self) -> Vec<usize>{
        let mut fails = 0;
        let posibilities = self.dimension * self.dimension;
        let mut picks: Vec<usize> = (0usize..posibilities*posibilities as usize).collect();
        let mut rng = thread_rng();
        let sudoku = self.generate_one_full().unwrap();
        let mut last_sudoku = Vec::new();
        let mut  sudoku_to_return = sudoku::Sudoku::new(sudoku.as_str(), self.dimension);
        picks.shuffle(&mut rng);        
        
        let first_sudoku = utils::string_to_sudoku_vec(sudoku.as_str());
       
        let mut curr_sudoku = utils::string_to_sudoku_vec(sudoku.as_str());
        while let Some(pick) = picks.pop(){
                curr_sudoku[pick] = 0usize;
                sudoku_to_return.set_new_sudoku(utils::vec_to_sudoku_string(&curr_sudoku));
                if fails >=10000{
                    return curr_sudoku
                }
                let result = sudoku_to_return.solver(None);
            if let Some( result) = result{
                last_sudoku =utils::string_to_sudoku_vec(sudoku_to_return.sudoku.as_str());
                if result.len()==1{
                    println!("current sudoku: {:?}", curr_sudoku );
                    let mut possible_good_grid = curr_sudoku.clone();
                    possible_good_grid.shuffle(&mut rng);
                    sudoku_to_return.set_new_sudoku(utils::vec_to_sudoku_string(&possible_good_grid));
                    if let Some(checked) = sudoku_to_return.solver(None){
                        if result.len() == 1 {
                            curr_sudoku = utils::string_to_sudoku_vec(sudoku_to_return.sudoku.as_str());

                        }else{curr_sudoku = last_sudoku; fails+=1}
                    }else {
                        curr_sudoku = last_sudoku;
                        fails +=1;
                    }   
                }else {
                    curr_sudoku = last_sudoku;
                    fails +=1
                }
                
            }  
        }
        curr_sudoku
    }
}
