fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("内部スコープにおけるXの値は: {}", x);
    }

    println!("外部スコープにおけるXの値は: {}", x);
}
