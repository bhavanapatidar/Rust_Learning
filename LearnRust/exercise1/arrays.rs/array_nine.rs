//An array is monotonic if it is either monotone increasing or monotone decreasing.

//An array nums is monotone increasing if for all i <= j, nums[i] <= nums[j]. An array nums is monotone decreasing if for all i <= j, nums[i] >= nums[j].

//Given an integer array nums, return true if the given array is monotonic, or false otherwise.
impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut increasing = true; //true is because we want to check if the array is monotone increasing
        let mut decreasing = true;

        for i in 1..nums.len() { 
            if nums[i] < nums[i - 1] {
                increasing = false;
            }
            if nums[i] > nums[i - 1] {
                decreasing = false;
            }
        }

        increasing || decreasing
    }
}