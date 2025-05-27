
// Problem: Find Peak Element
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        
        let mut max_val = nums[0];
        let mut max_indx_val = 0;
        for (i,val) in nums.iter().enumerate(){
            if (max_val as i32) < (*val  as i32){
                *&mut max_val = *val as i32;
                *&mut max_indx_val = i as i32;
            }
        } 
            max_indx_val as i32

        
    }

}