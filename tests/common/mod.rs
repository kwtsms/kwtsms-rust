// FILE:      tests/common/mod.rs
// CREATED:   2026-11-05 08:34 +03
// TASK:      int-testnumber
// AUTHOR:    M Alhassoun
// WHAT:      Shared test helper: the test phone number and its format variants.
// WHY:       Tests must not hardcode a phone number. The number comes from the
//            KWTSMS_TEST_NUMBER environment variable (a Kuwait mobile in
//            international form, digits only, 965 plus 8 digits). Tests that
//            need it return early with a "skipped" note when it is unset or
//            malformed (Rust has no runtime skip).
// USAGE:     KWTSMS_TEST_NUMBER=965XXXXXXXX cargo test
//            tests/*.rs: `mod common;`  src unit tests: `crate::test_common`
//            (src/lib.rs includes this file under #[cfg(test)]).
//            CI reads the value from the repository secret of the same name.
//            Never print or log the returned value.
// DEPENDS:   std only.

#![allow(dead_code)]

/// Placeholder for values only passed through unchanged (for example a
/// webhook "mobile" field or a mocked response echo).
pub const PLACEHOLDER: &str = "965XXXXXXXX";

/// Returns KWTSMS_TEST_NUMBER when it is 965 plus 8 ASCII digits, else None
/// after printing why the calling test is skipped. Never prints the value.
pub fn test_number() -> Option<String> {
    let n = std::env::var("KWTSMS_TEST_NUMBER").unwrap_or_default();
    let n = n.trim();
    if n.len() == 11 && n.starts_with("965") && n.bytes().all(|b| b.is_ascii_digit()) {
        Some(n.to_string())
    } else {
        eprintln!("skipped: KWTSMS_TEST_NUMBER not set (digits only, 965 plus 8 digits)");
        None
    }
}

/// Country code, first 4 and last 4 local digits of a 965XXXXXXXX number.
pub fn split_kw(n: &str) -> (&str, &str, &str) {
    (&n[..3], &n[3..7], &n[7..])
}

/// The local 8-digit part (without 965).
pub fn local_part(n: &str) -> &str {
    &n[3..]
}

/// Joins the parts of the number with a separator, e.g. "965 XXXX XXXX".
pub fn with_sep(n: &str, sep: &str) -> String {
    let (cc, a, b) = split_kw(n);
    format!("{}{}{}{}{}", cc, sep, a, sep, b)
}

fn shift_digits(s: &str, zero: u32) -> String {
    s.chars()
        .map(|c| match c.to_digit(10) {
            Some(d) => char::from_u32(zero + d).unwrap_or(c),
            None => c,
        })
        .collect()
}

/// Latin digits to Arabic-Indic (U+0660-0669).
pub fn to_arabic_indic(s: &str) -> String {
    shift_digits(s, 0x0660)
}

/// Latin digits to Extended Arabic-Indic / Persian (U+06F0-06F9).
pub fn to_persian(s: &str) -> String {
    shift_digits(s, 0x06F0)
}
