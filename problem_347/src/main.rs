pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::{BinaryHeap, HashMap};

    #[derive(Debug, Eq)]
    struct Pair {
        v: i32,
        c: u32,
    }

    impl PartialEq for Pair {
        fn eq(&self, other: &Self) -> bool {
            self.c == other.c
        }
    }

    impl Ord for Pair {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.c.cmp(&other.c)
        }
    }

    impl PartialOrd for Pair {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut occur = HashMap::new();
    nums.iter().for_each(|&v| {
        occur
            .entry(v)
            .and_modify(|p: &mut Pair| {
                p.c += 1;
            })
            .or_insert(Pair { v, c: 1 });
    });

    let mut sort = BinaryHeap::new();
    occur.values().for_each(|p| sort.push(p));

    (0..(k as usize).min(sort.len()))
        .map(|_| unsafe { sort.pop().unwrap_unchecked().v })
        .collect()
}

fn main() {
    println!("{:?}", top_k_frequent(vec![5, 2, 5, 3, 5, 3, 1, 1, 3], 2));
}
