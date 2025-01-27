use std::io;

fn main() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("floored: {}", floored);

    let remainder = 43 % 5;
    println!("remainder: {}", remainder);

    let t = false;
    let f: bool = false;

    let c = 'z';

    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple;
    println!("The value of y is: {}", y);

    println!("タプルの3つめの値: {}", tuple.2);

    let a = [1, 2, 3, 4, 5];
    println!("配列の何番目にアクセスするか入力してください");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("行の読み込みに失敗");

    let index: usize = index.trim().parse().expect("入力された値は数字ではない");

    let element = a[index];

    println!("{}番目の要素は{}です", index, element);
}
