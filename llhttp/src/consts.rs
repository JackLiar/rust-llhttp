#![allow(non_camel_case_types, clippy::unreadable_literal)]

use crate::ffi;

#[repr(u8)]
#[derive(Debug, Primitive)]
pub enum Type {
    BOTH = ffi::llhttp_type_HTTP_BOTH as u8,
    REQUEST = ffi::llhttp_type_HTTP_REQUEST as u8,
    RESPONSE = ffi::llhttp_type_HTTP_RESPONSE as u8,
}

impl Into<ffi::llhttp_type_t> for Type {
    #[inline(always)]
    fn into(self) -> ffi::llhttp_type_t {
        self as ffi::llhttp_type_t
    }
}

#[repr(C)]
#[derive(Debug, Primitive)]
pub enum Error {
    Ok = ffi::llhttp_errno_HPE_OK as isize,
    Internal = ffi::llhttp_errno_HPE_INTERNAL as isize,
    Strict = ffi::llhttp_errno_HPE_STRICT as isize,
    LFExpected = ffi::llhttp_errno_HPE_LF_EXPECTED as isize,
    UnexpectedContentLength = ffi::llhttp_errno_HPE_UNEXPECTED_CONTENT_LENGTH as isize,
    ClosedConnection = ffi::llhttp_errno_HPE_CLOSED_CONNECTION as isize,
    InvalidMethod = ffi::llhttp_errno_HPE_INVALID_METHOD as isize,
    InvalidUrl = ffi::llhttp_errno_HPE_INVALID_URL as isize,
    InvalidConstant = ffi::llhttp_errno_HPE_INVALID_CONSTANT as isize,
    InvalidVersion = ffi::llhttp_errno_HPE_INVALID_VERSION as isize,
    InvalidHeaderToken = ffi::llhttp_errno_HPE_INVALID_HEADER_TOKEN as isize,
    InvalidContentLength = ffi::llhttp_errno_HPE_INVALID_CONTENT_LENGTH as isize,
    InvalidChunkSize = ffi::llhttp_errno_HPE_INVALID_CHUNK_SIZE as isize,
    InvalidStatus = ffi::llhttp_errno_HPE_INVALID_STATUS as isize,
    InvalidEOFState = ffi::llhttp_errno_HPE_INVALID_EOF_STATE as isize,
    InvalidTransferEncoding = ffi::llhttp_errno_HPE_INVALID_TRANSFER_ENCODING as isize,
    CBMessageBegin = ffi::llhttp_errno_HPE_CB_MESSAGE_BEGIN as isize,
    CBHeadersComplete = ffi::llhttp_errno_HPE_CB_HEADERS_COMPLETE as isize,
    CBMessageComplete = ffi::llhttp_errno_HPE_CB_MESSAGE_COMPLETE as isize,
    CBChunkHeader = ffi::llhttp_errno_HPE_CB_CHUNK_HEADER as isize,
    CBChunkComplete = ffi::llhttp_errno_HPE_CB_CHUNK_COMPLETE as isize,
    Paused = ffi::llhttp_errno_HPE_PAUSED as isize,
    PausedUpgrade = ffi::llhttp_errno_HPE_PAUSED_UPGRADE as isize,
    User = ffi::llhttp_errno_HPE_USER as isize,
}

#[repr(C)]
#[derive(Debug, Primitive)]
pub enum Method {
    DELETE = ffi::llhttp_method_HTTP_DELETE as isize,
    GET = ffi::llhttp_method_HTTP_GET as isize,
    HEAD = ffi::llhttp_method_HTTP_HEAD as isize,
    POST = ffi::llhttp_method_HTTP_POST as isize,
    PUT = ffi::llhttp_method_HTTP_PUT as isize,
    CONNECT = ffi::llhttp_method_HTTP_CONNECT as isize,
    OPTIONS = ffi::llhttp_method_HTTP_OPTIONS as isize,
    TRACE = ffi::llhttp_method_HTTP_TRACE as isize,
    COPY = ffi::llhttp_method_HTTP_COPY as isize,
    LOCK = ffi::llhttp_method_HTTP_LOCK as isize,
    MKCOL = ffi::llhttp_method_HTTP_MKCOL as isize,
    MOVE = ffi::llhttp_method_HTTP_MOVE as isize,
    PROPFIND = ffi::llhttp_method_HTTP_PROPFIND as isize,
    PROPPATCH = ffi::llhttp_method_HTTP_PROPPATCH as isize,
    SEARCH = ffi::llhttp_method_HTTP_SEARCH as isize,
    UNLOCK = ffi::llhttp_method_HTTP_UNLOCK as isize,
    BIND = ffi::llhttp_method_HTTP_BIND as isize,
    REBIND = ffi::llhttp_method_HTTP_REBIND as isize,
    UNBIND = ffi::llhttp_method_HTTP_UNBIND as isize,
    ACL = ffi::llhttp_method_HTTP_ACL as isize,
    REPORT = ffi::llhttp_method_HTTP_REPORT as isize,
    MKACTIVITY = ffi::llhttp_method_HTTP_MKACTIVITY as isize,
    CHECKOUT = ffi::llhttp_method_HTTP_CHECKOUT as isize,
    MERGE = ffi::llhttp_method_HTTP_MERGE as isize,
    MSEARCH = ffi::llhttp_method_HTTP_MSEARCH as isize,
    NOTIFY = ffi::llhttp_method_HTTP_NOTIFY as isize,
    SUBSCRIBE = ffi::llhttp_method_HTTP_SUBSCRIBE as isize,
    UNSUBSCRIBE = ffi::llhttp_method_HTTP_UNSUBSCRIBE as isize,
    PATCH = ffi::llhttp_method_HTTP_PATCH as isize,
    PURGE = ffi::llhttp_method_HTTP_PURGE as isize,
    MKCALENDAR = ffi::llhttp_method_HTTP_MKCALENDAR as isize,
    LINK = ffi::llhttp_method_HTTP_LINK as isize,
    UNLINK = ffi::llhttp_method_HTTP_UNLINK as isize,
    SOURCE = ffi::llhttp_method_HTTP_SOURCE as isize,
    PRI = ffi::llhttp_method_HTTP_PRI as isize,
    DESCRIBE = ffi::llhttp_method_HTTP_DESCRIBE as isize,
    ANNOUNCE = ffi::llhttp_method_HTTP_ANNOUNCE as isize,
    SETUP = ffi::llhttp_method_HTTP_SETUP as isize,
    PLAY = ffi::llhttp_method_HTTP_PLAY as isize,
    PAUSE = ffi::llhttp_method_HTTP_PAUSE as isize,
    TEARDOWN = ffi::llhttp_method_HTTP_TEARDOWN as isize,
    GET_PARAMETER = ffi::llhttp_method_HTTP_GET_PARAMETER as isize,
    SET_PARAMETER = ffi::llhttp_method_HTTP_SET_PARAMETER as isize,
    REDIRECT = ffi::llhttp_method_HTTP_REDIRECT as isize,
    RECORD = ffi::llhttp_method_HTTP_RECORD as isize,
    FLUSH = ffi::llhttp_method_HTTP_FLUSH as isize,
}

#[repr(u8)]
#[derive(Primitive)]
pub enum LenientFlags {
    HEADERS = ffi::llhttp_lenient_flags_LENIENT_HEADERS as u8,
    CHUNKED_LENGTH = ffi::llhttp_lenient_flags_LENIENT_CHUNKED_LENGTH as u8,
}
