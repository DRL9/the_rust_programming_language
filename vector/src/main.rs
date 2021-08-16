use std::collections::HashMap;

fn main() {
    let mut v1 = vec![1, 2, 3];
    println!("v1: {}", v1.len());
    println!("v1: {}", v1.capacity());
    // 10个元素内，不需要重新分配内存
    let mut v2: Vec<i32> = Vec::with_capacity(10);
    println!("v2: {}", v2.len());
    println!("v2: {}", v2.capacity());
    for i in 0..11 {
        v1.push(i);
        v2.push(i);
    }
    println!("v1: {}, v2: {}", v1.capacity(), v2.capacity());
    v2.reserve(10);
    println!("v2: {}", v2.capacity());
    v2.shrink_to_fit();
    println!("v2: {}", v2.capacity());

    e_1();
}

fn e_1() {
    let mut list = vec![1, 6, 23, 457, 78, 45, 44, 44, 20];
    let average = {
        let mut sum = 0;
        for i in &list {
            sum += i;
        }
        sum / list.len()
    };
    let median = {
        list.sort();
        if list.len() % 2 == 0 {
            (list[list.len() / 2] + list[list.len() / 2 - 1]) / 2
        } else {
            list[(list.len() - 1) / 2]
        }
    };
    let mode = {
        let mut map = HashMap::new();
        let mut mode = 0;
        let mut max = 0;
        for i in &list {
            let v = map.entry(i).or_insert(0);
            *v += 1;
            if *v > max {
                max = *v;
                mode = *i;
            }
        }
        mode
    };
    println!("avg is {}, median is {}, mode is {}", average, median, mode);
}
