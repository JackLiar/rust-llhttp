#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

extern crate llhttp_sys as llhttp;

use std::os::raw::c_int;

use num_traits::FromPrimitive;

use llhttp::{llhttp_cb, llhttp_data_cb};
pub use llhttp::{llhttp_t, size_t};

pub type CallBack = llhttp_cb;
pub type DataCallBack = llhttp_data_cb;

#[derive(Copy, Clone, Debug)]
pub struct Settings {
    settings: llhttp::llhttp_settings_s,
}

#[repr(C)]
pub enum Type {
    BOTH = llhttp::llhttp_type_HTTP_BOTH as isize,
    REQUEST = llhttp::llhttp_type_HTTP_REQUEST as isize,
    RESPONSE = llhttp::llhttp_type_HTTP_RESPONSE as isize,
}

impl Into<llhttp::llhttp_type_t> for Type {
    #[inline(always)]
    fn into(self) -> llhttp::llhttp_type_t {
        self as llhttp::llhttp_type_t
    }
}

#[derive(Primitive)]
#[repr(C)]
pub enum Error {
    OK = llhttp::llhttp_errno_HPE_OK as isize,
    INTERNAL = llhttp::llhttp_errno_HPE_INTERNAL as isize,
    STRICT = llhttp::llhttp_errno_HPE_STRICT as isize,
    LF_EXPECTED = llhttp::llhttp_errno_HPE_LF_EXPECTED as isize,
    UNEXPECTED_CONTENT_LENGTH = llhttp::llhttp_errno_HPE_UNEXPECTED_CONTENT_LENGTH as isize,
    CLOSED_CONNECTION = llhttp::llhttp_errno_HPE_CLOSED_CONNECTION as isize,
    INVALID_METHOD = llhttp::llhttp_errno_HPE_INVALID_METHOD as isize,
    INVALID_URL = llhttp::llhttp_errno_HPE_INVALID_URL as isize,
    INVALID_CONSTANT = llhttp::llhttp_errno_HPE_INVALID_CONSTANT as isize,
    INVALID_VERSION = llhttp::llhttp_errno_HPE_INVALID_VERSION as isize,
    INVALID_HEADER_TOKEN = llhttp::llhttp_errno_HPE_INVALID_HEADER_TOKEN as isize,
    INVALID_CONTENT_LENGTH = llhttp::llhttp_errno_HPE_INVALID_CONTENT_LENGTH as isize,
    INVALID_CHUNK_SIZE = llhttp::llhttp_errno_HPE_INVALID_CHUNK_SIZE as isize,
    INVALID_STATUS = llhttp::llhttp_errno_HPE_INVALID_STATUS as isize,
    INVALID_EOF_STATE = llhttp::llhttp_errno_HPE_INVALID_EOF_STATE as isize,
    INVALID_TRANSFER_ENCODING = llhttp::llhttp_errno_HPE_INVALID_TRANSFER_ENCODING as isize,
    CB_MESSAGE_BEGIN = llhttp::llhttp_errno_HPE_CB_MESSAGE_BEGIN as isize,
    CB_HEADERS_COMPLETE = llhttp::llhttp_errno_HPE_CB_HEADERS_COMPLETE as isize,
    CB_MESSAGE_COMPLETE = llhttp::llhttp_errno_HPE_CB_MESSAGE_COMPLETE as isize,
    CB_CHUNK_HEADER = llhttp::llhttp_errno_HPE_CB_CHUNK_HEADER as isize,
    CB_CHUNK_COMPLETE = llhttp::llhttp_errno_HPE_CB_CHUNK_COMPLETE as isize,
    PAUSED = llhttp::llhttp_errno_HPE_PAUSED as isize,
    PAUSED_UPGRADE = llhttp::llhttp_errno_HPE_PAUSED_UPGRADE as isize,
    USER = llhttp::llhttp_errno_HPE_USER as isize,
}

