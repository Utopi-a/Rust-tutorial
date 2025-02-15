fn main() {
    let s = "hello";
    let mut s_heap = String::from("hello");
    s_heap.push_str(", world!");
    let s2 = s_heap.clone(); // deep copyする，しないと所有権がmoveしてs_heapが死ぬ
    println!("{}", s_heap);
    println!("{}", s2);

    let s3 = takes_ownership(s_heap);
    let x = 5;
    makes_copy(x);

    println!("{}", x);

    println!("{}", s3);

    references_and_borrowing();

    slice_type();
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);

    some_string
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn references_and_borrowing() {
    let mut s1 = String::from("hello");
    let len = change_calculate_length(&mut s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn change_calculate_length(s: &mut String) -> usize {
    s.push_str(", mwah");
    s.len()
}

fn slice_type() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    println!("{}", word);

    let a = [1, 2, 3, 4, 5];
    let a_slice = &a[3..];

    println!("{:?}", a_slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
