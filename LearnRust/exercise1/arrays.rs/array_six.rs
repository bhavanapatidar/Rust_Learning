// A peak element is an element that is strictly greater than its neighbors.

// Given a 0-indexed integer array nums, find a peak element, and return its index. If the array contains multiple peaks, return the index to any of the peaks.

// You may imagine that nums[-1] = nums[n] = -∞. In other words, an element is always considered to be strictly greater than a neighbor that is outside the array.

// You must write an algorithm that runs in O(log n) time.

 

// Example 1:

// Input: nums = [1,2,3,1]
// Output: 2
// Explanation: 3 is a peak element and your function should return the index number 2.

use std::collections::HashMap;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut peak_map = HashMap::new();

        if nums.len() == 1 {
            return 0;
        }

        for (i, &val) in nums.iter().enumerate().skip(1).take(nums.len() - 2) {
            let left = nums[i - 1];
            let right = nums[i + 1];

            if val > left && val > right {
                peak_map.insert(i, val); // store index and value
            }
        }

        // Also check the first element
        if nums[0] > nums[1] {
            peak_map.insert(0, nums[0]);
        }

        // Check the last element
        let last_index = nums.len() - 1;
        if nums[last_index] > nums[last_index - 1] {
            peak_map.insert(last_index, nums[last_index]);
        }

        // Return any one peak index
        if let Some((&idx, _)) = peak_map.iter().next() {
            return idx as i32;
        }

        -1 // fallback
    }
}
