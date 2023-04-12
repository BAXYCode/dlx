pub(crate) fn vec_to_sudoku_string<T: ToString>(sudoku_vec: &Vec<T>) -> String{

   let to_return = sudoku_vec
                    .into_iter()
                    .map(|v| v.to_string())
                    .fold(String::new(),|a,b|a.to_owned()+&b);
    to_return
}

pub(crate) fn string_to_sudoku_vec(sudoku_str: &str) -> Vec<usize>{

    sudoku_str.chars()
        .map(|v|v.to_digit(10).unwrap() as usize)
        .collect()
}
