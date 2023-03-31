use std::collections::HashMap;

#[allow(dead_code)]
pub fn http_status_code_map() -> HashMap<u32, String> {
    let mut map = HashMap::new();
    map.insert(100, "Continue".to_string());
    map.insert(101, "Switching Protocols".to_string());
    map.insert(102, "Processing".to_string());
    map.insert(103, "Early Hints".to_string());
    map.insert(200, "OK".to_string());
    map.insert(201, "Created".to_string());
    map.insert(202, "Accepted".to_string());
    map.insert(203, "Non-Authoritative Information".to_string());
    map.insert(204, "No Content".to_string());
    map.insert(205, "Reset Content".to_string());
    map.insert(206, "Partial Content".to_string());
    map.insert(207, "Multi-Status".to_string());
    map.insert(226, "IM Used".to_string());
    map.insert(300, "Multiple Choices".to_string());
    map.insert(301, "Moved Permanently".to_string());
    map.insert(302, "Found".to_string());
    map.insert(303, "See Other".to_string());
    map.insert(304, "Not Modified".to_string());
    map.insert(305, "Use Proxy".to_string());
    map.insert(306, "Reserved".to_string());
    map.insert(307, "Temporary Redirect".to_string());
    map.insert(308, "Permanent Redirect".to_string());
    map.insert(400, "Bad Request".to_string());
    map.insert(401, "Unauthorized".to_string());
    map.insert(402, "Payment Required".to_string());
    map.insert(403, "Forbidden".to_string());
    map.insert(404, "Not Found".to_string());
    map.insert(405, "Method Not Allowed".to_string());
    map.insert(406, "Not Acceptable".to_string());
    map.insert(407, "Proxy Authentication Required".to_string());
    map.insert(408, "Request Timeout".to_string());
    map.insert(409, "Conflict".to_string());
    map.insert(410, "Gone".to_string());
    map.insert(411, "Length Required".to_string());
    map.insert(412, "Precondition Failed".to_string());
    map.insert(413, "Request Entity Too Large".to_string());
    map.insert(414, "Request-URI Too Long".to_string());
    map.insert(415, "Unsupported Media Type".to_string());
    map.insert(416, "Requested Range Not Satisfiable".to_string());
    map.insert(417, "Expectation Failed".to_string());
    map.insert(418, "I\'m a teapot".to_string());
    map.insert(421, "Misdirected Request".to_string());
    map.insert(422, "Unprocessable Entity".to_string());
    map.insert(423, "Locked".to_string());
    map.insert(424, "Failed Dependency".to_string());
    map.insert(426, "Upgrade Required".to_string());
    map.insert(428, "Precondition Required".to_string());
    map.insert(429, "Too Many Requests".to_string());
    map.insert(431, "Request Header Fields Too Large".to_string());
    map.insert(451, "Unavailable For Legal Reasons".to_string());
    map.insert(500, "Internal Server Error".to_string());
    map.insert(501, "Not Implemented".to_string());
    map.insert(502, "Bad Gateway".to_string());
    map.insert(503, "Service Unavailable".to_string());
    map.insert(504, "Gateway Timeout".to_string());
    map.insert(505, "HTTP Version Not Supported".to_string());
    map.insert(506, "Variant Also Negotiates".to_string());
    map.insert(507, "Insufficient Storage".to_string());
    map.insert(510, "Not Extended".to_string());
    map.insert(511, "Network Authentication Required".to_string());

    map
}
