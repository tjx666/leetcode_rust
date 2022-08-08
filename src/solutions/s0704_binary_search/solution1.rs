pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut mid;
        let mut value;

        while left <= right {
            mid = (left + right) / 2;
            value = nums[mid];

            if target == value {
                return mid as i32;
            }

            if target > value {
                left = mid + 1;
            } else {
                if mid == 0 {
                    return -1;
                }
                // 如果 mid 等于 0 会溢出
                right = mid - 1;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
