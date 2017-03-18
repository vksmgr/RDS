use std::collections::HashMap;

pub fn run() {
    speaking_in_tongues();
}

//Speaking in Tongues

fn speaking_in_tongues() {
    let test_case: usize = read!();

    for case in 0..test_case {
        logic(case);
    }
}

fn logic(case: usize) {
    //get input

    let mut input_string: Vec<String> = Vec::new();
    let err = &'0';

    let mut dic = HashMap::new();
    dic.insert('a', 'y');
    dic.insert('b', 'h');
    dic.insert('c', 'e');
    dic.insert('d', 's');
    dic.insert('e', 'o');
    dic.insert('f', 'c');
    dic.insert('g', 'v');
    dic.insert('h', 'x');
    dic.insert('i', 'd');
    dic.insert('j', 'u');
    dic.insert('k', 'i');
    dic.insert('l', 'g');

    dic.insert('m', 'l');
    dic.insert('n', 'b');
    dic.insert('o', 'k');
    dic.insert('p', 'r');

    dic.insert('q', 'z');

    dic.insert('r', 't');
    dic.insert('s', 'n');
    dic.insert('t', 'w');
    dic.insert('u', 'j');
    dic.insert('v', 'p');
    dic.insert('w', 'f');
    dic.insert('x', 'm');
    dic.insert('y', 'a');

    dic.insert('z', 'q');

    dic.insert(' ', ' ');

    println!("");
    input_string.push(read!("{}\n"));

    print!("Case #{}: ",case+1);
    for i in 0..input_string.len() {
        for character in input_string[i].chars() {
            print!("{}",
                   match dic.get(&character) {
                       Some(ch) => ch,
                       None => err,
                   });
        }
    }
}