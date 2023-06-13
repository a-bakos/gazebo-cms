# This file is just a collection of random ideas at this point

Technologies:

- Rust, Warp
- PostgreSQL
- Yew, WebAssembly 
- Tailwind CSS, Bulma CSS
- Main crates: sqlx, warp, tokio, serde, yew

// todo rename GB_Post to GB_Entry

// todo consider gb_config table : to store cms settings

// todo consider gb_status table : eg. for logged in user IDs ??

// todo gb_log table : whodunnit log

idea: 

```rust
struct GB_LogItem {
    date: String,
    user_id: u32,
    entry_id: u32,
    event_code: u32,
    note: String,
}
```

// todo: menu item entry type

idea: 

```rust
struct GB_MenuItem {
    date: String,
    added_by: u32,
    title: String,
    entry_id: Option<EntryID>,
    url: String
}

---

Test structure:

```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {}
}
```

---

# Idea: GB_Query struct

### WP Query args:

```php
$args = [
    'author'                 => '',         // Retrieve posts by Author ID or comma-separated list of author IDs
    'author__in'             => array(),    // Display posts by specific author's user ID(s)
    'author__not_in'         => array(),    // Exclude posts by specific author's user ID(s)
    'author_name'            => '',         // Display posts by specific author's username
    'cache_results'          => true,       // Cache query results or not (true/false)
    'cat'                    => '',         // Category ID or comma-separated list of category IDs
    'category__and'          => array(),    // Retrieve posts assigned to specific categories by ID (AND condition)
    'category__in'           => array(),    // Display posts assigned to specific categories by ID
    'category__not_in'       => array(),    // Exclude posts assigned to specific categories by ID
    'category_name'          => '',         // Display posts assigned to specific categories by slug
    'comment_fields'         => '',         // Specify fields to retrieve for the comments
    'comments_per_page'      => '',         // Number of comments to display per page
    'custom_field_key'       => '',         // Retrieve posts with specific custom field key
    'custom_field_value'     => '',         // Retrieve posts with specific custom field value
    'custom_fields'          => '',         // Retrieve posts with specific custom fields
    'date_query'             => array(),     // Date query parameters (array)
    'day'                    => '',         // Display posts published on a specific day
    'error'                  => '',         // Query error handling
    'feed'                   => '',         // Feed type to retrieve
    'fields'                 => 'all',      // Specify fields to retrieve for the posts (e.g., 'ids', 'names', 'all', 'count')
    'hour'                   => '',         // Display posts published in a specific hour
    'ignore_custom_sort'     => false,      // Ignore custom sorting or not (true/false)
    'ignore_filters'         => false,      // Ignore filters or not (true/false)
    'ignore_sticky_posts'    => false,   // Ignore sticky posts or not (true/false)
    'issearch'               => '',         // Perform a keyword search (alternative to 'search')
    'm'                      => '',         // Display posts with specific post IDs
    'meta_compare'           => '',         // Comparison operator for custom field value (e.g., '=', '>', '<')
    'meta_key'               => 'custom_field', // Retrieve posts with specific custom field key
    'meta_query'             => array(),     // Custom field (meta) query parameters (array)
    'meta_value'             => 'value',    // Retrieve posts with specific custom field value
    'minute'                 => '',         // Display posts published in a specific minute
    'monthnum'               => '',         // Display posts published in a specific month
    'monthnum__and'          => array(),    // Display posts published in specific months (AND condition)
    'monthnum__in'           => array(),    // Display posts published in specific months (IN condition)
    'monthnum__not_in'       => array(),    // Exclude posts published in specific months
    'name'                   => '',         // Display posts by post name (slug)
    'no_found_rows'          => false,      // Disable SQL_CALC_FOUND_ROWS in the query (true/false)
    'nopaging'               => false,       // Show all posts or paginate (true/false)
    'offset'                 => '',         // Number of posts to offset in the query
    'order'                  => 'DESC',     // Sort order for posts (e.g., 'ASC', 'DESC')
    'orderby'                => 'date',     // Order posts by specific criteria (e.g., 'date', 'title', 'rand')
    'p'                      => '',         // Display specific posts by ID
    'page'                   => '',         // Current page number
    'paged'                  => '',         // Current page of results
    'pagename'               => '',         // Display posts by page name (slug)
    'post__in'               => array(),    // Retrieve posts specified by an array of post IDs
    'post__not_in'           => array(),    // Exclude posts specified by an array of post IDs
    'post_fields'            => '',         // Specify fields to retrieve for the posts (e.g., 'ids', 'names', 'all', 'count')
    'post_mime_type'         => '',         // Retrieve posts of a specific mime type
    'post_name__in'          => array(),    // Display posts with specific post slugs
    'post_parent'            => '',         // Display child posts of specific parent post ID
    'post_parent__in'        => array(),    // Display child posts of any of the specified parent post IDs
    'post_parent__not_in'    => array(),     // Exclude child posts of specific parent post IDs
    'post_status'            => '',         // Post status (e.g., 'publish', 'draft', 'private')
    'post_type'              => 'post',     // Display posts of specific post types (e.g., 'post', 'page', 'custom_post_type')
    'post_type__in'          => array(),    // Display posts of specific post types (AND condition)
    'post_type__not_in'      => array(),    // Exclude posts of specific post types
    'posts_per_archive_page' => '',         // Number of posts to display per page in archives
    'posts_per_page'         => '',         // Number of posts to retrieve
    'preview'                => '',         // Preview post status or not (true/false)
    'robotsmeta'             => '',         // Robots meta tag value
    's'                      => '',         // Perform a keyword search (alternative to 'search')
    'search'                 => '',         // Perform a keyword search
    'search_terms'           => '',         // Perform a keyword search (alternative to 'search')
    'second'                 => '',         // Display posts published in a specific second
    'sentence'               => '',         // Perform a keyword search (alternative to 'search')
    'suppress_filters'       => false,      // Suppress filters or not (true/false)
    'tag'                    => '',         // Display posts assigned to specific tags by slug
    'tag__and'               => array(),    // Display posts assigned to all of the specified tags by ID
    'tag__in'                => array(),    // Display posts assigned to any of the specified tags by ID
    'tag__not_in'            => array(),    // Exclude posts assigned to specific tags by ID
    'tag_id'                 => '',         // Display posts assigned to specific tags by ID
    'tag_slug__and'          => array(),    // Display posts assigned to all of the specified tags by slug
    'tag_slug__in'           => array(),    // Display posts assigned to any of the specified tags by slug
    'tag_slug__not_in'       => array(),    // Exclude posts assigned to specific tags by slug
    'tax_query'              => array(),     // Taxonomy query parameters (array)
    'title'                  => '',         // Display posts by specific title
    'update_post_term_cache' => true,       // Update post term cache or not (true/false)
    'w'                      => '',         // Display posts published in a specific week
    'year'                   => '',         // Display posts published in a specific year
    'year__and'              => array(),    // Display posts published in specific years (AND condition)
    'year__in'               => array(),    // Display posts published in specific years (IN condition)
    'year__not_in'           => array()     // Exclude posts published in specific years
];
```

```rust
struct GB_Query(Vec<GB_QueryArg>);

