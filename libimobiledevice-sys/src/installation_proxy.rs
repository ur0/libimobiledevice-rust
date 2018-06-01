//! Bindings to `installation_proxy.h`.

use idevice::idevice_t;
use libplist_sys::plist_t;
use lockdown::lockdownd_service_descriptor_t;

use std::os::raw::{c_char, c_void};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum instproxy_error_t {
    Success = 0,
    InvalidArgument = -1,
}

#[doc(hidden)]
#[repr(C)]
pub struct instproxy_client_private(c_void);
pub type instproxy_client_t = *mut instproxy_client_private;

extern "C" {
    pub fn instproxy_client_new(
        device: idevice_t,
        service: lockdownd_service_descriptor_t,
        client: *mut instproxy_client_t,
    ) -> instproxy_error_t;
    pub fn instproxy_client_free(client: instproxy_client_t) -> instproxy_error_t;
    pub fn instproxy_client_options_new() -> plist_t;
    pub fn instproxy_client_options_add(options: plist_t, ...);
    pub fn instproxy_install(
        client: instproxy_client_t,
        path: *const c_char,
        client_opts: plist_t,
        callback: *const c_void,
        callback_data: *const c_void,
    ) -> instproxy_error_t;
    pub fn instproxy_client_options_free(opts: plist_t);
}
