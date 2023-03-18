use crate::consts;

pub fn create_permalink_from(slug: String) -> String {
    let mut permalink = slug.trim().to_lowercase().to_string();
    permalink = slug.replace(' ', consts::PERMALINK_SEPARATOR);

    // Get rid of unwanted characters
    let permalink: String = permalink
        .chars()
        .map(|ch| match ch {
            '@' | '#' => String::new(),
            _ => ch.to_string(),
        })
        .collect();

    // Remove duplicated hyphens
    let permalink: String = permalink.chars().fold(String::new(), |mut acc, ch| {
        if ch == consts::PERMALINK_SEPARATOR.chars().next().unwrap()
            && acc.ends_with(consts::PERMALINK_SEPARATOR)
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