enum GB_QueryArg {
    AuthorID(Vec<UserID>),
    ID(Vec<EntryID>),
    Title(String),
    EntryType(Vec<EntryType>), // like post_type
    EntryStatus(EntryStatus), // like post_status
}
```
---

## Roles and caps in WP

### Super Admin

- create_sites
- delete_sites
- manage_network
- manage_sites
- manage_network_users
- manage_network_plugins
- manage_network_themes
- manage_network_options
- upgrade_network
- setup_network

### Administrator

- activate_plugins
- delete_others_pages
- delete_others_posts
- delete_pages
- delete_posts
- delete_private_pages
- delete_private_posts
- delete_published_pages
- delete_published_posts
- edit_dashboard
- edit_others_pages
- edit_others_posts
- edit_pages
- edit_posts
- edit_private_pages
- edit_private_posts
- edit_published_pages
- edit_published_posts
- edit_theme_options
- export
- import
- list_users
- manage_categories
- manage_links
- manage_options
- moderate_comments
- promote_users
- publish_pages
- publish_posts
- read_private_pages
- read_private_posts
- read
- create Reusable Blocks
- edit Reusable Blocks
- read Reusable Blocks
- delete Reusable Blocks
- remove_users
- switch_themes
- upload_files
- customize
- delete_site

#### Additional admin capabilities

- update_core
- update_plugins
- update_themes
- install_plugins
- install_themes
- delete_themes
- delete_plugins
- edit_plugins
- edit_themes
- edit_files
- edit_users
- add_users
- create_users
- delete_users
- unfiltered_html

### Editor

- delete_others_pages
- delete_others_posts
- delete_pages
- delete_posts
- delete_private_pages
- delete_private_posts
- delete_published_pages
- delete_published_posts
- delete Reusable Blocks
- edit_others_pages
- edit_others_posts
- edit_pages
- edit_posts
- edit_private_pages
- edit_private_posts
- edit_published_pages
- edit_published_posts
- create Reusable Blocks
- edit Reusable Blocks
- manage_categories
- manage_links
- moderate_comments
- publish_pages
- publish_posts
- read
- read_private_pages
- read_private_posts
- unfiltered_html (not with Multisite)
- upload_files

### Author

- delete_posts
- delete_published_posts
- edit_posts
- edit_published_posts
- publish_posts
- read
- upload_files
- create Reusable Blocks
- read Reusable Blocks
- edit Reusable Blocks (own)
- delete Reusable Blocks (own)

### Contributor

- delete_posts
- edit_posts
- read
- read Reusable Blocks

### Subscriber

- read

---

Some todo ideas

- anyhow for errors
 
```php
/**
 * Serializes data, if needed.
 *
 * @since 2.0.5
 *
 * @param string|array|object $data Data that might be serialized.
 * @return mixed A scalar data.
 */
