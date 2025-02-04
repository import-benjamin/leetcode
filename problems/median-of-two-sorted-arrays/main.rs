// Runtime: 0ms, 2.7MB

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged_nums: Vec<i32> = nums1.into_iter().chain(nums2).collect();
        merged_nums.sort();
        let mid_index: f32 = ((merged_nums.len() - 1) as f32 / 2f32);
        match (mid_index).fract() == 0.0 {
            true => merged_nums[mid_index as usize] as f64,
            false => {
                (merged_nums[((mid_index as i32) as usize)]
                    + merged_nums[((mid_index as i32) as usize) + 1]) as f64
                    / 2.0
            }
        }
    }
}
