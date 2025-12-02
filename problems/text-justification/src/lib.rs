fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    // We accumulate each justified line as a string in justified_paragraph
    let mut justified_paragraph: Vec<String> = Vec::new();
    // We build our line as a vector of words
    let mut curr_line: Vec<String> = Vec::new();
    // We keep track of characters in current line rather than computing it in each iteration.
    let mut char_in_curr_line: i32 = 0;

    // Start by iterating over words and push to the curr_line as long as char_in_curr_line is smaller than max_width
    for word in words.into_iter() {
        // compute word length
        let word_len: i32 = word.len() as i32;
    
        // Assert if current word overflow curr_line
        let require_new_line: bool = (
                char_in_curr_line
                + match(char_in_curr_line > 0) { true => 1, false => 0} // space
                + word_len
            ) > max_width;
        
        if require_new_line {
            // We justify and publish our curr_line
            // we can compute the total number of space we must add
            let missing_spaces_count: i32 = max_width - char_in_curr_line;
            // Now let's insert spaces until we match our need
            let words_in_line = curr_line.len();

            // add spaces until line match max_width
            for missing_space in 0..missing_spaces_count {
                let word_to_suffix = match (words_in_line - 1) > 0 {
                    // If there is more than one word in line, iterate over each one
                    true => missing_space as usize % (words_in_line - 1),
                    // If there is one word, keep it
                    false => 0,
                };
                if let Some(curr_word) = curr_line.get_mut(word_to_suffix) {
                    *curr_word += " ";
                }
            }
            // Then we create a new curr_line vector
            justified_paragraph.push(curr_line.join(""));
            curr_line = Vec::new();
            char_in_curr_line = 0;
        }

        // DEBUG: From this point, we either have a new line as curr_line or enough space for our word in the current line
        // If the line already contain a word with prefix ours with a space otherwise, we simply push it to the line buffer.
        if char_in_curr_line > 0 {
            curr_line.push(String::from(" ")+&String::from(word));
            char_in_curr_line += word_len + 1;
        } else {
            curr_line.push(String::from(word));
            char_in_curr_line += word_len;
        }
    }

    // for the last line, we fill out remaining space char
    let missing_spaces_count: i32 = max_width - char_in_curr_line;
    // Now let's insert spaces until we match our need
    let words_in_line = curr_line.len();

    if let Some(curr_word) = curr_line.get_mut(words_in_line - 1) {
        *curr_word += &" ".repeat(missing_spaces_count.try_into().unwrap_or(0));
    }

    // we need to flush the last remaining line
    justified_paragraph.push(curr_line.join(""));
    justified_paragraph
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
        assert_eq!(result, expected, "Expected {:#?} but received {:#?}", expected, result);
    }

    #[test]
    fn case_3() {
        let input: Vec<String> = ["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"].iter().map(|s| s.to_string()).collect();
        let expected: Vec<String> = [  "Science  is  what we", "understand      well", "enough to explain to", "a  computer.  Art is", "everything  else  we", "do                  "].iter().map(|s| s.to_string()).collect();
        let result = full_justify(input, 20);
        assert_eq!(result, expected);
    }

    #[test]
    fn case_4() {
        let input: Vec<String> = ["Listen","to","many,","speak","to","a","few."].iter().map(|s| s.to_string()).collect();
        let expected: Vec<String> = ["Listen","to    ","many, ","speak ","to   a","few.  "].iter().map(|s| s.to_string()).collect();
        let result = full_justify(input, 6);
        assert_eq!(result, expected, "Expected {:#?} but received {:#?}", expected, result);
    }
}
