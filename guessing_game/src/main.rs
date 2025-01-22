use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("数字を当ててみて！");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();

        println!("数字入れてね");

        io::stdin()
            .read_line(&mut guess)
            .expect("行の読み込みに失敗しました");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("あなたの予想は: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さいよ！"),
            Ordering::Greater => println!("大きいよ！"),
            Ordering::Equal => {
                println!("あたりだよ，おめでとう！");
                break;
            }
        }
    }
}
