use crate::guess;
use crate::Solution;

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        while left <= right {
            let mid = left + (right - left) / 2;
            match guess(mid) {
                -1 => right = mid - 1,
                0 => return mid,
                1 => left = mid + 1,
                _ => unreachable!(),
            }
        }
        -1
    }
}
