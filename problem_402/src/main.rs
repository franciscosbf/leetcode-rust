pub fn remove_kdigits(num: String, k: i32) -> String {
    if num.len() <= k as usize {
        return "0".to_string();
    }

    use std::{char, collections::VecDeque};

    let len = num.len();
    let mut num = num.chars().map(|c| c.to_digit(10).unwrap());
    let mut k = k;

    let mut stack = VecDeque::with_capacity(len - k as usize);

    stack.push_front(num.next().unwrap());

    num.for_each(|d| {
        while k > 0 {
            let backd = stack.back();
            if backd.is_some() && *backd.unwrap() > d {
                stack.pop_back();
                k -= 1;
            } else {
                break;
            }
        }
        stack.push_back(d);
    });
    while k > 0 {
        stack.pop_back();
        k -= 1;
    }

    while let Some(&d) = stack.front() {
        if d == 0 {
            stack.pop_front();
        } else {
            break;
        }
    }

    if stack.is_empty() {
        return "0".to_string();
    }

    stack
        .iter()
        .map(|&d| char::from_digit(d, 10).unwrap())
        .collect()
}

fn main() {
    let samples = vec![
        ("1432219", 3),
        ("10200", 1),
        ("10", 2),
        ("10", 4),
        ("112", 1),
        ("3212", 3),
    ];

    for (num, k) in samples {
        let integers = remove_kdigits(num.to_string(), k);

        println!("result for num={}, k={}: {}\n", num, k, integers);
    }
}
