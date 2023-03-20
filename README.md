# WIP Rust CMS - A thought experiment based on WordPress

This is a project for myself to practice and grow my Rust skills. The main idea behind this project is to explore and
re-create some parts of WordPress using Rust.

 Some todo ideas
 
 - http mod

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


