use crate::consts;
use crate::database::db;
use regex::Regex;
use std::collections::HashSet;
use urlencoding::encode;

/**
 * Permalink generator
 *
 * Rules to consider when generating permalinks
 *
 * Make sure the URL is:
 * - SEO-friendly
 * - [done] Consistent
 * - [done] Human readable
 * - [done] Lowercase
 * - [done] Using dashes as separator
 * - [done] Free of special characters
 * - [done] Encoded
 * - [done] Trimmed leading and trailing whitespaces
 * - [done] Limited to 50-60 characters in length
 * - Unique with an appended number, if needed
 */

#[derive(Debug)]
pub enum PermalinkTruncationMethod {
    // Simply cut the string at the end
    CutOffEnd,
    // Remove the least significant words
    #[allow(dead_code)]
    RemoveLeastSignificantWords,
    // Remove longest words
    #[allow(dead_code)]
    RemoveLongestWords,
}

#[derive(Debug)]
struct PermalinksConfig {
    length_limit: usize,
    #[allow(dead_code)]
    allow_unlimited_length: bool,
    #[allow(dead_code)]
    allow_stop_words: bool,
    truncation_method: PermalinkTruncationMethod,
}

impl PermalinksConfig {
    fn new() -> Self {
        Self {
            length_limit: consts::DEFAULT_PERMALINK_LIMIT,
            allow_unlimited_length: false,
            allow_stop_words: false,
            truncation_method: PermalinkTruncationMethod::CutOffEnd, // todo this is test only for now,
        }
    }
}

#[derive(Debug)]
pub struct PermalinkGenerator<'a> {
    pub separator: &'a str,
    pub stop_words: HashSet<&'a str>,
    pub not_allowed_characters: HashSet<&'a str>,
    config: PermalinksConfig,
    pub length_limit: usize,
}

impl<'a> PermalinkGenerator<'a> {
    pub fn new(use_dash_as_separator: bool) -> Self {
        let separator = if use_dash_as_separator {
            consts::DEFAULT_PERMALINK_SEPARATOR
        } else {
            "_"
        };

        Self {
            separator,
            stop_words: consts::DEFAULT_PERMALINK_STOP_WORDS
                .iter()
                .cloned()
                .collect(),
            not_allowed_characters: consts::DEFAULT_PERMALINK_NOT_ALLOWED_CHARS
                .iter()
                .cloned()
                .collect(),
            length_limit: consts::DEFAULT_PERMALINK_LIMIT,
            config: PermalinksConfig::new(),
        }
    }

    #[allow(dead_code)]
    pub fn allow_unlimited_length(&mut self, allow_unlimited: bool) {
        if allow_unlimited {
            self.config.allow_unlimited_length = true;
        }
    }

    #[allow(dead_code)]
    pub fn set_max_length(&mut self, length: usize) {
        if length < consts::PERMALINK_MAX_ALLOWED_LENGTH && length > 0 {
            self.config.length_limit = length;
            self.config.allow_unlimited_length = false;
        }
    }

    #[allow(dead_code)]
    pub fn allow_stop_words(&mut self, allow_stop_words: bool) {
        if allow_stop_words {
            self.config.allow_stop_words = true;
        }
    }

    #[allow(dead_code)]
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

    fn maybe_remove_stop_words(&self, permalink: String) -> Vec<String> {
        let permalink_as_words: Vec<&str> = permalink.split(' ').collect();
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
        let permalink = slug.trim().to_lowercase();
        // Get rid of unwanted characters
        let permalink = self.remove_unwanted_characters(permalink);
        // Remove stop words
        let permalink_as_words = self.maybe_remove_stop_words(permalink);
        // Join words by separator character
        let permalink = permalink_as_words.join(self.separator);
        // Remove duplicated dashes
        let permalink = self.remove_duplicate_dashes(permalink);
        // Maybe limit length of permalink
        let permalink = self.maybe_limit_length(permalink);

        // todo
        // check for uniqueness
        let permalink = self.make_unique(permalink);

        // URL Encoding
        let permalink = self.encode_permalink(permalink);

        // Return the final permalink
        #[allow(clippy::let_and_return)]
        permalink
    }

    fn make_unique(&self, permalink: String) -> String {
        // Get all posts, clean results, only keep a list of permalinks
        let mut all_permalinks: HashSet<String> = HashSet::new();
        if db::parse_csv(consts::FILE_PATH_POSTS).is_ok() {
            let csv_db = db::parse_csv(consts::FILE_PATH_POSTS).ok().unwrap();
            for post in csv_db.iter() {
                let post_permalink = post
                    .get(crate::database::columns::COL_INDEX_POST_SLUG)
                    .unwrap()
                    .to_string();
                all_permalinks.insert(post_permalink);
            }
        }
        if !all_permalinks.is_empty() {
            for link in all_permalinks.iter() {
                if link.as_str() == permalink.as_str() {
                    // links are the same

                    // get the stored permalink and extract the digit
                    let permalink_str = link.as_str();
                    let re = Regex::new(r"-(\d+)$").unwrap();
                    if re.is_match(permalink_str) {
                        if let Some(captures) = re.captures(permalink_str) {
                            if let Some(num_str) = captures.get(1) {
                                if let Ok(mut the_permalink_num) = num_str.as_str().parse::<u32>() {
                                    the_permalink_num += 1;
                                    let mut new_permalink = permalink_str
                                        [..permalink_str.len() - num_str.as_str().len()]
                                        .to_string();
                                    new_permalink.push_str(&format!("-{}", the_permalink_num));
                                    return new_permalink;
                                }
                            }
                        }
                    } else {
                        // if no digit, append one
                        let mut new_permalink = permalink.to_string();
                        new_permalink.push_str(&format!("-{}", 1));
                        return new_permalink;
                    }
                }
            }
        }

        permalink
    }

    fn encode_permalink(&self, permalink: String) -> String {
        // Encoding done using the external crate
        encode(&permalink).to_string()
    }

    fn maybe_limit_length(&self, permalink: String) -> String {
        match self.config.truncation_method {
            PermalinkTruncationMethod::CutOffEnd => {
                // Cut the end of the string
                if permalink.chars().count() > self.config.length_limit {
                    return permalink
                        .chars()
                        .take(self.config.length_limit)
                        .collect::<String>();
                }
            }
            PermalinkTruncationMethod::RemoveLeastSignificantWords => {}
            PermalinkTruncationMethod::RemoveLongestWords => {}
        }

        permalink
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn allow_unlimited_permalink_length() {
        let permalink_config = PermalinksConfig::new();
        let mut permalinks = PermalinkGenerator::new(true);
        permalinks.allow_unlimited_length(true);
        assert!(permalinks.config.allow_unlimited_length);
    }

    #[test]
    fn words_joined_by_separator() {
        let permalink_as_words = vec![
            "this".to_string(),
            "is".to_string(),
            "a".to_string(),
            "permalink".to_string(),
            "test".to_string(),
        ];
        let permalink = permalink_as_words.join("-");
        assert_eq!("this-is-a-permalink-test", permalink);
    }
}
