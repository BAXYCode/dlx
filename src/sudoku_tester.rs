use crate::solver::Solver;
use crate::sudoku::Sudoku;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::thread::{self, JoinHandle};
pub(crate) struct SudokuTester {
    pub response: Vec<String>,
}

impl SudokuTester {
    pub fn get_sudoku(&self, sudokus: &str) -> Vec<String> {
        let sudokus_vec: Vec<String> = {
            let untreated = sudokus.split("\n");
            let mut treated: Vec<String> = Vec::new();
            for each in untreated {
                treated.push(each.to_owned().trim_end().to_owned());
            }
            treated
        };
        sudokus_vec
    }
    fn treat_file(&mut self) -> String {
        let path = Path::new("data.txt");
        let mut file = File::open(path).unwrap();
        let mut bufreader = String::new();
        let ok = file.read_to_string(&mut bufreader);
        if let Ok(_ok) = ok {
            return bufreader;
        }
        "None".to_owned()
    }

    pub(crate) fn test_performance(&mut self) {
        let mut results: Vec<String> = Vec::new();
        let treated = self.treat_file();
        let unsolved = self.get_sudoku(&treated);
        let mut handles: Vec<JoinHandle<_>> = Vec::with_capacity(1000);
        for (i, sud) in unsolved.into_iter().enumerate() {
            let mut sudoku = Sudoku::new(sud, 3usize);
            if (i + 1) % 5000 == 0 {
                for handle in handles {
                    let answer = handle.join().unwrap();
                    results.push(answer);
                }
                handles = Vec::new();
            }

            let handle = thread::spawn(move || -> String {
                let answer = sudoku.solver(None);
                if let Some(answer) = answer {
                    return answer[0].clone();
                }
                return "".to_owned();
            });
            handles.push(handle);
        }
        for handle in handles {
            let answer = handle.join().unwrap();
            results.push(answer);
        }

        self.response = results;
    }
}