#[derive(Debug, Primitive)]
pub enum Method {
    DELETE = llhttp::llhttp_method_HTTP_DELETE as isize,
    GET = llhttp::llhttp_method_HTTP_GET as isize,
    HEAD = llhttp::llhttp_method_HTTP_HEAD as isize,
    POST = llhttp::llhttp_method_HTTP_POST as isize,
    PUT = llhttp::llhttp_method_HTTP_PUT as isize,
    CONNECT = llhttp::llhttp_method_HTTP_CONNECT as isize,
    OPTIONS = llhttp::llhttp_method_HTTP_OPTIONS as isize,
    TRACE = llhttp::llhttp_method_HTTP_TRACE as isize,
    COPY = llhttp::llhttp_method_HTTP_COPY as isize,
    LOCK = llhttp::llhttp_method_HTTP_LOCK as isize,
    MKCOL = llhttp::llhttp_method_HTTP_MKCOL as isize,
    MOVE = llhttp::llhttp_method_HTTP_MOVE as isize,
    PROPFIND = llhttp::llhttp_method_HTTP_PROPFIND as isize,
    PROPPATCH = llhttp::llhttp_method_HTTP_PROPPATCH as isize,
    SEARCH = llhttp::llhttp_method_HTTP_SEARCH as isize,
    UNLOCK = llhttp::llhttp_method_HTTP_UNLOCK as isize,
    BIND = llhttp::llhttp_method_HTTP_BIND as isize,
    REBIND = llhttp::llhttp_method_HTTP_REBIND as isize,
    UNBIND = llhttp::llhttp_method_HTTP_UNBIND as isize,
    ACL = llhttp::llhttp_method_HTTP_ACL as isize,
    REPORT = llhttp::llhttp_method_HTTP_REPORT as isize,
    MKACTIVITY = llhttp::llhttp_method_HTTP_MKACTIVITY as isize,
    CHECKOUT = llhttp::llhttp_method_HTTP_CHECKOUT as isize,
    MERGE = llhttp::llhttp_method_HTTP_MERGE as isize,
    MSEARCH = llhttp::llhttp_method_HTTP_MSEARCH as isize,
    NOTIFY = llhttp::llhttp_method_HTTP_NOTIFY as isize,
    SUBSCRIBE = llhttp::llhttp_method_HTTP_SUBSCRIBE as isize,
    UNSUBSCRIBE = llhttp::llhttp_method_HTTP_UNSUBSCRIBE as isize,
    PATCH = llhttp::llhttp_method_HTTP_PATCH as isize,
    PURGE = llhttp::llhttp_method_HTTP_PURGE as isize,
    MKCALENDAR = llhttp::llhttp_method_HTTP_MKCALENDAR as isize,
    LINK = llhttp::llhttp_method_HTTP_LINK as isize,
    UNLINK = llhttp::llhttp_method_HTTP_UNLINK as isize,
    SOURCE = llhttp::llhttp_method_HTTP_SOURCE as isize,
    PRI = llhttp::llhttp_method_HTTP_PRI as isize,
}

impl Settings {
    pub fn new() -> Settings {
        let mut settings = llhttp::llhttp_settings_s {
            on_message_begin: None,
            on_url: None,
            on_status: None,
            on_header_field: None,
            on_header_value: None,
            on_headers_complete: None,
            on_body: None,
            on_message_complete: None,
            on_chunk_header: None,
            on_chunk_complete: None,
        };
        unsafe {
            llhttp::llhttp_settings_init(&mut settings);
        }
        Settings { settings }
    }

    #[inline]
    pub fn get_inner(&self) -> &llhttp::llhttp_settings_s {
        &self.settings
    }

    #[inline]
    pub fn get_inner_mut(&mut self) -> &llhttp::llhttp_settings_s {
        &mut self.settings
    }
}

/// llhttp parser
#[derive(Clone)]
pub struct Parser {
    parser: llhttp::llhttp_t,
    settings: Settings,
}

impl Parser {
    /// Create a new llhttp parser
    pub fn new(settings: Settings, lltype: Type) -> Parser {
        let mut parser = llhttp::llhttp_t {
            _index: 0,
            _span_pos0: std::ptr::null_mut(),
            _span_cb0: std::ptr::null_mut(),
            error: 0,
            reason: std::ptr::null(),
            error_pos: std::ptr::null(),
            data: std::ptr::null_mut(),
            _current: std::ptr::null_mut(),
            content_length: 0,
            type_: 0,
            method: 0,
            http_major: 0,
            http_minor: 0,
            header_state: 0,
            flags: 0,
            upgrade: 0,
            status_code: 0,
            finish: 0,
            settings: std::ptr::null_mut(),
        };
        unsafe {
            llhttp::llhttp_init(&mut parser, lltype.into(), settings.get_inner());
        }
        Parser { parser, settings }
    }

    #[inline]
    pub fn parse(&mut self, data: &[u8]) -> Error {
        let err;
        unsafe {
            err = llhttp::llhttp_execute(
                &mut self.parser,
                data.as_ptr() as *const i8,
                data.len() as llhttp::size_t,
            );
        }
        match Error::from_u32(err) {
            Some(i) => i,
            None => unreachable!(),
        }
    }

    #[inline]
    pub fn finish(&mut self) -> Error {
        let err;
        unsafe { err = llhttp::llhttp_finish(&mut self.parser) }
        match Error::from_u32(err) {
            Some(i) => i,
            None => unreachable!(),
        }
    }

    #[inline]
    pub fn message_needs_eof(&self) -> c_int {
        unsafe { llhttp::llhttp_message_needs_eof(&self.parser) }
    }

    #[inline]
    pub fn should_keep_alive(&self) -> c_int {
        unsafe { llhttp::llhttp_should_keep_alive(&self.parser) }
    }

    #[inline]
    pub fn pause(&mut self) {
        unsafe { llhttp::llhttp_pause(&mut self.parser) }
    }

    #[inline]
    pub fn resume(&mut self) {
        unsafe { llhttp::llhttp_resume(&mut self.parser) }
    }

    #[inline]
    pub fn resume_after_upgrade(&mut self) {
        unsafe { llhttp::llhttp_resume_after_upgrade(&mut self.parser) }
    }

    #[inline]
    pub fn status_code(&self) -> u16 {
        self.parser.status_code
    }
    #[inline]
    pub fn method(&self) -> Method {
        match Method::from_u8(self.parser.method) {
            Some(m) => m,
            None => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(String::from("ACL"), format!("{:?}", Method::ACL));
        // assert_eq!(2 + 2, 4);
    }
}
