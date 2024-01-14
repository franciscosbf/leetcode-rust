fn min_steps(s: String, t: String) -> i32 {
    let mut scount = [0; 26];
    let mut tcount = [0; 26];

    #[inline]
    fn to_index(c: char) -> usize {
        c as usize - 97
    }

    std::iter::zip(s.chars(), t.chars()).for_each(|(cs, ct)| {
        scount[to_index(cs)] += 1;
        tcount[to_index(ct)] += 1;
    });

    (0..26).map(|i| 0.max(scount[i] - tcount[i])).sum()
}

fn main() {
    println!(
        "{}",
        min_steps("leetcode".to_string(), "practice".to_string())
    );
}
