fn main() {
    println!("Hello, world!");
    let y = {
        let x = 3;
        x + 1
    };
    println!("yの値は{}です", y);
    let result = test_function(10, 'd');
    println!("resultの値は{}です", result);
}

fn test_function(num: i32, unitLabel: char) -> i32 {
    println!("他の関数だよ〜 {} {}", num, unitLabel);
    2345
    // なんの意味もないコメントです
    // コメント，たのし〜
}
