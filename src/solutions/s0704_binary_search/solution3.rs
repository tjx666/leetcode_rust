#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        return nums.binary_search(&target).map_or(-1, |x| x as i32);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_1() {
        // assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        // assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
        assert_eq!(Solution::search(vec![2, 4], 1), -1);
    }
}