function maybe_serialize( $data ) {
	if ( is_array( $data ) || is_object( $data ) ) {
		return serialize( $data );
	}

	/*
	 * Double serialization is required for backward compatibility.
	 * See https://core.trac.wordpress.org/ticket/12930
	 * Also the world will end. See WP 3.6.1.
	 */
	if ( is_serialized( $data, false ) ) {
		return serialize( $data );
	}

	return $data;
}

/**
 * Unserializes data only if it was serialized.
 *
 * @since 2.0.0
 *
 * @param string $data Data that might be unserialized.
 * @return mixed Unserialized data can be any type.
 */
function maybe_unserialize( $data ) {
	if ( is_serialized( $data ) ) { // Don't attempt to unserialize data that wasn't serialized going in.
		return @unserialize( trim( $data ) );
	}

	return $data;
}

/**
 * Checks value to find if it was serialized.
 *
 * If $data is not a string, then returned value will always be false.
 * Serialized data is always a string.
 *
 * @since 2.0.5
 * @since 6.1.0 Added Enum support.
 *
 * @param string $data   Value to check to see if was serialized.
 * @param bool   $strict Optional. Whether to be strict about the end of the string. Default true.
 * @return bool False if not serialized and true if it was.
 */
function is_serialized( $data, $strict = true ) {
	// If it isn't a string, it isn't serialized.
	if ( ! is_string( $data ) ) {
		return false;
	}
	$data = trim( $data );
	if ( 'N;' === $data ) {
		return true;
	}
	if ( strlen( $data ) < 4 ) {
		return false;
	}
	if ( ':' !== $data[1] ) {
		return false;
	}
	if ( $strict ) {
		$lastc = substr( $data, -1 );
		if ( ';' !== $lastc && '}' !== $lastc ) {
			return false;
		}
	} else {
		$semicolon = strpos( $data, ';' );
		$brace     = strpos( $data, '}' );
		// Either ; or } must exist.
		if ( false === $semicolon && false === $brace ) {
			return false;
		}
		// But neither must be in the first X characters.
		if ( false !== $semicolon && $semicolon < 3 ) {
			return false;
		}
		if ( false !== $brace && $brace < 4 ) {
			return false;
		}
	}
	$token = $data[0];
	switch ( $token ) {
		case 's':
			if ( $strict ) {
				if ( '"' !== substr( $data, -2, 1 ) ) {
					return false;
				}
			} elseif ( false === strpos( $data, '"' ) ) {
				return false;
			}
			// Or else fall through.
		case 'a':
		case 'O':
		case 'E':
			return (bool) preg_match( "/^{$token}:[0-9]+:/s", $data );
		case 'b':
		case 'i':
		case 'd':
			$end = $strict ? '$' : '';
			return (bool) preg_match( "/^{$token}:[0-9.E+-]+;$end/", $data );
	}
	return false;
}

/**
 * Checks whether serialized data is of string type.
 *
 * @since 2.0.5
 *
 * @param string $data Serialized data.
 * @return bool False if not a serialized string, true if it is.
 */
