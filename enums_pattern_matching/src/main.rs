// enum IpAddrKind {
//     V4,
//     V6,
//     V8(u128),
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// enum Message {
//     Quit,
//     Move {
//         x: i32,
//         y: i32,
//     },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// enum Option<T> {
//     None,
//     Some(T),
// }

// fn main() {
//     let absent_number: Option<i32> = Option::None;

//     let sum = absent_number + 3;

//     let home = IpAddr {
//         kind: IpAddrKind::V8(1),
//         address: String::from("127.0.0.1"),
//     };

//     println!("Hello, world!");
// }

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny coin");
            return 1;
        }
        Coin::Dime => 5,
        Coin::Nickel => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let var = value_in_cents(Coin::Penny);

    let x = Some(1);

    println!("{}", var);

    let config = Some(1);

    if let Some(1) = config {
        println!("The maximum is configured to be {}", 1);
    }

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 3;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => {
            count += 1;
        }
    }

    println!("{}", count);
}
