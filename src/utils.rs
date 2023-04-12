pub(crate) fn vec_to_sudoku_string<T: ToString>(sudoku_vec: Vec<T>) -> String{

    sudoku_vec
        .into_iter()
        .map(|v| v.to_string())
        .fold(String::new(),|mut a,b|a.to_owned()+&b)
}
