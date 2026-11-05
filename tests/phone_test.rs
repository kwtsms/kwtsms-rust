// 2026-11-05 08:45 +03: Kuwait tests read KWTSMS_TEST_NUMBER through tests/common and
// return early when it is unset; no phone number in the repo. int-testnumber
mod common;
use common::*;
use kwtsms::{find_country_code, normalize_phone, validate_phone_format, validate_phone_input};

// ===== normalize_phone tests =====

#[test]
fn test_normalize_plain() {
    let Some(n) = test_number() else { return };
    assert!(normalize_phone(&n) == n, "values hidden");
}

#[test]
fn test_normalize_plus() {
    let Some(n) = test_number() else { return };
    assert!(normalize_phone(&format!("+{}", n)) == n, "values hidden");
}

#[test]
fn test_normalize_double_zero() {
    let Some(n) = test_number() else { return };
    assert!(normalize_phone(&format!("00{}", n)) == n, "values hidden");
}

#[test]
fn test_normalize_spaces() {
    let Some(n) = test_number() else { return };
    assert!(normalize_phone(&with_sep(&n, " ")) == n, "values hidden");
}

#[test]
fn test_normalize_dashes() {
    let Some(n) = test_number() else { return };
    assert!(normalize_phone(&with_sep(&n, "-")) == n, "values hidden");
}

#[test]
fn test_normalize_dots() {
    let Some(n) = test_number() else { return };
    assert!(normalize_phone(&with_sep(&n, ".")) == n, "values hidden");
}

#[test]
fn test_normalize_parens() {
    let Some(n) = test_number() else { return };
    assert!(
        normalize_phone(&format!("(965) {}", local_part(&n))) == n,
        "values hidden"
    );
}

#[test]
fn test_normalize_slashes() {
    let Some(n) = test_number() else { return };
    assert!(normalize_phone(&with_sep(&n, "/")) == n, "values hidden");
}

#[test]
fn test_normalize_arabic_indic() {
    let Some(n) = test_number() else { return };
    assert!(normalize_phone(&to_arabic_indic(&n)) == n, "values hidden");
}

#[test]
fn test_normalize_persian_digits() {
    let Some(n) = test_number() else { return };
    assert!(normalize_phone(&to_persian(&n)) == n, "values hidden");
}

#[test]
fn test_normalize_mixed_formats() {
    let Some(n) = test_number() else { return };
    assert!(
        normalize_phone(&format!("+00 {}", with_sep(&n, "-"))) == n,
        "values hidden"
    );
}

#[test]
fn test_normalize_empty() {
    assert_eq!(normalize_phone(""), "");
}

#[test]
fn test_normalize_whitespace_only() {
    assert_eq!(normalize_phone("   "), "");
}

#[test]
fn test_normalize_no_digits() {
    assert_eq!(normalize_phone("abc"), "");
}

#[test]
fn test_normalize_leading_zeros() {
    // Leading zeros stripped from a non-Kuwait digit string.
    assert_eq!(normalize_phone("0012345678"), "12345678");
}

#[test]
fn test_normalize_all_zeros() {
    assert_eq!(normalize_phone("000"), "");
}

#[test]
fn test_normalize_brackets() {
    let Some(n) = test_number() else { return };
    assert!(
        normalize_phone(&format!("[965] {}", local_part(&n))) == n,
        "values hidden"
    );
}

#[test]
fn test_normalize_mixed_arabic_latin() {
    let Some(n) = test_number() else { return };
    // Every second digit Arabic-Indic, the rest Latin.
    let mixed: String = n
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 1 {
                to_arabic_indic(&c.to_string())
            } else {
                c.to_string()
            }
        })
        .collect();
    assert!(normalize_phone(&mixed) == n, "values hidden");
}

#[test]
fn test_normalize_leading_trailing_whitespace() {
    let Some(n) = test_number() else { return };
    assert!(normalize_phone(&format!("  {}  ", n)) == n, "values hidden");
}

// ===== Domestic trunk prefix stripping =====

#[test]
fn test_normalize_saudi_trunk_0559() {
    // 9660559876543 -> 966559876543
    assert_eq!(normalize_phone("9660559876543"), "966559876543");
}

#[test]
fn test_normalize_saudi_trunk_with_plus() {
    assert_eq!(normalize_phone("+9660559876543"), "966559876543");
}

#[test]
fn test_normalize_saudi_trunk_with_00() {
    assert_eq!(normalize_phone("009660559876543"), "966559876543");
}

#[test]
fn test_normalize_uae_trunk_050() {
    // 9710501234567 -> 971501234567
    assert_eq!(normalize_phone("9710501234567"), "971501234567");
}

