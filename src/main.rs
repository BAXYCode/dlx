
#![recursion_limit = "1000000"]
#![allow(dead_code)]
#![feature(int_roundings)]
use rand::{thread_rng,Rng};
use solver::Solver;
use std::{io,env};
mod cursor;
mod solver;
mod linked;
mod cells;
mod matrix;
mod sudoku;
mod sudoku_gen;
mod utils;
use crate::sudoku::{Sudoku};

fn main(){
    env::set_var("RUST_BACKTRACE", "1");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("give valid sudoku");
    let split :Vec<&str> = input.split_whitespace().collect();
    let mut sudoku_problem = Sudoku::new(split[0].to_owned(),split[1].parse::<usize>().unwrap());
    let mut res: Option<Vec<Vec<usize>>> =None ;
        if split.len()>2{
            res = sudoku_problem.time_to_solve(Sudoku::solver,Some(split[2].parse::<usize>().unwrap()));}
    else{
        res = sudoku_problem.time_to_solve(Sudoku::solver, None);}
    if let Some(res) = res{
        println!("found {} solutions to the problem",sudoku_problem.solutions);
        let index = thread_rng().gen_range(0..res.len());    
        //for x in 0..sudoku_problem.solutions{
            //if x >10{break;}
            println!("{:?}",res[index]);
     // }
    }
    
}

fn get_sudoku(sudokus: &str) -> Vec<String>{

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

  
