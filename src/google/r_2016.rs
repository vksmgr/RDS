pub fn run() {
    counting_sheep();
}

//Counting Sheep

fn counting_sheep() {
    let test_cases: i32 = read!();

    for current_case in 0..test_cases as usize {
        logic(current_case)
    }
}

fn logic(current_case: usize) {
    let mut sleep: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut sheep: Vec<i32> = Vec::new();
    let mut count: i32 = 1;
    let mut flag: bool = true;
    let mut index: i32;
    let mut number: i32 = read!();
    let mut current_number = number;
    let mut backup: i32 = current_number;
    if number == 0 {
        println!("Case #{}: {}", current_case+1, "INSOMNIA");
    }else {
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
                    println!("Case #{}: {}", current_case+1, backup);
                    flag = false;
                    break;
                }
                number = number / 10;
            }
            count = count + 1;
        }
    }
}