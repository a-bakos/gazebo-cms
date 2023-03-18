use crate::consts;

pub fn create_permalink_from(slug: String) -> String {
    let mut permalink = slug.trim().to_lowercase().to_string();
    permalink = slug.replace(' ', consts::PERMALINK_SEPARATOR);

    // TODO - the below mapping removes uppercase chars

    // Get rid of unwanted characters
    permalink = permalink
        .chars()
        .map(|ch| match ch {
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n'
            | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' | '-' | '_' => {
                ch.to_string().to_lowercase().to_string()
            }
            _ => String::new(),
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
