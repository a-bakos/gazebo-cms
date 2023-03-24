use crate::consts;
use std::collections::HashSet;

/**
 * Permalink generator
 *
 *
 */

#[derive(Debug)]
pub struct PermalinkGenerator<'a> {
    pub separator: String,
    pub stop_words: HashSet<&'a str>,
    pub not_allowed: HashSet<&'a str>,
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
            not_allowed: ["&", "#", "?", "%", "<", ">", "\"", "'", "/"]
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
        let mut permalink = slug.trim().to_lowercase().to_string();
        permalink = slug.replace(' ', &self.separator);

        // TODO - the below mapping removes uppercase chars

        // Get rid of unwanted characters
        permalink = permalink
            .chars()
            .map(|ch| match ch {
                'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm'
                | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z'
                | '-' | '_' => ch.to_string().to_lowercase().to_string(),
                _ => String::new(),
            })
            .collect();

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

        permalink
    }
}
