use std::ptr;
use std::mem;
// modular programming


//start vector to run
pub fn run(){
    //vector();
   // new_vec_with_capacity();
    vector_from_raw_parts();
}

fn vector(){
    let mut vec : Vec<i32> = Vec::new();
    let mut num: i32 ;
    let mut next: bool = true;
    println!("This will demonstrate about vectors ");
    println!(" Enter vector if you want to exit enter 00000");
    while next {
        num = read!();
        if num == 00000 {
            next = false;
        }else {
            vec.push(num);
        }
    }
    for i in vec.iter() {
        print!("{}",i);
    }
}


//creating vector with specified capacity

//with the method with_capacity()

//with capacity
fn new_vec_with_capacity(){
    let mut vec: Vec<i32> = Vec::with_capacity(10);

    println!("This will create a vector with capacity 10");

    for i in 0..10 {
        vec.push(i);
    }

    //when i push the 11th element the vector must have to move to the anathor space in memory

    vec.push(11);

}

//third method of the vector is from_raw_parts()
//This method will create the new vector from row parts of the vector
// for that perpose you must have the pointer to the vector
// lenght of the vector
//and also the capacity of the vector

fn vector_from_raw_parts(){
    //create a vector to destruct
    let mut v = vec![1, 2, 3];

    //pulls the infrormation of the vector
    // this will give the mutale pointer to the vector
    let p = v.as_mut_ptr();

    //this will get the lenght of the vector
    let len = v.len();

    //now we need just the cpacity of the vector
    let cap = v.capacity();

    //now we will clear free the vector
    unsafe {
        mem::forget(v);

        //overwrite the memory
        for i in 0..len as isize {
            ptr::write(p.offset(i), 4 + i);
        }

        //put everything back
        let rebuild = Vec::from_raw_parts(p, len, cap);
        assert_eq!(rebuild, [4, 5, 6]);
    }



}