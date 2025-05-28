//An array is monotonic if it is either monotone increasing or monotone decreasing.

//An array nums is monotone increasing if for all i <= j, nums[i] <= nums[j]. An array nums is monotone decreasing if for all i <= j, nums[i] >= nums[j].

//Given an integer array nums, return true if the given array is monotonic, or false otherwise.
impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut increasing = true; //true is because we want to check if the array is monotone increasing
        let mut decreasing = true;

        for i in 1..nums.len() { // Start from the second element and compare it with the previous one
            if nums[i] < nums[i - 1] {// If the current element is less than the previous one, it cannot be increasing
                increasing = false;
            }
            if nums[i] > nums[i - 1] {// If the current element is greater than the previous one, it cannot be decreasing
                decreasing = false;
            }
        }

        increasing || decreasing
    }
}