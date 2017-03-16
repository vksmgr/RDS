

pub fn run(){
    magic_trick();
}

//Magic Trick

fn magic_trick(){
    let test_case: usize = read!();
    for case in 0..test_case {
        logic(case);
    }
}

fn logic(case: usize){

    let ans_first: usize = read!();
    let mut count = 0;
    let mut number = 0;
    let mut first_arr = [[0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            first_arr[i][j] = read!();
        }
    }

    let ans_second: usize = read!();
    let mut second_arr =[[0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            second_arr[i][j] = read!();
        }
    }
    let mut new_array : [i32; 4] = [0; 4];
    for i in 0..4 {
        new_array[i] = second_arr[ans_second-1][i];
    }

    for i in 0..4 {
            if check(first_arr[ans_first-1][i],new_array){
                count = count + 1;
                number = first_arr[ans_first-1][i];
        }
    }
    if count == 1{
        println!("Case #{}: {}",case+1 , number);
    }
    if count > 1{
        println!("Case #{}: {}",case+1 , "Bad magician!");
    }
    if count == 0{
        println!("Case #{}: {}",case+1 , "Volunteer cheated!");
    }
}

fn check(num: i32,arr: [i32; 4] ) -> bool{
    for i in 0..4 {
        if num == arr[i]{
            return true;
        }
    }
    false
}