function is_serialized_string( $data ) {
	// if it isn't a string, it isn't a serialized string.
	if ( ! is_string( $data ) ) {
		return false;
	}
	$data = trim( $data );
	if ( strlen( $data ) < 4 ) {
		return false;
	} elseif ( ':' !== $data[1] ) {
		return false;
	} elseif ( ';' !== substr( $data, -1 ) ) {
		return false;
	} elseif ( 's' !== $data[0] ) {
		return false;
	} elseif ( '"' !== substr( $data, -2, 1 ) ) {
		return false;
	} else {
		return true;
	}
}

/**
 * Uses RegEx to extract URLs from arbitrary content.
 *
 * @since 3.7.0
 * @since 6.0.0 Fixes support for HTML entities (Trac 30580).
 *
 * @param string $content Content to extract URLs from.
 * @return string[] Array of URLs found in passed string.
 */
function wp_extract_urls( $content ) {
	preg_match_all(
		"#([\"']?)("
			. '(?:([\w-]+:)?//?)'
			. '[^\s()<>]+'
			. '[.]'
			. '(?:'
				. '\([\w\d]+\)|'
				. '(?:'
					. "[^`!()\[\]{}:'\".,<>«»“”‘’\s]|"
					. '(?:[:]\d+)?/?'
				. ')+'
			. ')'
		. ")\\1#",
		$content,
		$post_links
	);

	$post_links = array_unique(
		array_map(
			static function( $link ) {
				// Decode to replace valid entities, like &amp;.
				$link = html_entity_decode( $link );
				// Maintain backward compatibility by removing extraneous semi-colons (`;`).
				return str_replace( ';', '', $link );
			},
			$post_links[2]
		)
	);

	return array_values( $post_links );
}

/**
 * Builds URL query based on an associative and, or indexed array.
 *
 * This is a convenient function for easily building url queries. It sets the
 * separator to '&' and uses _http_build_query() function.
 *
 * @since 2.3.0
 *
 * @see _http_build_query() Used to build the query
 * @link https://www.php.net/manual/en/function.http-build-query.php for more on what
 *       http_build_query() does.
 *
 * @param array $data URL-encode key/value pairs.
 * @return string URL-encoded string.
 */
function build_query( $data ) {
	return _http_build_query( $data, null, '&', '', false );
}

/**
 * From php.net (modified by Mark Jaquith to behave like the native PHP5 function).
 *
 * @since 3.2.0
 * @access private
 *
 * @see https://www.php.net/manual/en/function.http-build-query.php
 *
 * @param array|object $data      An array or object of data. Converted to array.
 * @param string       $prefix    Optional. Numeric index. If set, start parameter numbering with it.
 *                                Default null.
 * @param string       $sep       Optional. Argument separator; defaults to 'arg_separator.output'.
 *                                Default null.
 * @param string       $key       Optional. Used to prefix key name. Default empty.
 * @param bool         $urlencode Optional. Whether to use urlencode() in the result. Default true.
 * @return string The query string.
 */
function _http_build_query( $data, $prefix = null, $sep = null, $key = '', $urlencode = true ) {
	$ret = array();

	foreach ( (array) $data as $k => $v ) {
		if ( $urlencode ) {
			$k = urlencode( $k );
		}
		if ( is_int( $k ) && null != $prefix ) {
			$k = $prefix . $k;
		}
		if ( ! empty( $key ) ) {
			$k = $key . '%5B' . $k . '%5D';
		}
		if ( null === $v ) {
			continue;
		} elseif ( false === $v ) {
			$v = '0';
		}

		if ( is_array( $v ) || is_object( $v ) ) {
			array_push( $ret, _http_build_query( $v, '', $sep, $k, $urlencode ) );
		} elseif ( $urlencode ) {
			array_push( $ret, $k . '=' . urlencode( $v ) );
		} else {
			array_push( $ret, $k . '=' . $v );
		}
	}

	if ( null === $sep ) {
		$sep = ini_get( 'arg_separator.output' );
	}

	return implode( $sep, $ret );
}

/**
 * Gets unique ID.
 *
 * This is a PHP implementation of Underscore's uniqueId method. A static variable
 * contains an integer that is incremented with each call. This number is returned
 * with the optional prefix. As such the returned value is not universally unique,
 * but it is unique across the life of the PHP process.
 *
 * @since 5.0.3
 *
 * @param string $prefix Prefix for the returned ID.
 * @return string Unique ID.
 */
function wp_unique_id( $prefix = '' ) {
	static $id_counter = 0;
	return $prefix . (string) ++$id_counter;
}

```


