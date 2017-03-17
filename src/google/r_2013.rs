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

    println!("");
    for i in 0..4 {
        print!("row  {} ", check_row(input, i));

        print!("column  {} ", check_col(input, i));

        print!("column  {} ", check_diagonal(input));

        break;
    }
    println!("");
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

