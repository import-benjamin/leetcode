// Runtime: 11ms, 2.94MB

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut smallest_positive_integer: i32 = 0;
        let mut numbers: Vec<_> = nums.into_iter().filter(|&i| i.gt(&(0 as i32))).collect();
        numbers.sort();
        for x in numbers.iter() {
            if *x == smallest_positive_integer + 1 {
                smallest_positive_integer = *x as i32;
            } else if *x == smallest_positive_integer {
            } else {
                break;
            }
        }
        return smallest_positive_integer + 1;
    }
}
