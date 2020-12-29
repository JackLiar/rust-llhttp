#![allow(non_camel_case_types, clippy::unreadable_literal)]

#[repr(C)]
#[derive(Primitive)]
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

#[repr(C)]
#[derive(Primitive)]
pub enum Error {
    Ok = llhttp::llhttp_errno_HPE_OK as isize,
    Internal = llhttp::llhttp_errno_HPE_INTERNAL as isize,
    Strict = llhttp::llhttp_errno_HPE_STRICT as isize,
    LFExpected = llhttp::llhttp_errno_HPE_LF_EXPECTED as isize,
    UnexpectedContentLength = llhttp::llhttp_errno_HPE_UNEXPECTED_CONTENT_LENGTH as isize,
    ClosedConnection = llhttp::llhttp_errno_HPE_CLOSED_CONNECTION as isize,
    InvalidMethod = llhttp::llhttp_errno_HPE_INVALID_METHOD as isize,
    InvalidUrl = llhttp::llhttp_errno_HPE_INVALID_URL as isize,
    InvalidConstant = llhttp::llhttp_errno_HPE_INVALID_CONSTANT as isize,
    InvalidVersion = llhttp::llhttp_errno_HPE_INVALID_VERSION as isize,
    InvalidHeaderToken = llhttp::llhttp_errno_HPE_INVALID_HEADER_TOKEN as isize,
    InvalidContentLength = llhttp::llhttp_errno_HPE_INVALID_CONTENT_LENGTH as isize,
    InvalidChunkSize = llhttp::llhttp_errno_HPE_INVALID_CHUNK_SIZE as isize,
    InvalidStatus = llhttp::llhttp_errno_HPE_INVALID_STATUS as isize,
    InvalidEOFState = llhttp::llhttp_errno_HPE_INVALID_EOF_STATE as isize,
    InvalidTransferEncoding = llhttp::llhttp_errno_HPE_INVALID_TRANSFER_ENCODING as isize,
    CBMessageBegin = llhttp::llhttp_errno_HPE_CB_MESSAGE_BEGIN as isize,
    CBHeadersComplete = llhttp::llhttp_errno_HPE_CB_HEADERS_COMPLETE as isize,
    CBMessageComplete = llhttp::llhttp_errno_HPE_CB_MESSAGE_COMPLETE as isize,
    CBChunkHeader = llhttp::llhttp_errno_HPE_CB_CHUNK_HEADER as isize,
    CBChunkComplete = llhttp::llhttp_errno_HPE_CB_CHUNK_COMPLETE as isize,
    Paused = llhttp::llhttp_errno_HPE_PAUSED as isize,
    PausedUpgrade = llhttp::llhttp_errno_HPE_PAUSED_UPGRADE as isize,
    User = llhttp::llhttp_errno_HPE_USER as isize,
}

#[repr(C)]
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
    DESCRIBE = llhttp::llhttp_method_HTTP_DESCRIBE as isize,
    ANNOUNCE = llhttp::llhttp_method_HTTP_ANNOUNCE as isize,
    SETUP = llhttp::llhttp_method_HTTP_SETUP as isize,
    PLAY = llhttp::llhttp_method_HTTP_PLAY as isize,
    PAUSE = llhttp::llhttp_method_HTTP_PAUSE as isize,
    TEARDOWN = llhttp::llhttp_method_HTTP_TEARDOWN as isize,
    GET_PARAMETER = llhttp::llhttp_method_HTTP_GET_PARAMETER as isize,
    SET_PARAMETER = llhttp::llhttp_method_HTTP_SET_PARAMETER as isize,
    REDIRECT = llhttp::llhttp_method_HTTP_REDIRECT as isize,
    RECORD = llhttp::llhttp_method_HTTP_RECORD as isize,
    FLUSH = llhttp::llhttp_method_HTTP_FLUSH as isize,
}

#[repr(u8)]
#[derive(Primitive)]
pub enum LenientFlags {
    HEADERS = llhttp::llhttp_lenient_flags_LENIENT_HEADERS as u8,
    CHUNKED_LENGTH = llhttp::llhttp_lenient_flags_LENIENT_CHUNKED_LENGTH as u8,
}