#[test]
fn test_normalize_egypt_trunk_010() {
    // 20010123456789 -> 2010123456789
    assert_eq!(normalize_phone("20010123456789"), "2010123456789");
}

#[test]
fn test_normalize_no_trunk_no_change() {
    assert_eq!(normalize_phone("966559876543"), "966559876543");
}

#[test]
fn test_normalize_kuwait_no_trunk() {
    let Some(n) = test_number() else { return };
    assert!(normalize_phone(&n) == n, "values hidden");
}

#[test]
fn test_normalize_jordan_trunk_079() {
    // 9620791234567 -> 962791234567
    assert_eq!(normalize_phone("9620791234567"), "962791234567");
}

// ===== find_country_code tests =====

#[test]
fn test_find_cc_kuwait_3digit() {
    // Prefix lookup only: the placeholder is enough.
    assert_eq!(find_country_code(PLACEHOLDER), Some("965"));
}

#[test]
fn test_find_cc_usa_1digit() {
    assert_eq!(find_country_code("12025551234"), Some("1"));
}

#[test]
fn test_find_cc_egypt_2digit() {
    assert_eq!(find_country_code("201012345678"), Some("20"));
}

#[test]
fn test_find_cc_unknown() {
    assert_eq!(find_country_code("99912345678"), None);
}

#[test]
fn test_find_cc_empty() {
    assert_eq!(find_country_code(""), None);
}

#[test]
fn test_find_cc_saudi() {
    assert_eq!(find_country_code("966559876543"), Some("966"));
}

#[test]
fn test_find_cc_uk() {
    assert_eq!(find_country_code("447911123456"), Some("44"));
}

// ===== validate_phone_format tests =====

#[test]
fn test_format_kuwait_valid() {
    let Some(n) = test_number() else { return };
    let (valid, _) = validate_phone_format(&n);
    assert!(valid);
}

#[test]
fn test_format_kuwait_wrong_length() {
    let Some(n) = test_number() else { return };
    // 7 local digits instead of 8
    let (valid, err) = validate_phone_format(&n[..10]);
    assert!(!valid);
    assert!(err.unwrap().contains("expected 8 digits"));
}

#[test]
fn test_format_kuwait_wrong_prefix() {
    let (valid, err) = validate_phone_format("96512345678");
    assert!(!valid);
    assert!(err.unwrap().contains("must start with"));
}

#[test]
fn test_format_kuwait_all_valid_prefixes() {
    let Some(n) = test_number() else { return };
    // Built from the test number, first local digit swapped
    for prefix in ['4', '5', '6', '9'] {
        let number = format!("965{}{}", prefix, &n[4..]);
        let (valid, _) = validate_phone_format(&number);
        assert!(valid, "Kuwait prefix {} should be valid", prefix);
    }
}

#[test]
fn test_format_saudi_valid() {
    let (valid, _) = validate_phone_format("966559876543");
    assert!(valid);
}

#[test]
fn test_format_saudi_wrong_prefix() {
    let (valid, err) = validate_phone_format("966159876543");
    assert!(!valid);
    assert!(err.unwrap().contains("must start with 5"));
}

#[test]
fn test_format_uae_valid() {
    let (valid, _) = validate_phone_format("971501234567");
    assert!(valid);
}

#[test]
fn test_format_unknown_country_passes() {
    let (valid, _) = validate_phone_format("99912345678");
    assert!(valid);
}

#[test]
fn test_format_usa_valid() {
    let (valid, _) = validate_phone_format("12025551234");
    assert!(valid);
}

#[test]
fn test_format_belgium_no_prefix_check() {
    let (valid, _) = validate_phone_format("32412345678");
    assert!(valid);
}

#[test]
fn test_format_india_valid() {
    // 2026-11-05 09:02 +03: sequential sample, the old one embedded the removed Kuwait digits, int-testnumber
    let (valid, _) = validate_phone_format("919123456789");
    assert!(valid);
}

#[test]
fn test_format_india_wrong_prefix() {
    let (valid, err) = validate_phone_format("911234567890");
    assert!(!valid);
    assert!(err.unwrap().contains("India"));
}

// ===== validate_phone_input tests =====

#[test]
fn test_validate_valid_number() {
    let Some(n) = test_number() else { return };
    let (valid, err, normalized) = validate_phone_input(&format!("+{}", n));
    assert!(valid);
    assert!(err.is_none());
    assert!(normalized == n, "values hidden");
}

#[test]
fn test_validate_empty() {
    let (valid, err, _) = validate_phone_input("");
    assert!(!valid);
    assert_eq!(err.unwrap(), "Phone number is required");
}

