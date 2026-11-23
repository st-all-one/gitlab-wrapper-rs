/// Codifica um valor de parâmetro de consulta usando codificação percentual (RFC 3986).
///
/// ## Params
/// - `input`: String a ser codificada.
///
/// ## Returns
/// `String` — valor codificado para uso em URL.
#[must_use]
pub fn encode_query_param(input: &str) -> String {
    url::form_urlencoded::byte_serialize(input.as_bytes()).collect()
}

/// Converte um filtro opcional em um vetor de pares chave-valor para parâmetros de consulta.
///
/// Utiliza serialização `serde` para achatar recursivamente a estrutura do filtro,
/// suportando objetos aninhados (formato `chave[subchave]=valor`) e arrays.
///
/// A ordem de serialização é estável: mesmo com mapas `serde_json::Map` internos
/// (que não preservam ordem de inserção), as chaves de cada objeto são ordenadas
/// lexicalmente antes de gerar os pares. Arrays mantêm a ordem original.
///
/// ## Params
/// - `filter`: Referência opcional ao filtro a ser convertido.
///
/// ## Returns
/// `Vec<(String, String)>` — lista de parâmetros de consulta.
#[must_use]
pub fn filter_to_query<T: serde::Serialize>(filter: Option<&T>) -> Vec<(String, String)> {
    fn inner(val: &serde_json::Value, prefix: &str, query: &mut Vec<(String, String)>) {
        match val {
            serde_json::Value::Object(map) => {
                // `serde_json::Map` usa HashMap internamente; ordenar as chaves
                // elimina a variação de ordem entre execuções.
                let mut entries: Vec<_> = map.iter().collect();
                entries.sort_by_key(|(a, _)| *a);
                for (k, v) in entries {
                    let new_prefix = if prefix.is_empty() {
                        k.clone()
                    } else {
                        format!("{prefix}[{k}]")
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
    if let Some(f) = filter
        && let Ok(val) = serde_json::to_value(f)
    {
        inner(&val, "", &mut query);
    }
    query
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[test]
    fn test_encode_query_param_basic() {
        assert_eq!(encode_query_param("hello"), "hello");
        assert_eq!(encode_query_param("a b"), "a+b");
    }

    #[test]
    fn test_encode_query_param_special_chars() {
        assert_eq!(encode_query_param("a&b=c"), "a%26b%3Dc");
        assert_eq!(
            encode_query_param("path/with/slashes"),
            "path%2Fwith%2Fslashes"
        );
    }

    #[test]
    fn test_filter_to_query_none() {
        let result: Vec<(String, String)> = filter_to_query(None::<&serde_json::Value>);
        assert_eq!(result, Vec::new());
    }

    #[derive(Serialize)]
    struct TestFilter {
        name: String,
        active: bool,
    }

    #[test]
    fn test_filter_to_query_simple() {
        let filter = TestFilter {
            name: "test".into(),
            active: true,
        };
        let result = filter_to_query(Some(&filter));
        assert_eq!(result.len(), 2);
        assert!(result.contains(&("name".into(), "test".into())));
        assert!(result.contains(&("active".into(), "true".into())));
    }
}
