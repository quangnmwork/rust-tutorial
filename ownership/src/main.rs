fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}

fn main() {
    let mut s = String::from("hello");

    // example about take_ownership
    // let s2 = s1.clone();

    // println!("{}, world!", s1);

    // takes_ownership(s2);

    //　Error : cannot use multiple mutable ref
    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    // Error too because can use with immutable and mutable reference
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);
}
