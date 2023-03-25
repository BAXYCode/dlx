#![allow(unused)]
#![feature(int_roundings)]

use std::itertools::*;

struct Sudoku{
    sudoku:Vector<usize>,
    dimension:usize,}

impl  Sudoku{

    fn sudoku_to_sparse() -> Vec<Vec<usize>>{
        let mut sparse = Vec::new();
        
        for (ind, val)  in  enumerate(self.sudoku){
        
            if val == 0{
            
                let mut temp = self.build_nine_rows(ind);
                temp.iter.map(|item|sparse.push(item));
                
            }else{
            
                let temp = self.build_one_row(ind,val);
                sparse.push(temp);
                
            }
        
        }
    sparse}
    
    fn get_index(dim:usize, val:usize, constraint_ind:usize) -> usize{

        dim*(constraint_ind-1)+val

    } 

    fn build_one_row(ind:usize,val: usize) -> Vecusize{

        let dimension = 
        self.dimension*self.dimension*self.dimension*self.dimension;
        let n = self.dimension*self.dimension;
        let mut completed_row = Vec::new();

        let mut  col = ind%(self.dimension*self.dimension);
        if col == 0{
            col = 9;}

        let row = ind.div_ceil(self.dimension*self.dimension);
        let box = (row-(row%self.dimension))+row.div_ceil(self.dimension);

        let row_index = self.get_index(n,val,row) ;
        let col_index = self.get_index(n,val,col);
        let box_index = self.get_index(n,val,box);
        let digit_index = get_index(n,val,ind);

        let mut row_vec = Vec::new();
        let mut col_vec = Vec::new();
        let mut box_vec = Vec::new();
        let mut digit_vec = Vec::new();

        for i in 0..dimension{

            if i == col_index{
                col_vec.push(1usize);
            }else{col_vec.push(0usize);} 
            if i == row_index{
                row_vec.push(1usize);}
            else{row_vec.push(0usize);}
            if i == box_index{box_vec.push(1usize);}
            else{box_vec.push(0usize);}
            if i == digit_index{digit_vec.push(1usize);}
            else{digit_vec.push(0usize);}
                
            }
        completed_row.push(digit_vec);
        completed_row.push(row_vec);
        completed_row.push(col_vec);
        completed_row.push(box_vec);
        completed_row
        }
       
        fn build_nine_rows(dim: usize){

        let mut result = Vec::new();

        for i in 1..=dim*dim{
            let temp = build_one_row(dim,i);
            result.push(temp);
        
        }
        result
    }

}
