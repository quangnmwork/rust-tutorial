use std::collections::HashMap;

fn access_variable() {
    let v = vec![1, 2, 3];

    // This use when you want your program to crash if there's an attempt to access an element
    let third = v[2];

    println!("The third element is {third}");

    let third = v.get(2);

    match third {
        None => println!("There is no third element"),
        Some(value) => println!("The value is {}", third.unwrap()),
    }
}

fn iterating_vector() {
    let v = vec![1, 2, 3];
    for i in &v {
        println!("{i}");
    }
}

// Vector store enum variable
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Text(String),
}

fn store_enum_variables() {
    let row = vec![SpreadsheetCell::Int(3), SpreadsheetCell::Text(String::from("blue"))];
}

// Add two string

fn add_two_string() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2;

    println!("{}", s3);
}

fn index_to_string() {
    let hello = String::from("Здравствуйте");
    // let answer = &hello[0];

    for c in hello.chars() {
        print!("{c} ");
    }
}

fn hash_map() {
    let mut cnt_words = HashMap::new();

    let words = String::from("Hello word");

    for c in words.chars() {
        cnt_words
            .entry(c)
            .and_modify(|e| {
                *e += 1;
            })
            .or_insert(1);
    }

    println!("{:?}", cnt_words);
}

fn main() {
    // access vector value
    access_variable();

    // Iterating code
    iterating_vector();

    // Store enum variables
    store_enum_variables();

    // Add two string
    add_two_string();

    // Index string char
    index_to_string();

    // Collections hash map
    hash_map();
}
