use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    guessing_game();
}

fn guessing_game() {
    // 生成本次游戏的答案数字
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // 让用户输入数字
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            // expect 会让程序在出现错误时, 直接终止并运行
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input illegal!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
