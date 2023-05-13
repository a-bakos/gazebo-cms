pub fn str_contains_special_char(str_to_check: &str) -> bool {
    return str_to_check
        .chars()
        .any(|ch| match ch.to_ascii_lowercase() {
            'a'..='z' | '0'..='9' => false,
            _ => true,
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_str_contain_special_char() {
        assert_eq!(str_contains_special_char("hello"), false); // input contains only letters
        assert_eq!(str_contains_special_char("12345"), false); // input contains only numbers
        assert_eq!(str_contains_special_char("hello12345"), false); // input contains letters and numbers
        assert_eq!(str_contains_special_char("hello#world"), true); // input contains special characters
        assert_eq!(str_contains_special_char("?!@#$%"), true); // input contains only special characters
        assert_eq!(str_contains_special_char(""), false); // empty input string
        assert_eq!(str_contains_special_char("HeLlO"), false); // input contains only letters (different cases)
        assert_eq!(str_contains_special_char("HeLLo12345"), false); // input contains letters and numbers (different cases)
        assert_eq!(str_contains_special_char("HeLLo#WorLD"), true); // input contains special characters (different cases)
    }
}
