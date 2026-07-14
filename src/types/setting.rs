/// As configurações da aplicação GitLab são dinâmicas e representadas
/// como `serde_json::Value`. Consulte os métodos em `SettingsResource`
/// para obter e atualizar as configurações.
pub type SettingsValue = serde_json::Value;
