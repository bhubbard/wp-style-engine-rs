pub const VARIABLE_REFERENCE_PREFIX: &str = "var:";
pub const VARIABLE_PATH_SEPARATOR_TOKEN_ATTRIBUTE: char = '|';
pub const VARIABLE_PATH_SEPARATOR_TOKEN_STYLE: &str = "--";

/// Capitalizes the first letter in a string.
pub fn upper_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Converts an array of strings into a camelCase string.
pub fn camel_case_join(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let mut result = strings[0].to_lowercase();
    for s in &strings[1..] {
        result.push_str(&upper_first(s));
    }
    result
}

/// Converts a string to kebab-case, mimicking the custom regex word-boundary split of @wordpress/style-engine.
pub fn kebab_case(s: &str) -> String {
    let mut words = Vec::new();
    let mut current_word = String::new();
    let chars: Vec<char> = s.chars().collect();
    
    let mut i = 0;
    while i < chars.len() {
        let curr = chars[i];
        
        if !curr.is_alphanumeric() {
            if !current_word.is_empty() {
                words.push(current_word.to_lowercase());
                current_word = String::new();
            }
            i += 1;
            continue;
        }
        
        if !current_word.is_empty() {
            let prev = chars[i - 1];
            let mut split = false;
            
            // 1. Lower/Digit to Upper: aB, 3B
            if (prev.is_lowercase() || prev.is_numeric()) && curr.is_uppercase() {
                split = true;
            }
            // 2. Digit to Lower: 3b
            else if prev.is_numeric() && curr.is_lowercase() {
                split = true;
            }
            // 3. Alpha to Digit: a3, B3
            else if prev.is_alphabetic() && curr.is_numeric() {
                split = true;
            }
            // 4. FOOBar case: Upper Upper Lower
            else if prev.is_uppercase() && curr.is_uppercase() {
                if i + 1 < chars.len() && chars[i + 1].is_lowercase() {
                    split = true;
                }
            }
            
            if split {
                words.push(current_word.to_lowercase());
                current_word = String::new();
            }
        }
        
        current_word.push(curr);
        i += 1;
    }
    
    if !current_word.is_empty() {
        words.push(current_word.to_lowercase());
    }
    
    words.join("-")
}

/// Safely decodes a URI with decodeURI behavior. Returns the URI unmodified if malformed percent-encoded sequences are detected.
pub fn safe_decode_uri(uri: &str) -> String {
    let mut decoded = Vec::new();
    let bytes = uri.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            if i + 2 < bytes.len() {
                let h1_char = bytes[i + 1] as char;
                let h2_char = bytes[i + 2] as char;
                if let (Some(h1), Some(h2)) = (h1_char.to_digit(16), h2_char.to_digit(16)) {
                    decoded.push((h1 * 16 + h2) as u8);
                    i += 3;
                    continue;
                }
            }
            return uri.to_string();
        } else {
            decoded.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(decoded).unwrap_or_else(|_| uri.to_string())
}

/// Encodes a URI mimicking JS encodeURI. Escapes all characters except A-Z a-z 0-9 ; , / ? : @ & = + $ - _ . ! ~ * ' ( ) #
pub fn encode_uri(uri: &str) -> String {
    let mut encoded = String::new();
    for b in uri.as_bytes() {
        let c = *b as char;
        if c.is_ascii_alphanumeric() || ";,/?:@&=+$-_.!~*'()#".contains(c) {
            encoded.push(c);
        } else {
            encoded.push_str(&format!("%{:02X}", b));
        }
    }
    encoded
}

/// Resolves a WordPress CSS variable preset string (e.g. `var:preset|color|sky-blue` -> `var(--wp--preset--color--sky-blue)`).
pub fn get_css_value_from_raw_style(style_value: &str) -> String {
    if style_value.starts_with(VARIABLE_REFERENCE_PREFIX) {
        let sliced = &style_value[VARIABLE_REFERENCE_PREFIX.len()..];
        let segments: Vec<String> = sliced
            .split(VARIABLE_PATH_SEPARATOR_TOKEN_ATTRIBUTE)
            .map(|segment| kebab_case(segment))
            .collect();
        let variable = segments.join(VARIABLE_PATH_SEPARATOR_TOKEN_STYLE);
        format!("var(--wp--{})", variable)
    } else {
        style_value.to_string()
    }
}

/// Dynamic Value variation for getCSSValueFromRawStyle (generic processing)
pub fn get_css_value_from_raw_style_value(val: &serde_json::Value) -> serde_json::Value {
    if let serde_json::Value::String(s) = val {
        serde_json::Value::String(get_css_value_from_raw_style(s))
    } else {
        val.clone()
    }
}
