//Monk Takes a Walk


use std::io;

pub fn monk_takes_walk() {
    let mut strn = String::new();
    let mut count = 0;
    let mut T = 0;
    T = read!();
    while T > 0 {
        count = 0;
        strn = read!("{}\n");
        println!("{}", strn);
        for letter in strn.chars() {
            if letter == 'A' ||
                letter == 'E' ||
                letter == 'I' ||
                letter == 'O' ||
                letter == 'U' ||
                letter == 'a' ||
                letter == 'e' ||
                letter == 'i' ||
                letter == 'o' ||
                letter == 'u' {
                count = count + 1;
            }
        }
        println!("{}", count);
        T = T - 1;
    }
}


pub fn monk_takes_walk_2() {
    let mut T = 0;
    T = readInt();

    while T > 0 {
        let mut strn = String::new();
        io::stdin().read_line(&mut strn);
        let mut count = 0;
        for letter in strn.chars() {
            if letter == 'A' ||
                letter == 'E' ||
                letter == 'I' ||
                letter == 'O' ||
                letter == 'U' ||
                letter == 'a' ||
                letter == 'e' ||
                letter == 'i' ||
                letter == 'o' ||
                letter == 'u' {
                count = count + 1;
            }
        }
        println!("{}", count);
        T = T - 1;
    }
}

fn readInt() -> i32 {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => i,
        Err(..) => 00000
    }
}

//At first, we have no memory allocated at all, but as we append to the string
//it increases its capacity appropriately. If we instead use the with_capacity()
//method to allocated the correct capacity initially.

fn string_with_testing(){
    let mut s = String::with_capacity(25);

    println!("{}", s.capacity());

    for _ in 0..5{
        s.push_str("hello");
        println!("{}", s.capacity());
    }
}

//fn new() -> String
// Creates a new empty String.
//Given that the String is empty, this will not allocated any initial buffer.
//while that meaens that this initial operation is very inexpensive, but may
//cause excessive allocation later, when you add data. if you have an idea of
//how much data the String will hold, consider the with_capacity() method to
//prevent excessive re-allocation.

//fn with_capacity(capacity: usize) -> String

//Creates a new empty String with a perticular capacity.
//Strings have an internal buffer to hold their data. the capacity is the
//length of that buffer, and can be queried with the capacity() method.
//this method creates an empty String, but one with an initial buffer that
//can hold capacity bytes. this is useful when you may be appending a
//bunch of data to the String, reducing the number of reallocations it need
//to do.

//if the given cacity is 0, no allocation will occur, and this method is
//identical to the new() method.

fn contains_no(){
    let mut s = String::with_capacity(10);

    //the string contains no chars, even though it has capacity for more
    assert_eq!(s.len(), 0);

    //these are all done without reallocating...
    let cap = s.capacity();
    for i in 0..10 {
        s.push('a');
    }
    assert_eq!(s.capacity(), cap);

    //.. but this may make the vector reallocate
    s.push('a');
}


//fn from_utf8_sur(vec: Vec<u8>) -> Result<String, FromUtf8Error>
//A string slice(&str) is made of bytes(u8), and a vector of bytes(Vec<u8>) is  made
//of bytes, so this fuction converts between the two.
//Not all byte slice are valid Strings, howeve: String requires that is is valid
//UTF-8, and then does the conversion.

//if you are sure that the byte slice is valid UTF-8, and you don't want to
//incure the overhead of the validity check, there is an unsafe version of
//this function, from_utf8_unchecked(), which has the same behavior but skips
//the check.

//this method will take care to not copy the vector, for efficiency's sake.
//if you need a &str insted of a String, consider str::from_utf8().

//Errors
//Returns Err if the slice is not UTF-8 with a description as to why the
//provided bytes are not UTF-8. the vector you moved in is also included.

fn incorrects_bytes(){
    // some bytes, in a vector
    let sparkle_heart = vec![240, 159, 146, 150];

    //we know these bytes are valid, so we'll use unwrap().
    let sparklet_heart = String::from_utf8(sparkle_heart).unwrap();

    //assert_eq!(" ", sparkle_heart);
}

//Converts a slice of bytes to a string, including invalid characters.
// Strings are made of bytes(u8), and a slice of bytes(&[u8]) is made of
//bytes, so this function convers between the two. Not all bytes slice are
//valid strings, however: string are required to be valid UTF-8. During
//this conversion, from_utf8_lossy() will replace any invalid UTF-8
//squences with U+FFFD REPLACEMENT CHARACTER, which looks like this
//if you are sure that the bytes alice is valid UTF-8, and you don't want to
//incure the overhead of the conversion, there is an unsafe version of
//this function, from_utf8_unchecked(), which has the same behavior but
//skips the checks.

//This function returns a Cow<'a, str>. If our byte slice is invalid UTF-8,
//then we need to insert the replacement characters, which will change the
//size of the string, and