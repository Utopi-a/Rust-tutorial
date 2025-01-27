fn main() {
    println!("Hello, world!");
    let y = {
        let x = 3;
        x + 1
    };
    println!("yの値は{}です", y);
    let result = test_function(10, 'd');
    println!("resultの値は{}です", result);

    flow_control();
    loop_test();
    for_test();
    for index in 1..10 {
        println!("{}", fibonacci(index));
    }
}

fn test_function(num: i32, unitLabel: char) -> i32 {
    println!("他の関数だよ〜 {} {}", num, unitLabel);
    2345
    // なんの意味もないコメントです
    // コメント，たのし〜
}

fn flow_control() {
    let number = 204;
    const FILTER_NUMBER: i32 = 234;

    if number < FILTER_NUMBER {
        println!("小さいよ，{}よりも", FILTER_NUMBER);
    } else {
        println!("以上だよ，{}よりも", FILTER_NUMBER);
    }

    if number != 0 {
        println!("numberは0ではないなんらかである");
    }

    let test = if number == 234 { true } else { false };
    println!("testの値は{}です", test);
}

fn loop_test() {
    let mut counter = 0;
    'counting_up: loop {
        counter += 1;
        if counter == 10 {
            break 'counting_up;
        }
        println!("{}", counter);
    }
    println!("{}", counter);

    while counter != 0 {
        println!("{}!", counter);

        counter -= 1;
    }

    println!("射爆了");
}

fn for_test() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("{}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("打ち上げ");
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 || n == 2 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}
