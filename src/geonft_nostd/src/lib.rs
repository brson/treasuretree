#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::String;

pub mod crypto;

pub fn abbrev_pubkey(key: &str) -> String {
    let public_key_abbrev: String = key.chars().take(14).collect();
    let public_key_abbrev = format!("{}…", public_key_abbrev);
    public_key_abbrev
}
