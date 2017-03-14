
pub fn code_j(){

    //string_learn();
    let mut t_c: i32  = 1;

    let mut test_cases: i32 = read!();


    loop {
        if test_cases >= t_c  {

            reverse_sen(t_c);
            t_c = t_c + 1;
        }else {
            break;
        }
    }
}



fn string_learn() {
    // (all the type annotations are superfluous)
    // A reference to a string allocated in read only memory
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // Iterate over words in reverse, no new string is allocated
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        print!(" {}", word);
    }

    // Copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // Create an empty and growable `String`
    let mut string = String::new();
    for c in chars {
        // Insert a char at the end of string
        string.push(c);
        // Insert a string at the end of string
        string.push_str(", ");
    }

    // The trimmed string is a slice to the original string, hence no new
    // allocation is performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // Heap allocate a string
    let alice = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}


fn reverse_sen(case: i32){
    let mut string: String = String::new();

    string = read!("{}\n");

    print!("Case #{}: ",case);
    for word in string.split_whitespace().rev() {
        print!("{} ",word);
    }
    println!("");
}

use std::io;
fn read_string() -> String{
    let mut string = String::new();


    match io::stdin().read_line(&mut string){
        Ok(n) => {
            string
        }
        Err(error) => String::from("Error occured"),
    }
}