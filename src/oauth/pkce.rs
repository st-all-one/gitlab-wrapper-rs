//! Utilitários PKCE (Proof Key for Code Exchange) para o fluxo de código de autorização OAuth.
//!
//! Gera um verificador aleatório e seu respectivo desafio (SHA-256 codificado em base64url)
//! de acordo com a especificação RFC 7636.

use base64::Engine as _;
use sha2::{Digest, Sha256};

const VERIFIER_LENGTH: usize = 32;

/// Codificador base64url sem padding.
const B64: base64::engine::GeneralPurpose = base64::engine::GeneralPurpose::new(
    &base64::alphabet::URL_SAFE,
    base64::engine::general_purpose::NO_PAD,
);

/// Gera um verificador PKCE aleatório.
///
/// O verificador é uma sequência de 32 bytes aleatórios, codificada em base64url
/// sem padding, conforme a RFC 7636.
///
/// ## Returns
/// `String` — o verificador PKCE codificado em base64url.
///
/// ## Panics
/// Pode entrar em pânico se a fonte de aleatoriedade do sistema falhar (CSPRNG).
pub fn generate_code_verifier() -> String {
    let mut bytes = vec![0u8; VERIFIER_LENGTH];
    getrandom::getrandom(&mut bytes).expect("CSPRNG failure");
    B64.encode(&bytes)
}

/// Gera o desafio PKCE (`code_challenge`) a partir de um verificador.
///
/// Aplica SHA-256 ao verificador fornecido e codifica o hash em base64url
/// sem padding, conforme a RFC 7636.
///
/// ## Params
/// - `verifier`: O verificador PKCE gerado por [`generate_code_verifier`].
///
/// ## Returns
/// `String` — o desafio PKCE codificado em base64url.
pub fn generate_code_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let hash = hasher.finalize();
    B64.encode(hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_code_verifier_length() {
        let verifier = generate_code_verifier();
        // PKCE verifier should be 43-128 chars (base64url encoded 32+ bytes)
        assert!(verifier.len() >= 43, "verifier too short: {}", verifier.len());
        assert!(verifier.len() <= 128, "verifier too long: {}", verifier.len());
    }

    #[test]
    fn test_generate_code_challenge_length() {
        let verifier = generate_code_verifier();
        let challenge = generate_code_challenge(&verifier);
        // SHA-256 base64url encoded = 43 chars
        assert_eq!(challenge.len(), 43, "challenge should be exactly 43 chars");
    }

    #[test]
    fn test_code_challenge_consistency() {
        let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
        let challenge = generate_code_challenge(verifier);
        assert_eq!(challenge.len(), 43);
    }

    #[test]
    fn test_different_verifiers_produce_different_challenges() {
        let v1 = generate_code_verifier();
        let v2 = generate_code_verifier();
        let c1 = generate_code_challenge(&v1);
        let c2 = generate_code_challenge(&v2);
        assert_ne!(c1, c2, "two random verifiers should produce different challenges");
    }
}
