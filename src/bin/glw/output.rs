//! Formatação de saída: JSON, JSONL e tabela.

use serde::Serialize;
use serde_json::Value;

use crate::args::OutputFormat;
use crate::error::CliError;
use crate::io;

/// Escreve um valor serializável no formato escolhido.
pub fn write_value<T: Serialize>(format: OutputFormat, value: &T) -> Result<(), CliError> {
    match format {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(value)?;
            io::write_stdout(&json)
        }
        OutputFormat::Jsonl => write_jsonl(value),
        OutputFormat::Table => {
            let json = serde_json::to_value(value)?;
            for line in table_lines(&json) {
                io::write_stdout(&line)?;
            }
            Ok(())
        }
    }
}

/// Escreve um valor como uma única linha JSON (usado por batch e dry-run).
pub fn write_compact<T: Serialize>(value: &T) -> Result<(), CliError> {
    io::write_stdout(&serde_json::to_string(value)?)
}

/// Escreve cada item de uma sequência em uma linha JSON.
fn write_jsonl<T: Serialize>(value: &T) -> Result<(), CliError> {
    let json = serde_json::to_value(value)?;
    match json {
        Value::Array(items) => {
            for item in &items {
                io::write_stdout(&serde_json::to_string(item)?)?;
            }
            Ok(())
        }
        other => io::write_stdout(&serde_json::to_string(&other)?),
    }
}

/// Converte um valor JSON em linhas de tabela determinísticas.
fn table_lines(value: &Value) -> Vec<String> {
    let items: Vec<&Value> = match value {
        Value::Array(values) => values.iter().collect(),
        Value::Null => Vec::new(),
        other => vec![other],
    };
    if items.is_empty() {
        return vec!["(no results)".to_owned()];
    }

    let mut keys: Vec<String> = Vec::new();
    for item in &items {
        let Some(object) = item.as_object() else {
            return items.iter().map(|item| scalar(item)).collect();
        };
        for key in object.keys() {
            if !keys.iter().any(|existing| existing == key) && !is_complex(&object[key]) {
                keys.push(key.clone());
            }
        }
    }
    keys.sort();
    if keys.is_empty() {
        return items.iter().map(|item| scalar(item)).collect();
    }

    let widths: Vec<usize> = keys
        .iter()
        .map(|key| {
            let value_width = items
                .iter()
                .filter_map(|item| item.as_object()?.get(key))
                .map(|value| scalar(value).chars().count())
                .max()
                .unwrap_or(0);
            key.chars().count().max(value_width)
        })
        .collect();

    let mut lines = Vec::with_capacity(items.len() + 2);
    lines.push(render_row(&keys, &widths));
    lines.push(render_row(
        &widths
            .iter()
            .map(|width| "-".repeat(*width))
            .collect::<Vec<_>>(),
        &widths,
    ));
    for item in &items {
        let cells: Vec<String> = keys
            .iter()
            .map(|key| {
                item.as_object()
                    .and_then(|object| object.get(key))
                    .map_or_else(String::new, scalar)
            })
            .collect();
        lines.push(render_row(&cells, &widths));
    }
    lines
}

/// Verifica se um valor é complexo (objeto/array) e não cabe em uma coluna.
const fn is_complex(value: &Value) -> bool {
    matches!(value, Value::Object(_) | Value::Array(_))
}

/// Renderiza uma linha com larguras fixas.
fn render_row(cells: &[String], widths: &[usize]) -> String {
    cells
        .iter()
        .zip(widths)
        .map(|(cell, width)| format!("{cell:<width$}"))
        .collect::<Vec<_>>()
        .join("  ")
        .trim_end()
        .to_owned()
}

/// Converte um valor escalar em texto de célula.
fn scalar(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}
