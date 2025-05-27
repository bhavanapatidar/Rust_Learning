
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

            if !first_set || num > first { // If num is greater than the current first maximum and first is not set
                // Shift the current first and second maximums down
               
                third = second; // Move second to third
                third_set = second_set; // Update third_set to reflect the new value of third, because second is now the new third

                second = first;  //current first becomes second
                second_set = first_set; // Update second_set to reflect the new value of second, because first is now the new second

                first = num;
                first_set = true;
            } else if !second_set || num > second { // If num is greater than the current second maximum and second is not set
                third = second;
                third_set = second_set;

                second = num;
                second_set = true;
            } else if !third_set || num > third { // If num is greater than the current third maximum and third is not set
                third = num;
                third_set = true;
            }
        }

        if third_set { // If the third maximum is set, return it
            third 
        } else { // If the third maximum is not set, return the first maximum
            first
        }
    }
}
