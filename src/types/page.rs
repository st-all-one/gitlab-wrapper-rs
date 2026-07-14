use serde::{Deserialize, Serialize};

/// Configurações de GitLab Pages para um projeto.
///
/// Usa `serde_json::Value` para campos dinâmicos, pois o payload de
/// atualização aceita várias combinações de parâmetros.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PageSettings {
    /// Indica se o GitLab Pages está ativo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// URL de produção do site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Domínio personalizado configurado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Certificado SSL do domínio personalizado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// Chave privada do certificado SSL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_key: Option<String>,
    /// Campos extras não mapeados.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
