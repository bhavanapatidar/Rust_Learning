
// Given a non-empty array of integers, return the third maximum number in this array. If it does not exist, return the maximum number.
// If the maximum number exists more than once, return the maximum number.
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut first = 0;
        let mut second = 0;
        let mut third = 0;
        let mut first_set = false;
        let mut second_set = false;
        let mut third_set = false;

        for &num in nums.iter() {

            if !first_set || num > first {
               
                third = second;
                third_set = second_set;

                second = first;
                second_set = first_set;

                first = num;
                first_set = true;
            } else if !second_set || num > second {
                third = second;
                third_set = second_set;

                second = num;
                second_set = true;
            } else if !third_set || num > third {
                third = num;
                third_set = true;
            }
        }

        if third_set {
            third
        } else {
            first
        }
    }
}
