pub fn run() {
    alien_language();
}

// Alien Language

fn alien_language() {
    //local variables
    let  l: i32 = read!();
    let  d: i32 = read!();
    let no_test_cases: i32 = read!();   //inititaly no test case

    for i in 0..no_test_cases {
        logic(l, d, i);
    }
}

fn logic(l: i32, d: i32, test_case: i32) {
    //input


    println!("L : {}", l);
    println!("D : {}", d);
    println!("N : {}", test_case);

}