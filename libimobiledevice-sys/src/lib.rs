#![allow(non_camel_case_types)]

extern crate libplist_sys;

pub mod afc;
pub mod diagnostics_relay;
pub mod idevice;
pub mod installation_proxy;
pub mod lockdown;

pub use idevice::*;

#[test]
fn test_validity() {
    unsafe {
        // Just to check if libimobiledevice is linked.
        idevice_set_debug_level(0);
    }
}
