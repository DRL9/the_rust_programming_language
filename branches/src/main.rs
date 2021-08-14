fn main() {
    let num = 3;
    if num < 5 {
        println!("condition was true");
    } else if num < 3 {
        println!("another condition");
    } else {
        println!("condition was false");
    }

    let gt3 = if num > 3 { true } else { false };
    println!("gt3? {}", gt3);

    let mut counter = 1;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };
    println!("result: {}", result);

    let a = [10, 20, 30];
    for num in a.iter() {
        println!("a[]: {}", num);
    }

    for c in (1..=4).rev() {
        println!("count down. {}", c);
    }
    fibonacci(10);
}

fn fibonacci(n: u32) {
    let mut a = 1;
    let mut b = 1;
    print!("{},{}", a, b);
    for _ in 3..=n {
        let c = a + b;
        a = b;
        b = c;
        print!(",{}", c);
    }
}
