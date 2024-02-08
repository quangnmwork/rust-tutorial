// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     return largest;
// }

use std::{ collections::HashMap, fmt::Display, hash };

trait Summary {
    fn summary_info(&self) -> String;
}
struct Tweet {
    username: String,
}

impl Summary for Tweet {
    fn summary_info(&self) -> String {
        format!("Username: {}", self.username)
    }
}

fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summary_info());
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Point<T> {
    fn cmp(&self, other_point: &Point<T>) -> bool {
        if self.x > other_point.x && self.y > other_point.y {
            print!("Large than other_point");
            return true;
        }
        print!("Less than other_point");
        return false;
    }
}

// lifetime

// fn lifetime() {
//     let r;
//     {
//         let x = 5;
//         r = &x; // Error here . x will be destroy when out of scoped
//     }
//     println!("{}", r)
// }

// Error missing lifetime specifier
// This is because maybe you we have some case like this

// let x = 5;
// let y ;
// { y = &z}
// longest(x,y) will cause error
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// So you need life annotation for this

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

const HELLO_WORLD: &'static str = "hello world"; // Can use any where

fn main() {
    // let tweet = Tweet {
    //     username: "Evan".to_string(),
    // };

    // let tweet_str = tweet.summary_info();
    // dbg!(tweet_str);

    // notify(
    //     &(Tweet {
    //         username: "Quang".to_string(),
    //     })
    // );

    // let point1 = Point { x: 1, y: 2 };
    // let point2 = Point { x: 0, y: 0 };

    // point1.cmp(&point2);

}
