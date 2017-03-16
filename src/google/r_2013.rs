
pub fn run(){
 tic_tac_toe();
}

//Problem A. Tic-Tac-Toe-Tomek
fn tic_tac_toe(){
    let test_case: usize = read!();
    for case in 0..test_case {
        logic(case);
    }
}

fn logic(case: usize){
    let mut string: Vec<Vec<char>> = Vec::new();
    for _ in 0..4 {
        string.push(read!("{}\n"));
    }
    //let mut input_mtx: [[char; 4]; 4] ;    //initialisation of the matrix
    let mut input: Vec<Vec<char>> = Vec::new();

    for i in 0..4 {
        for j in 0..4 {
            println!("{}", string[i][j]);
        }
    }
    //println!("Case #{}: {}",case+1, string[i]);

}