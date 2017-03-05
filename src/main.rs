#[macro_use]
extern crate text_io;

fn main() {
    //let i: i32;
    let n: i32;
    let mut process: [i32; 100] =[0; 100];
    let mut idea_process:[i32; 100] = [0; 100];
    let mut count: i32;
    let mut tp: i32;

    n = read!();
    for i in 0..n as usize  {
        process[i] = read!();
    }

    for i in 0..n as usize  {
        idea_process[i] = read!();
    }
    rotate(&mut process, n as usize);

    for i in 0..n {
        if process[i] == idea_process[i] {
           count= count + 1;
        }else {
            while process[tp] != idea_process[tp] {
                process = rotate(process,n);
            }
        }
    }


    for i in 0..n as usize {
        println!(" process : {}  idealProcess : {}\n", process[i], idea_process[i]);
    }

}

fn rotate(arr: &mut[i32], len: usize) ->&mut[i32]{
    let mut temp;
    let mut count: usize = 0;
    temp = arr[0];
    for i in 0..len {
        arr[i] = arr[i+1];
        count = count + 1;
    }
    arr[len - 1] = temp;
    arr
}