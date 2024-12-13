use std::time::Instant;

use rand::Rng;

pub fn find_score(nums: Vec<i32>) -> i64 {
    let sz = nums.len();
    let mut visited = vec![false; sz];
    let mut score = 0;
    let mut sorted: Vec<(usize, &i32)> = nums.iter().enumerate().collect();

    sorted.sort_by_key(|&(_, &n)| n);

    for (i, &n) in sorted {
        if visited[i] {
            continue;
        }
        visited[i] = true;

        if let Some(i) = i.checked_sub(1) {
            visited[i] = true;
        }
        let i = i + 1;
        if i < sz {
            visited[i] = true;
        }

        score += n as i64;
    }

    score
}

fn main() {
    let mut rng = rand::thread_rng();

    let nums: Vec<i32> = (0..100_000).map(|_| rng.gen_range(0..=1_000_000)).collect();

    let start = Instant::now();
    let score = find_score(nums);
    let duration = start.elapsed();

    println!("{}, time: {:?}", score, duration);
}
