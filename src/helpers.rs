pub fn str_contains_special_char(str_to_check: &str) -> bool {
    return str_to_check.chars().any(|ch| match ch {
        '@' | '#' | '!' | '?' | '>' | '<' | '|' | '~' | ':' | ';' | '\'' | '"' | '`' | '£'
        | '$' | '%' | '^' | '&' | '*' | '(' | ')' | '_' | '+' | '=' | '-' | '}' | '{' | '['
        | ']' | ',' | '.' | '/' | '\\' => true,
        _ => false,
    });
}
