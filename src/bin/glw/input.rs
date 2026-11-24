//! Parsing de parâmetros, corpos e entrada do stdin.

use serde_json::{Map, Value};

use crate::error::CliError;
use crate::io;

/// Lê o próximo valor de uma linha do stdin; usado por `@-`.
pub fn read_line_from_stdin(what: &str) -> Result<String, CliError> {
    io::read_stdin()
        .map(|content| content.lines().next().unwrap_or_default().trim().to_owned())
        .map_err(|error: CliError| {
            CliError::io(format!("failed to read {what} from stdin: {error}"))
        })
}

/// Resolve um valor que pode ser literal, `@caminho` ou `@-` (stdin).
pub fn resolve_value(value: &str) -> Result<String, CliError> {
    value.strip_prefix('@').map_or_else(
        || Ok(value.to_owned()),
        |path| {
            if path == "-" {
                io::read_stdin()
            } else {
                io::read_file(path)
            }
        },
    )
}

/// Converte `JSON` ou `chave=valor,chave=valor` em um objeto JSON.
///
/// Chaves repetidas em `k=v` são permitidas (tornam-se arrays, como em query
/// strings repetidas); em JSON, um objeto é exigido.
pub fn parse_object(input: &str) -> Result<Map<String, Value>, CliError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(Map::new());
    }
    if trimmed.starts_with('{') {
        let value: Value = serde_json::from_str(trimmed)?;
        return value
            .as_object()
            .cloned()
            .ok_or_else(|| CliError::usage("expected a JSON object for parameters/query"));
    }
    let mut map = Map::new();
    for item in trimmed
        .split([',', '\n'])
        .filter(|item| !item.trim().is_empty())
    {
        let item = item.trim();
        if item.is_empty() {
            continue;
        }
        let (key, value) = item.split_once('=').ok_or_else(|| {
            CliError::usage(format!(
                "parâmetro inválido '{item}'; use --params 'chave=valor,outra=valor', JSON objeto ou @- com linhas chave=valor. Veja 'glw help <verbo>'."
            ))
        })?;
        let key = key.trim();
        if key.is_empty() {
            return Err(CliError::usage(
                "chave de parâmetro vazia; use --params 'chave=valor'. Veja 'glw help <verbo>'.",
            ));
        }
        insert_value(&mut map, key, value.trim());
    }
    Ok(map)
}

/// Insere um valor textual, acumulando chaves repetidas em arrays.
fn insert_value(map: &mut Map<String, Value>, key: &str, value: &str) {
    match map.get_mut(key) {
        Some(Value::Array(values)) => values.push(Value::String(value.to_owned())),
        Some(existing) => {
            let previous = std::mem::replace(existing, Value::Null);
            *existing = Value::Array(vec![previous, Value::String(value.to_owned())]);
        }
        None => {
            map.insert(key.to_owned(), Value::String(value.to_owned()));
        }
    }
}

/// Converte um objeto JSON em pares de query ordenados e determinísticos.
pub fn object_to_query(map: &Map<String, Value>) -> Vec<(String, String)> {
    let mut entries: Vec<_> = map.iter().collect();
    entries.sort_by_key(|(key, _)| *key);
    let mut query = Vec::with_capacity(entries.len());
    for (key, value) in entries {
        match value {
            Value::Null => {}
            Value::Array(values) => {
                for item in values {
                    query.push((key.clone(), scalar_to_string(item)));
                }
            }
            Value::Object(_) => {
                // Objetos aninhados são achatados como `chave[subchave]`.
                if let Some(inner) = value.as_object() {
                    let mut nested: Vec<_> = inner.iter().collect();
                    nested.sort_by_key(|(key, _)| *key);
                    for (subkey, subvalue) in nested {
                        query.push((format!("{key}[{subkey}]"), scalar_to_string(subvalue)));
                    }
                }
            }
            _ => query.push((key.clone(), scalar_to_string(value))),
        }
    }
    query
}

/// Converte um valor JSON escalar em string de query.
fn scalar_to_string(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// Lê um corpo JSON de um literal, `@arquivo` ou `@-` (stdin).
pub fn parse_body(input: &str) -> Result<Value, CliError> {
    let resolved = resolve_value(input)?;
    serde_json::from_str(&resolved)
        .map_err(|error| CliError::usage(format!("invalid JSON body: {error}")))
}

/// Constrói um objeto JSON para desserialização em filtros tipados.
///
/// Valores de `k=v` chegam como strings; campos booleanos e numéricos conhecidos
/// são convertidos para os tipos correspondentes para que `serde` aceite o filtro.
pub fn filter_object(
    query: &[(String, String)],
    bool_keys: &[&str],
    int_keys: &[&str],
) -> Map<String, Value> {
    let mut map = Map::new();
    for (key, value) in query {
        let value = if bool_keys.iter().any(|known| *known == key) {
            match value.as_str() {
                "true" => Value::Bool(true),
                "false" => Value::Bool(false),
                _ => Value::String(value.clone()),
            }
        } else if int_keys.iter().any(|known| *known == key) {
            value
                .parse::<u64>()
                .map_or_else(|_| Value::String(value.clone()), Value::from)
        } else {
            Value::String(value.clone())
        };
        map.insert(key.clone(), value);
    }
    map
}