#[test]
fn test_validate_blank() {
    let (valid, err, _) = validate_phone_input("   ");
    assert!(!valid);
    assert_eq!(err.unwrap(), "Phone number is required");
}

#[test]
fn test_validate_email() {
    let (valid, err, _) = validate_phone_input("user@gmail.com");
    assert!(!valid);
    let err_msg = err.unwrap();
    assert!(err_msg.contains("email address"));
    assert!(err_msg.contains("user@gmail.com"));
}

#[test]
fn test_validate_no_digits() {
    let (valid, err, _) = validate_phone_input("abc");
    assert!(!valid);
    assert!(err.unwrap().contains("no digits found"));
}

#[test]
fn test_validate_too_short() {
    let (valid, err, normalized) = validate_phone_input("123456");
    assert!(!valid);
    let err_msg = err.unwrap();
    assert!(err_msg.contains("too short"));
    assert!(err_msg.contains("6 digits"));
    assert_eq!(normalized, "123456");
}

#[test]
fn test_validate_too_long() {
    let (valid, err, _) = validate_phone_input("1234567890123456");
    assert!(!valid);
    let err_msg = err.unwrap();
    assert!(err_msg.contains("too long"));
    assert!(err_msg.contains("16 digits"));
}

#[test]
fn test_validate_minimum_valid_7_digits() {
    // Use 999xxxx (no country rule) for generic E.164 test
    let (valid, err, normalized) = validate_phone_input("9991234");
    assert!(valid);
    assert!(err.is_none());
    assert_eq!(normalized, "9991234");
}

#[test]
fn test_validate_maximum_valid_15_digits() {
    // Use 999... (no country rule) for generic E.164 test
    let (valid, err, normalized) = validate_phone_input("999456789012345");
    assert!(valid);
    assert!(err.is_none());
    assert_eq!(normalized, "999456789012345");
}

#[test]
fn test_validate_exactly_16_digits() {
    let (valid, err, _) = validate_phone_input("1234567890123456");
    assert!(!valid);
    assert!(err.unwrap().contains("too long"));
}

#[test]
fn test_validate_arabic_digits() {
    let Some(n) = test_number() else { return };
    let (valid, err, normalized) = validate_phone_input(&to_arabic_indic(&n));
    assert!(valid);
    assert!(err.is_none());
    assert!(normalized == n, "values hidden");
}

#[test]
fn test_validate_double_zero_prefix() {
    let Some(n) = test_number() else { return };
    let (valid, err, normalized) = validate_phone_input(&format!("00{}", n));
    assert!(valid);
    assert!(err.is_none());
    assert!(normalized == n, "values hidden");
}

#[test]
fn test_validate_special_chars() {
    let (valid, err, _) = validate_phone_input("!@#$%^&*");
    assert!(!valid);
    assert!(err.unwrap().contains("email address"));
}

#[test]
fn test_validate_only_special_no_at() {
    let (valid, err, _) = validate_phone_input("!#$%^&*");
    assert!(!valid);
    assert!(err.unwrap().contains("no digits found"));
}

// ===== Country format validation via validate_phone_input =====

#[test]
fn test_validate_saudi_trunk_stripped_and_valid() {
    let (valid, err, normalized) = validate_phone_input("+9660559876543");
    assert!(valid);
    assert!(err.is_none());
    assert_eq!(normalized, "966559876543");
}

#[test]
fn test_validate_kuwait_wrong_prefix_rejected() {
    let (valid, err, normalized) = validate_phone_input("96512345678");
    assert!(!valid);
    assert!(err.unwrap().contains("Invalid Kuwait mobile number"));
    assert_eq!(normalized, "96512345678");
}

#[test]
fn test_validate_kuwait_wrong_length_rejected() {
    let Some(n) = test_number() else { return };
    // 7 local digits
    let (valid, err, _) = validate_phone_input(&n[..10]);
    assert!(!valid);
    assert!(err.unwrap().contains("expected 8 digits"));
}

#[test]
fn test_validate_saudi_wrong_prefix_rejected() {
    let (valid, err, _) = validate_phone_input("966159876543");
    assert!(!valid);
    assert!(err.unwrap().contains("Saudi Arabia"));
}

#[test]
fn test_validate_unknown_country_passes_generic() {
    let (valid, err, _) = validate_phone_input("99912345678");
    assert!(valid);
    assert!(err.is_none());
}

#[test]
fn test_validate_uae_trunk_stripped_valid() {
    // 971 + 0 + 501234567 -> trunk stripped -> 971501234567 (9 local digits, starts with 5)
    let (valid, err, normalized) = validate_phone_input("9710501234567");
    assert!(valid, "UAE trunk should be stripped: {:?}", err);
    assert_eq!(normalized, "971501234567");
}
