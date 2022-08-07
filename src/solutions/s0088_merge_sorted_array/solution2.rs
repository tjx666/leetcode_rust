#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    /**
     * 双指针，从后往前遍历
     * 时间复杂度：m + n
     */
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m as usize;
        let mut n = n as usize;
        let mut index = m + n ;

        while m > 0 && n > 0 {
            let _m = m - 1;
            let _n = n - 1;

            if nums1[_m] >= nums2[_n] {
                nums1[index - 1] = nums1[_m];
                m -= 1;
            } else {
                nums1[index - 1] = nums2[_n];
                n -= 1;
            }
            index -= 1;
        }

        if n > 0 {
            while n > 0 {
                nums1[index - 1] = nums2[n - 1];
                n -= 1;
                index -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6])
    }

    #[test]
    fn test2() {
        // [2,0] 1 [1] 1
        let mut nums1 = vec![2, 0];
        let mut nums2 = vec![1];
        Solution::merge(&mut nums1, 1, &mut nums2, 1);
        assert_eq!(nums1, vec![1, 2])
    }
}
