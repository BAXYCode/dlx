use std::fs::File;
use std::io::{BufReader,Read};
use crate::solver::Solver;
use crate::sudoku::{Sudoku,WantedSolutions};
use crate::utils;
pub(crate)  struct  SudokuTester{
    response: Vec<String>
}

impl SudokuTester {

    pub fn get_sudoku(&self,sudokus: &str) -> Vec<String>{

    let sudokus_vec: Vec<String> = { 
        let untreated = sudokus.split("\n");
        let mut treated:Vec<String> = Vec::new();
        for each in untreated{
            treated.push(each.to_owned().trim_end().to_owned());
        }
        treated
    };
    sudokus_vec
}
    fn treat_file(&mut self)-> String{

        let mut file = File::open("data.txt").unwrap();
        let mut  bufreader = BufReader::new(file);
        let mut contents = String::new();
        contents = bufreader.read_to_string(&mut contents);
        Ok(contents)
    }

    pub(crate)fn  test_performance(&mut self) -> Vec<String>{
        let mut result: Vec<String> = Vec::new();
        let mut treated = self.treat_file();
        let mut unsolved = self.get_sudoku(&treated);
        let mut sudoku_struct = Sudoku{sudoku:"".to_owned(),dimension:3usize,solutions:0usize,recursion_depth:0usize,wanted_ans_num: WantedSolutions::None,} ;
        for sudoku in unsolved{
            sudoku_struct.set_new_sudoku(&sudoku);
            let solved = sudoku_struct.solver(None);
            if let Some(solved) = solved{
                let solved_str = utils::vec_to_sudoku_string(&solved[0]);
                result.push(solved_str);
            }
        }
        self.response = result.clone();
    result
    }
         
            
        



}
