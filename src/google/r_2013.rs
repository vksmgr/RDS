use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn run() {
    tic_tac_toe();
    //get_ip();
}

//Problem A. Tic-Tac-Toe-Tomek
fn tic_tac_toe() {
    let test_case: usize = read!();
    for case in 0..test_case {
        logic(case);
    }
}

fn logic(case: usize) {
    let mut input: [[char; 4]; 4] = [['.'; 4]; 4];
    let mut string: Vec<String> = Vec::new();
    /*for _ in 0..4 {
        string.push(read!("{}\n"));
    }*/

    for _ in 0..5 {
        string.push(read!("{}\n"));
    }

    for i in 0..5 {
        let mut g = 0;
        for charer in string[i].chars() {
            input[i][g] = charer;
            g = g + 1;
        }
    }

    for i in 0..4 {

        if check_row(input, i) == 1 || check_col(input, i) == 1 || check_antidiagonal(input) == 1 || check_diagonal(input) == 1{
            println!("Case #{}: {}",case+1,"X won");
            break;
        }
        if check_row(input, i) == 0 || check_col(input, i) == 0 || check_antidiagonal(input) == 0 || check_diagonal(input) == 0 {
            println!("Case #{}: {}",case+1,"O won");
            break;
        }else if check_row(input, i) == -1 || check_col(input, i) == -1 || check_antidiagonal(input) == -1 || check_diagonal(input) == -1 {
            if is_complete(input){
                println!("Case #{}: {}",case+1,"Game has not completed");
            }else {
                println!("Case #{}: {}",case+1,"Draw");
            }
            break;
        }

    }
}

fn check_row(input: [[char; 4]; 4], c: usize) -> i32 {
    let mut X: usize = 0;
    let mut T: usize = 0;
    let mut O: usize = 0;
    for i in 0..4 {
        if input[c][i] == 'X' {
            X = X + 1;
        } else if input[c][i] == 'T' {
            T = T + 1;
        } else if input[c][i] == 'O' {
            O = O + 1;
        }
    }
    if X == 4 || X == 3 && T == 1 {
        return 1;
    } else if O == 4 || O == 3 && T == 1 {
        return 0;
    } else {
        -1
    }
}

fn check_col(input: [[char; 4]; 4], c: usize) -> i32 {
    let mut X: usize = 0;
    let mut T: usize = 0;
    let mut O: usize = 0;
    for i in 0..4 {
        if input[i][c] == 'X' {
            X = X + 1;
        } else if input[i][c] == 'T' {
            T = T + 1;
        } else if input[i][c] == 'O' {
            O = O + 1;
        }
    }
    if X == 4 || X == 3 && T == 1 {
        return 1;
    } else if O == 4 || O == 3 && T == 1 {
        return 0;
    } else {
        -1
    }
}

fn check_diagonal(input: [[char; 4]; 4]) -> i32 {
    let mut X: usize = 0;
    let mut T: usize = 0;
    let mut O: usize = 0;
    for i in 0..4 {
        for j in 0..4 {
            if i == j {
                if input[i][j] == 'X' {
                    X = X + 1;
                } else if input[i][j] == 'T' {
                    T = T + 1;
                } else if input[i][j] == 'O' {
                    O = O + 1;
                }
            }
        }
    }

    if X == 4 || X == 3 && T == 1 {
        return 1;
    } else if O == 4 || O == 3 && T == 1 {
        return 0;
    } else {
        -1
    }
}


fn check_antidiagonal(input: [[char; 4]; 4]) -> i32 {
    let mut X: usize = 0;
    let mut T: usize = 0;
    let mut O: usize = 0;
    for i in 0..4 {
        for j in 0..4 {
            if i == j {
                if input[i][3-j] == 'X' {
                    X = X + 1;
                } else if input[i][3-j] == 'T' {
                    T = T + 1;
                } else if input[i][3-j] == 'O' {
                    O = O + 1;
                }
            }
        }
    }

    if X == 4 || X == 3 && T == 1 {
        return 1;
    } else if O == 4 || O == 3 && T == 1 {
        return 0;
    } else {
        -1
    }
}


fn is_complete(input: [[char; 4]; 4]) -> bool {
    let mut dot : usize = 0;
    for i in 0..4 {
        for j in 0..4 {
            if input[i][j] == '.' {
                dot = dot + 1;
            }
        }
    }
    if dot > 0{
        return true;
    }
    false
}