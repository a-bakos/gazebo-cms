/// Checks if a string contains any special characters.
///
/// A special character is considered any character that is not a lowercase or uppercase letter,
/// or a number. This function checks if at least one special character is present in the input string.
///
/// # Arguments
///
/// - `str_to_check` - A string slice containing the input string to check
///
/// # Returns
///
/// Returns `true` if the input string contains at least one special character, `false` otherwise.
///
/// # Example
///
///  ```
///  assert_eq!(str_contains_special_char("abc123"), false);
///  assert_eq!(str_contains_special_char("abc$123"), true);
///  ```
///
/// Note: This function also has a unit test suite in the same module.
pub fn str_contains_special_char(str_to_check: &str) -> bool {
    return str_to_check
        .chars()
        .any(|ch| match ch.to_ascii_lowercase() {
            'a'..='z' | '0'..='9' => false,
            _ => true,
        });
}

/// Checks if a string contains at least one uppercase letter.
///
/// # Arguments
///
/// - `str_to_check` - The string slice to check for uppercase letters
///
/// # Returns
///
/// Returns `true` if `str_to_check` contains at least one uppercase letter, `false` otherwise.
///
/// # Examples
///
/// ```
/// let valid_str = "Abcdefg123";
/// let invalid_str = "abcdefg123";
///
/// assert_eq!(str_contains_uppercase(valid_str), true);
/// assert_eq!(str_contains_uppercase(invalid_str), false);
/// ```
///
/// Note: This function also has a unit test suite in the same module.
pub fn str_contains_uppercase(str_to_check: &str) -> bool {
    str_to_check.chars().any(|ch| {
        if ch.is_ascii_uppercase() {
            return true;
        }
        false
    })
}

/// Checks if a string contains at least one numeric character.
///
/// # Arguments
///
/// - `str_to_check` - The string slice to check for numeric characters
///
/// # Returns
///
/// Returns `true` if `str_to_check` contains at least one numeric character, `false` otherwise.
///
/// # Examples
///
/// ```
/// let valid_str = "Abcdefg123";
/// let invalid_str = "abcdefg";
///
/// assert_eq!(str_contains_number(valid_str), true);
/// assert_eq!(str_contains_number(invalid_str), false);
/// ```
///
/// Note: This function also has a unit test suite in the same module.
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
