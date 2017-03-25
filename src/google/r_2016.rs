pub fn run() {
    test_cases();
}


fn test_cases() {
    let test_cases: i32 = read!();

    for current_case in 0..test_cases as usize {
        //count_sheep(current_case)
        println!("");
        last_word(current_case);
    }
}


//Qualification Round : https://code.google.com/codejam/contest/6254486/dashboard
//Counting Sheep
fn count_sheep(current_case: usize) {
    let mut sleep: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut sheep: Vec<i32> = Vec::new();
    let mut count: i32 = 1;
    let mut flag: bool = true;
    let mut index: i32;
    let mut number: i32 = read!();
    let mut current_number = number;
    let mut backup: i32 = current_number;
    if number == 0 {
        println!("Case #{}: {}", current_case + 1, "INSOMNIA");
    } else {
        while flag {
            number = current_number * count;
            backup = number;
            while number > 0 {
                index = number % 10;

                if sheep.contains(&index) {} else {
                    sheep.push(index);
                    sheep.sort();
                }
                if sleep == sheep {
                    println!("Case #{}: {}", current_case + 1, backup);
                    flag = false;
                    break;
                }
                number = number / 10;
            }
            count = count + 1;
        }
    }
}

//Round 1A 2016 : https://code.google.com/codejam/contest/4304486/dashboard
//The last word.
fn last_word(case: usize) {
    let s: String = read!("{}");
    let mut last_char: char = ' ';
    let mut count: usize = 0;
    let mut char_print: Vec<char> = Vec::new();

    // Copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = s.chars().collect();
    let mut tesmp: Vec<char> = s.chars().collect();
    chars.sort();
    chars.dedup();
    let len = chars.len();
    last_char = chars[len-1];
    let len = tesmp.len();
    for i in 0..len {
        if tesmp[i] == last_char{
            count = count + 1;
          // println!("Count {}", count);
        }else {
            //println!("{}", chars[i]);
            char_print.push(tesmp[i]);
        }
    }

    //printing the out put :
    print!("Case #{}: ",case+1);
    for i in 0..count {
        print!("{}",last_char);
    }
    for i in 0..char_print.len(){
        print!("{}",char_print[i]);
    }
}
