use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    
    let secret_number = rand::thread_rng().gen_range(1..=10);
    println!("Secret number : {secret_number}");
    
    loop {
        println!("Please input your guess.");
        let mut guess: String  = String::new();
        
       /* ここで戻り「Result]タイプが得されるため、エラーを処理するためのExpect()メソッドが必要　*/
        io::stdin().read_line(&mut  guess).expect("Failed to read line");

        /* それはシャドーイングと呼ばれる、効果は変数名を考えるを減らすこと*/
        let guess: u32 =match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed : {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
        }
   }
}
