
#![recursion_limit = "1000000"]
#![feature(int_roundings)]
use rand::{thread_rng,Rng};
use std::{io, collections};
mod cursor;
mod linked;
mod cells;
mod matrix;
mod sudoku;
use crate::sudoku::Sudoku;
use std::env;

fn main(){
env::set_var("RUST_BACKTRACE", "1");
let mut input = String::new();
io::stdin().read_line(&mut input).expect("give valid sudoku");
let split :Vec<&str> = input.split_whitespace().collect();
let mut sudoku_problem = Sudoku::new(split[0],split[1].parse::<usize>().unwrap(),split[2].parse::<usize>().unwrap());
let res = sudoku_problem.time_to_solve(Sudoku::solver);
if let Some(res) = res{
    println!("found {} solutions to the problem",sudoku_problem.solutions);
    let index = thread_rng().gen_range(1..res.len());    
    //for x in 0..sudoku_problem.solutions{
        //if x >10{break;}
        println!("{:?}",res[index]);
     //   }
    }
}
  
