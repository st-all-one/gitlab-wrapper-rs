/// Percent-encode a query parameter value.
/// Safe characters: A-Z, a-z, 0-9, '-', '_', '.', '~'
pub fn encode_query_param(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            b' ' => {
                result.push('%');
                result.push('2');
                result.push('0');
            }
            _ => {
                result.push('%');
                let hex = format!("{:02X}", byte);
                result.push_str(&hex);
            }
        }
    }
    result
}

/// Convert an optional filter struct into a `Vec<(String, String)>` of query parameters.
/// Uses serde serialization to recursively flatten the struct.
pub fn filter_to_query<T: serde::Serialize>(filter: Option<&T>) -> Vec<(String, String)> {
    fn inner(val: &serde_json::Value, prefix: &str, query: &mut Vec<(String, String)>) {
        match val {
            serde_json::Value::Object(map) => {
                for (k, v) in map {
                    let new_prefix = if prefix.is_empty() {
                        k.clone()
                    } else {
                        format!("{}[{}]", prefix, k)
                    };
                    inner(v, &new_prefix, query);
                }
            }
            serde_json::Value::Array(arr) => {
                for v in arr {
                    inner(v, prefix, query);
                }
            }
            serde_json::Value::String(s) => {
                query.push((prefix.to_string(), s.clone()));
            }
            serde_json::Value::Number(n) => {
                query.push((prefix.to_string(), n.to_string()));
            }
            serde_json::Value::Bool(b) => {
                query.push((prefix.to_string(), b.to_string()));
            }
            serde_json::Value::Null => {}
        }
    }

    let mut query = Vec::new();
    if let Some(f) = filter {
        if let Ok(val) = serde_json::to_value(f) {
            inner(&val, "", &mut query);
        }
    }
    query
}
