use crate::consts;
use std::collections::HashSet;
use urlencoding::encode;

/**
 * Permalink generator
 *
 * Rules to consider when generating permalinks
 *
 * Make sure the URL is:
 * - SEO-friendly
 * - Human readable
 * - Consistent
 * - Lowercase
 * - Using dashes as separator
 * - Free of special characters
 * - Encoded
 * - Trimmed leading and trailing whitespaces
 * - Limited to 50-60 characters in length
 * - Unique with an appended number, if needed
 */

#[derive(Debug)]
pub struct PermalinkGenerator<'a> {
    pub separator: &'a str,
    pub stop_words: HashSet<&'a str>,
    pub not_allowed_characters: HashSet<&'a str>,
    pub length_limit: usize,
    pub use_limit: bool,
}

impl<'a> PermalinkGenerator<'a> {
    pub fn new(use_dash_as_separator: bool) -> Self {
        let separator;
        if use_dash_as_separator {
            separator = consts::DEFAULT_PERMALINK_SEPARATOR;
        } else {
            separator = "_";
        }

        Self {
            separator,
            stop_words: ["and", "the", "of", "a"].iter().cloned().collect(),
            not_allowed_characters: ["&", "#", "?", "%", "<", ">", "\"", "'", "/"]
                .iter()
                .cloned()
                .collect(),
            length_limit: consts::DEFAULT_PERMALINK_LIMIT,
            use_limit: true,
        }
    }

    pub fn unlimited_permalink_length(&mut self, unlimited: bool) {
        if unlimited {
            self.use_limit = false;
        }
    }

    pub fn set_max_permalink_length(&mut self, permalink_length: usize) {
        if permalink_length > 60 {
            // todo
            // notice to use that the permalink len is too long to be good
        }

        self.length_limit = permalink_length;
    }

    pub fn extend_stop_words(&mut self, more_stop_words: Vec<&'a str>) {
        for word in more_stop_words.iter() {
            self.stop_words.insert(word);
        }
    }

    fn remove_unwanted_characters(&self, permalink: String) -> String {
        let permalink = permalink
            .chars()
            .map(|ch| {
                if self
                    .not_allowed_characters
                    .contains(&ch.to_string() as &str)
                {
                    String::new()
                } else {
                    ch.to_string()
                }
            })
            .collect();
        permalink
    }

    fn remove_stop_words(&self, permalink: String) -> Vec<String> {
        let mut permalink_as_words: Vec<&str> = permalink.split(' ').collect();
        let mut filtered_words: Vec<String> = Vec::new();
        for word in permalink_as_words {
            if !self.stop_words.contains(word) {
                filtered_words.push(word.to_string());
            }
        }
        filtered_words
    }

    fn remove_duplicate_dashes(&self, permalink: String) -> String {
        let permalink: String = permalink
            .chars()
            .fold(String::new(), |mut accumulator, ch| {
                if ch == consts::DEFAULT_PERMALINK_SEPARATOR.chars().next().unwrap()
                    && accumulator.ends_with(consts::DEFAULT_PERMALINK_SEPARATOR)
                {
                    // Skip adding the separator to `accumulator`
                    accumulator
                } else {
                    accumulator.push(ch);
                    accumulator
                }
            });
        permalink
    }

    pub fn create_permalink_from(&mut self, slug: String) -> String {
        // Basic clean: Trim + Lowercase
        let mut permalink = slug.trim().to_lowercase().to_string();
        // Get rid of unwanted characters
        let mut permalink = self.remove_unwanted_characters(permalink);
        // Remove stop words
        let permalink_as_words = self.remove_stop_words(permalink);
        // Join words by separator character
        let permalink = permalink_as_words.join(&self.separator);
        // Remove duplicated dashes
        let permalink = self.remove_duplicate_dashes(permalink);
        // Maybe limit length of permalink
        let permalink = self.maybe_limit_length(permalink);
        // URL Encoding
        let permalink = encode(&permalink).to_string();

        // Return the final permalink
        permalink
    }

    fn maybe_limit_length(&self, permalink: String) -> String {
        // todo - limit to char
        permalink
    }
}
