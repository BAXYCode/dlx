use std::fs::File;
use std::io::{BufReader,Read};
use std::path::Path;
use crate::solver::Solver;
use crate::sudoku::{Sudoku,WantedSolutions};
use crate::utils;
pub(crate)  struct  SudokuTester{
    pub response: Vec<String>
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
        let path = Path::new("data.txt");
        let mut file = File::open(path).unwrap();
        let mut  bufreader = String::new();
        let contents = file.read_to_string(&mut bufreader);
        if let Ok(contents) = contents{
            return bufreader;}
        "None".to_owned()
            }

    pub(crate)fn  test_performance(&mut self){
        let mut result: Vec<String> = Vec::new();
        let mut treated = self.treat_file();
        let mut unsolved = self.get_sudoku(&treated);
        let total = unsolved.len() as f32;
        let mut sudoku_struct = Sudoku{sudoku:"".to_owned(),dimension:3usize,solutions:0usize,recursion_depth:0usize,wanted_ans_num: WantedSolutions::None,} ;
        for (i ,sudoku) in unsolved.iter().enumerate(){
            if (i+1)%1000 == 0{
                println!("solved: {}, completed percentage: {}%",i+1,((i+1) as f32)/total*100f32);
            }
            sudoku_struct.set_new_sudoku(&sudoku);
            let solved = sudoku_struct.solver(None);
            if let Some(solved) = solved{
                let solved_str = utils::vec_to_sudoku_string(&solved[0]);
                result.push(solved_str);
            }
        }
        self.response = result;
    }
         
            
        



}
