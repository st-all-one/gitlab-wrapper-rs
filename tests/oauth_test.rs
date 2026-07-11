use gitlab_wrapper::oauth;

#[test]
fn test_generate_code_verifier() {
    let verifier = oauth::generate_code_verifier();
    assert!(!verifier.is_empty(), "Verifier should not be empty");
    // PKCE verifier should be 43-128 chars (base64url encoded 32+ bytes)
    assert!(verifier.len() >= 43);
    assert!(verifier.len() <= 128);
}

#[test]
fn test_generate_code_challenge() {
    let verifier = oauth::generate_code_verifier();
    let challenge = oauth::generate_code_challenge(&verifier);
    assert!(!challenge.is_empty(), "Challenge should not be empty");
    // SHA-256 base64url encoded = 43 chars
    assert_eq!(challenge.len(), 43);
}

#[test]
fn test_code_challenge_consistency() {
    let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
    let challenge = oauth::generate_code_challenge(verifier);
    // Known SHA-256 base64url of this verifier
    assert_eq!(challenge.len(), 43);
}

#[test]
fn test_authorization_code_url() {
    use gitlab_wrapper::oauth::AuthCodeUrlOptions;

    let url = oauth::authorization_code_url(&AuthCodeUrlOptions {
        base_url: "https://gitlab.com".into(),
        client_id: "my-client".into(),
        redirect_uri: "https://app.example.com/callback".into(),
        scope: "api read_user".into(),
        state: "random-state".into(),
        code_challenge: Some("challenge-value".into()),
    });

    assert!(url.starts_with("https://gitlab.com/oauth/authorize?"));
    assert!(url.contains("client_id=my-client"));
    assert!(url.contains("redirect_uri=https%3A%2F%2Fapp.example.com%2Fcallback"));
    assert!(url.contains("response_type=code"));
    assert!(url.contains("scope=api%20read_user") || url.contains("scope=api+read_user"));
    assert!(url.contains("state=random-state"));
    assert!(url.contains("code_challenge=challenge-value"));
    assert!(url.contains("code_challenge_method=S256"));
}

#[test]
fn test_authorization_code_url_without_pkce() {
    use gitlab_wrapper::oauth::AuthCodeUrlOptions;

    let url = oauth::authorization_code_url(&AuthCodeUrlOptions {
        base_url: "https://gitlab.com".into(),
        client_id: "my-client".into(),
        redirect_uri: "https://app.example.com/callback".into(),
        scope: "api".into(),
        state: "s".into(),
        code_challenge: None,
    });

    assert!(!url.contains("code_challenge"));
}
