
use std::collections::HashMap;

pub fn run(){
    hash_example();
}

//hash map example()

fn hash_example(){
    let mut map = HashMap::new();
    map.insert("c", "a");
    map.insert("b", "b");
    map.insert("a", "c");

   /* for key in map.values() {
        println!("{}", key);
    }*/
    let mut t:char ;
    let error = &"0";
    println!("{}",match map.get(&"v") {
        Some(k) => k,
        None => error,
        });
}