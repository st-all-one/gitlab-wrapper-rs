use sha2::{Digest, Sha256};

const VERIFIER_LENGTH: usize = 64;

pub fn generate_code_verifier() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    // Generate random-ish bytes using time + a simple counter
    // In production, use a proper CSPRNG (e.g., `rand` crate).
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();

    let mut bytes = Vec::with_capacity(VERIFIER_LENGTH);
    for i in 0..VERIFIER_LENGTH {
        // Simple LCG to generate pseudo-random bytes from seed
        let val = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let shift = (i as u32 % 8) * 8;
        bytes.push(((val >> shift) & 0xFF) as u8);
    }

    base64url_encode(&bytes)
}

pub fn generate_code_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let hash = hasher.finalize();
    base64url_encode(&hash)
}

fn base64url_encode(input: &[u8]) -> String {
    let b64 = base64_simple(input);
    // Convert to base64url (no padding)
    b64.replace('+', "-")
        .replace('/', "_")
        .trim_end_matches('=')
        .to_string()
}

/// Minimal base64 encoder without external dependencies.
fn base64_simple(input: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;

        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);

        if chunk.len() >= 2 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() >= 3 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }

    result
}
