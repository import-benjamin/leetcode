fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input: Vec<String> = ["This", "is", "an", "example", "of", "text", "justification."].iter().map(|s| s.to_string()).collect();
        let expected: Vec<String> = ["This    is    an", "example  of text", "justification.  "].iter().map(|s| s.to_string()).collect();
        let result = full_justify(input, 16);
        assert_eq!(result, expected);
    }

    #[test]
    fn case_2() {
        let input: Vec<String> = ["What","must","be","acknowledgment","shall","be"].iter().map(|s| s.to_string()).collect();
        let expected: Vec<String> = ["What   must   be", "acknowledgment  ", "shall be        "].iter().map(|s| s.to_string()).collect();
        let result = full_justify(input, 16);
        assert_eq!(result, expected);
    }

    #[test]
    fn case_3() {
        let input: Vec<String> = ["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"].iter().map(|s| s.to_string()).collect();
        let expected: Vec<String> = [  "Science  is  what we", "understand      well", "enough to explain to", "a  computer.  Art is", "everything  else  we", "do                  "].iter().map(|s| s.to_string()).collect();
        let result = full_justify(input, 20);
        assert_eq!(result, expected);
    }
}
