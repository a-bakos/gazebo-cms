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
    pub separator: String,
    pub stop_words: HashSet<&'a str>,
    pub not_allowed_characters: HashSet<&'a str>,
}

impl<'a> PermalinkGenerator<'a> {
    pub fn new(use_hyphen_as_separator: bool) -> Self {
        let separator;
        if use_hyphen_as_separator {
            separator = consts::DEFAULT_PERMALINK_SEPARATOR.to_string();
        } else {
            separator = "_".to_string();
        }

        Self {
            separator,
            stop_words: ["and", "the", "of", "a"].iter().cloned().collect(),
            not_allowed_characters: ["&", "#", "?", "%", "<", ">", "\"", "'", "/"]
                .iter()
                .cloned()
                .collect(),
        }
    }

    // permalink_generator.extend_stop_words(vec!["dog", "bread"]);
    pub fn extend_stop_words(&mut self, more_stop_words: Vec<&'a str>) {
        for word in more_stop_words.iter() {
            self.stop_words.insert(word);
        }
    }

    pub fn create_permalink_from(&self, slug: String) -> String {
        // Trim + Lowercase
        let mut permalink = slug.trim().to_lowercase().to_string();

        // Get rid of unwanted characters
        permalink = permalink
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

        // Remove stop words
        let mut permalink_as_words: Vec<&str> = permalink.split(' ').collect();
        permalink_as_words.retain(|&word| !self.stop_words.contains(word));

        // Join words by separator character
        let permalink = permalink_as_words.join(&self.separator);

        // Remove duplicated hyphens
        let permalink: String = permalink.chars().fold(String::new(), |mut acc, ch| {
            if ch == consts::DEFAULT_PERMALINK_SEPARATOR.chars().next().unwrap()
                && acc.ends_with(consts::DEFAULT_PERMALINK_SEPARATOR)
            {
                // Skip adding the separator to `acc`
                acc
            } else {
                acc.push(ch);
                acc
            }
        });

        // URL Encoding
        let permalink = encode(&permalink).to_string();

        permalink
    }
}
