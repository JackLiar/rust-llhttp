#![allow(non_camel_case_types, clippy::unreadable_literal)]

use crate::ffi;

pub type Type = ffi::llhttp_type;
pub type Error = ffi::llhttp_errno;
pub type Method = ffi::llhttp_method;
pub type LenientFlags = ffi::llhttp_lenient_flags;
