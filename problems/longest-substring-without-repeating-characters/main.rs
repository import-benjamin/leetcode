// Runtime: 44ms, 2.35MB

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() < 2 { return s.len() as i32 }
        let mut start_l: usize = 0;
        let mut end_l: usize = 1;
        let mut longest: usize = 0;
        for (i, ch) in s[1..].char_indices() {
            let chk: &str = &s[start_l..end_l];
            if let Some(nsl) = chk.find(ch) {
                println!("Found at {:?}", nsl);
                start_l += nsl + 1
            }
            end_l += 1;
            if (end_l - start_l) > longest { longest = (end_l - start_l)}
        }
        longest as i32
    }
}
