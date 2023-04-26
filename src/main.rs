
#![recursion_limit = "1000000"]
#![allow(dead_code)]
#![feature(int_roundings)]
use rand::{thread_rng,Rng};
use solver::Solver;
use sudoku_tester::SudokuTester;
use std::{io,env, time::{SystemTime, Instant}};
mod cursor;
mod solver;
mod linked;
mod cells;
mod matrix;
mod sudoku;
mod sudoku_gen;
mod utils;
mod sudoku_tester;
use crate::sudoku::{Sudoku};

fn main(){       
     let start = Instant::now();

    let mut _test = SudokuTester{response: Vec::new()};

    _test.test_performance();
   
    let duration = start.elapsed();
    println!("time elapsed: {:?}",duration);
    
    println!("puzzles solved {:?}", _test.response.len());

   
}



fn djfkh(){    env::set_var("RUST_BACKTRACE", "1");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("give valid sudoku");
    let split :Vec<&str> = input.split_whitespace().collect();
    let mut sudoku_problem = Sudoku::new(split[0].to_owned(),split[1].parse::<usize>().unwrap());
    let res: Option<Vec<String>> ;
        if split.len()>2{
            res = sudoku_problem.time_to_solve(Sudoku::solver,Some(split[2].parse::<usize>().unwrap()));}
    else{
        res = sudoku_problem.time_to_solve(Sudoku::solver, None);}
    if let Some(res) = res{
        println!("found {} solutions to the problem",sudoku_problem.solutions);
        let index = thread_rng().gen_range(0..res.len());    
            println!("{:?}",res[index]);
    }
} 
