pub struct Solution {}

impl Solution {
    fn binary_search(nums: &Vec<i32>, target: i32, left: usize, right: usize) -> i32 {
        if left > right {
            return -1;
        }

        let mid = (left + right) / 2;
        let value = nums[mid];

        if target == value {
            return mid as i32;
        }

        if target > value {
            return Solution::binary_search(nums, target, mid + 1, right);
        } else {
            if mid == 0 {
                return -1;
            }

            return Solution::binary_search(nums, target, left, mid - 1);
        }
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Solution::binary_search(&nums, target, 0, nums.len() - 1)
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
