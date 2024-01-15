pub fn max_area(height: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = height.len() - 1;

    let mut h = 0;
    let mut mcs = 0;

    while l != r {
        let lh = height[l];
        let rh = height[r];
        let th = lh.min(rh);
        if th > h {
            h = th;
            let tcs = th * (r - l) as i32;
            if tcs > mcs {
                mcs = tcs;
            }
        }
        match lh.cmp(&rh) {
            std::cmp::Ordering::Less => l += 1,
            _ => r -= 1, // equal or greater
        }
    }

    mcs
}

fn main() {
    println!("{}", max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
}
