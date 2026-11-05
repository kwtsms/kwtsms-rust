//! FILE:      tests/user_agent_test.rs
//! CREATED:   2026-11-05 14:14 +03
//! TASK:      #939 (int-measure-signups)
//! AUTHOR:    M Alhassoun
//! WHAT:      Unit tests for the User-Agent the client sends.
//! WHY:       kwtSMS counts accounts per integration from the User-Agent; it must be
//!            "kwtsms-rust/<Cargo.toml version>" and a caller's app token must never inject headers.
//! USAGE:     cargo test --test user_agent_test
//! DEPENDS:   no network, no credentials.

use kwtsms::{build_user_agent, KwtSms, VERSION};

fn is_lib_token(s: &str) -> bool {
    match s.strip_prefix("kwtsms-rust/") {
        Some(v) => {
            v.starts_with(|c: char| c.is_ascii_digit())
                && v.chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '+' | '-'))
        }
        None => false,
    }
}

#[test]
fn test_version_equals_manifest() {
    assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
}

#[test]
fn test_default_user_agent_is_library_token() {
    let sms = KwtSms::new("rust_username", "rust_password", None, true, None).unwrap();
    assert_eq!(sms.user_agent(), format!("kwtsms-rust/{}", VERSION));
    assert!(is_lib_token(sms.user_agent()));
}

#[test]
fn test_app_token_prepended() {
    let sms = KwtSms::new("rust_username", "rust_password", None, true, None)
        .unwrap()
        .with_app_name("kwtsms-wordpress/3.5.2");
    assert_eq!(
        sms.user_agent(),
        format!("kwtsms-wordpress/3.5.2 kwtsms-rust/{}", VERSION)
    );
}

#[test]
fn test_empty_app_token_gives_library_token() {
    assert_eq!(
        build_user_agent("  \r\n "),
        format!("kwtsms-rust/{}", VERSION)
    );
}

#[test]
fn test_sanitizer_removes_crlf_and_spaces() {
    let ua = build_user_agent("my app\r\nX-Injected: 1");
    assert!(!ua.contains('\r') && !ua.contains('\n'));
    assert!(!ua.contains(" X-Injected"));
    assert_eq!(ua, format!("my-appX-Injected:-1 kwtsms-rust/{}", VERSION));
}

#[test]
fn test_sanitizer_limits_length_byte_safe() {
    let ua = build_user_agent(&"a".repeat(200));
    let app = ua.split(' ').next().unwrap();
    assert_eq!(app.len(), 64);
}

// 2026-11-05 14:17 +03: non-ASCII is dropped, ureq refuses such a header (#939).
#[test]
fn test_sanitizer_drops_non_ascii() {
    let ua = build_user_agent("\u{0643}\u{0648}shop/1.0");
    assert!(ua.is_ascii());
    assert_eq!(ua, format!("shop/1.0 kwtsms-rust/{}", VERSION));
    assert_eq!(
        build_user_agent(&"\u{0643}".repeat(100)),
        format!("kwtsms-rust/{}", VERSION)
    );
}
