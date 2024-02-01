fn main() {
    // println!("Hello, world!");

    // let x = 5; // ここでエラー発生する、”mut"をついか
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // Tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 109);

    let (x, y, z) = tup;

    println!("The value of tuple is: {x} {y} {z}");

    // Array type
    const ARRAY_LEN: usize = 5;

    let a: [i32; ARRAY_LEN] = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("First element is :  {first}");

    // Functions

    let y = {
        let x = 3;
        x + 1
    };

    println!("Value of y is {y}");

    // Control flow

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of condition number is: {number}");

    // Loop

    let mut cnt = 0;
    let result = loop {
        cnt += 1;
        if cnt == 10 {
            break cnt * 2;
        }
    };

    println!("Result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    let n = 15;
    for num in 1..n {
        print!("{num}");
    }
}
