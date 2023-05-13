pub fn str_contains_special_char(str_to_check: &str) -> bool {
    return str_to_check
        .chars()
        .any(|ch| match ch.to_ascii_lowercase() {
            'a'..='z' | '0'..='9' => false,
            _ => true,
        });
}

pub fn str_contains_uppercase(str_to_check: &str) -> bool {
    str_to_check.chars().any(|ch| {
        if ch.is_ascii_uppercase() {
            return true;
        }
        false
    })
}

pub fn str_contains_number(str_to_check: &str) -> bool {
    str_to_check.chars().any(|ch| {
        if ch.is_numeric() {
            return true;
        }
        false
    })
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

    #[test]
    fn does_str_contain_uppercase() {
        let str_with_uppercase = "Hello, World!";
        let str_without_uppercase = "12345";
        let str_with_uppercase_at_end = "1234U";
        let str_with_uppercase_at_beginning = "U1234";

        assert_eq!(str_contains_uppercase(str_with_uppercase), true);
        assert_eq!(str_contains_uppercase(str_without_uppercase), false);
        assert_eq!(str_contains_uppercase(str_with_uppercase_at_end), true);
        assert_eq!(
            str_contains_uppercase(str_with_uppercase_at_beginning),
            true
        );
    }

    #[test]
    fn does_str_contain_number() {
        let str_with_numbers = "12345";
        let str_without_numbers = "abcdefg";
        let str_with_numbers_at_end = "abcde1234";
        let str_with_numbers_at_beginning = "1234abcde";

        assert_eq!(str_contains_number(str_with_numbers), true);
        assert_eq!(str_contains_number(str_without_numbers), false);
        assert_eq!(str_contains_number(str_with_numbers_at_end), true);
        assert_eq!(str_contains_number(str_with_numbers_at_beginning), true);
    }
}
