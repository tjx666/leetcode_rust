pub struct Solution {}

impl Solution {
    /**
     * 二分
     */
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut search_start = 0;
        let mut nums1_size = m as usize;

        let mut jump_index = 0;
        for index in 0..n as usize {
            jump_index = index + 1;
            let num2 = nums2[index];
            let sub_nums = &nums1[search_start..nums1_size];
            let insert_index = match sub_nums.binary_search(&num2) {
                Ok(i) => search_start + i + 1,
                Err(i) => search_start + i,
            };
            for j in (insert_index..nums1_size).rev() {
                nums1[j + 1] = nums1[j];
            }
            nums1[insert_index] = num2;
            nums1_size += 1;
            search_start = insert_index + 1;
            if search_start >= nums1_size {
                break;
            }
        }

        if jump_index != n as usize {
            for num in &nums2[jump_index..nums2.len()] {
                nums1[nums1_size] = *num;
                nums1_size += 1;
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
