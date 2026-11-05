//! # kwtsms
//!
//! Rust client for the kwtSMS API (kwtsms.com).
//! Send SMS, check balance, validate numbers, list sender IDs, check coverage,
//! get delivery reports, and more.
//!
//! ## Quick Start
//!
//! ```no_run
//! use kwtsms::KwtSms;
//!
//! // Create client from .env file or environment variables
//! let sms = KwtSms::from_env(None).unwrap();
//!
//! // Verify credentials
//! let result = sms.verify();
//! println!("Balance: {:?}", result.balance);
//!
//! // Send SMS
//! let response = sms.send_one("965XXXXXXXX", "Hello from Rust!", None).unwrap();
//! println!("{}", response);
//! ```

mod client;
pub(crate) mod env;
pub mod errors;
pub mod logger;
pub mod message;
pub mod phone;
mod request;
pub mod types;

// 2026-11-05 08:35 +03: shared test number helper (KWTSMS_TEST_NUMBER) for the inline
// unit tests; the same file serves tests/*.rs as `mod common;`. Doc example above uses
// the placeholder 965XXXXXXXX (no_run, never executed). int-testnumber
#[cfg(test)]
#[path = "../tests/common/mod.rs"]
mod test_common;

// Re-export public API
// 2026-11-05 14:13 +03: build_user_agent and VERSION exported for the User-Agent (#939).
pub use client::{build_user_agent, KwtSms, VERSION};
pub use errors::{enrich_error, KwtSmsError, API_ERRORS};
pub use message::clean_message;
pub use phone::{
    find_country_code, normalize_phone, validate_phone_format, validate_phone_input, PhoneRule,
    PHONE_RULES,
};
pub use types::*;
