use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("1から100までの数を当ててください!");

    loop {
        println!("予想する数字を入力してください:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("入力エラー");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < secret_number {
            println!("もっと大きいです!");
        } else if guess > secret_number {
            println!("もっと小さいです!");
        } else {
            println!("正解です!");
            break;
        }
    }
}
