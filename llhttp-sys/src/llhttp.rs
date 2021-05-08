/* automatically generated by rust-bindgen 0.58.1 */

pub type llhttp__internal_t = llhttp__internal_s;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct llhttp__internal_s {
    pub _index: i32,
    pub _span_pos0: *mut ::libc::c_void,
    pub _span_cb0: *mut ::libc::c_void,
    pub error: i32,
    pub reason: *const ::libc::c_char,
    pub error_pos: *const ::libc::c_char,
    pub data: *mut ::libc::c_void,
    pub _current: *mut ::libc::c_void,
    pub content_length: u64,
    pub type_: u8,
    pub method: u8,
    pub http_major: u8,
    pub http_minor: u8,
    pub header_state: u8,
    pub lenient_flags: u8,
    pub upgrade: u8,
    pub finish: u8,
    pub flags: u16,
    pub status_code: u16,
    pub settings: *mut ::libc::c_void,
}
#[test]
fn bindgen_test_layout_llhttp__internal_s() {
    assert_eq!(
        ::core::mem::size_of::<llhttp__internal_s>(),
        96usize,
        concat!("Size of: ", stringify!(llhttp__internal_s))
    );
    assert_eq!(
        ::core::mem::align_of::<llhttp__internal_s>(),
        8usize,
        concat!("Alignment of ", stringify!(llhttp__internal_s))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>()))._index as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(_index)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>()))._span_pos0 as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(_span_pos0)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>()))._span_cb0 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(_span_cb0)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>())).error as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>())).reason as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(reason)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>())).error_pos as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(error_pos)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>())).data as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>()))._current as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(_current)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>())).content_length as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(content_length)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>())).type_ as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>())).method as *const _ as usize },
        73usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(method)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>())).http_major as *const _ as usize },
        74usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(http_major)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>())).http_minor as *const _ as usize },
        75usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(http_minor)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>())).header_state as *const _ as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(header_state)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>())).lenient_flags as *const _ as usize },
        77usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(lenient_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>())).upgrade as *const _ as usize },
        78usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(upgrade)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>())).finish as *const _ as usize },
        79usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(finish)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>())).flags as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>())).status_code as *const _ as usize },
        82usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(status_code)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp__internal_s>())).settings as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp__internal_s),
            "::",
            stringify!(settings)
        )
    );
}
impl Default for llhttp__internal_s {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
extern "C" {
    pub fn llhttp__internal_init(s: *mut llhttp__internal_t) -> ::libc::c_int;
}
extern "C" {
    pub fn llhttp__internal_execute(
        s: *mut llhttp__internal_t,
        p: *const ::libc::c_char,
        endp: *const ::libc::c_char,
    ) -> ::libc::c_int;
}
impl llhttp_errno {
    pub const HPE_OK: llhttp_errno = llhttp_errno(0);
}
impl llhttp_errno {
    pub const HPE_INTERNAL: llhttp_errno = llhttp_errno(1);
}
impl llhttp_errno {
    pub const HPE_STRICT: llhttp_errno = llhttp_errno(2);
}
impl llhttp_errno {
    pub const HPE_LF_EXPECTED: llhttp_errno = llhttp_errno(3);
}
impl llhttp_errno {
    pub const HPE_UNEXPECTED_CONTENT_LENGTH: llhttp_errno = llhttp_errno(4);
}
impl llhttp_errno {
    pub const HPE_CLOSED_CONNECTION: llhttp_errno = llhttp_errno(5);
}
impl llhttp_errno {
    pub const HPE_INVALID_METHOD: llhttp_errno = llhttp_errno(6);
}
impl llhttp_errno {
    pub const HPE_INVALID_URL: llhttp_errno = llhttp_errno(7);
}
impl llhttp_errno {
    pub const HPE_INVALID_CONSTANT: llhttp_errno = llhttp_errno(8);
}
impl llhttp_errno {
    pub const HPE_INVALID_VERSION: llhttp_errno = llhttp_errno(9);
}
impl llhttp_errno {
    pub const HPE_INVALID_HEADER_TOKEN: llhttp_errno = llhttp_errno(10);
}
impl llhttp_errno {
    pub const HPE_INVALID_CONTENT_LENGTH: llhttp_errno = llhttp_errno(11);
}
impl llhttp_errno {
    pub const HPE_INVALID_CHUNK_SIZE: llhttp_errno = llhttp_errno(12);
}
impl llhttp_errno {
    pub const HPE_INVALID_STATUS: llhttp_errno = llhttp_errno(13);
}
impl llhttp_errno {
    pub const HPE_INVALID_EOF_STATE: llhttp_errno = llhttp_errno(14);
}
impl llhttp_errno {
    pub const HPE_INVALID_TRANSFER_ENCODING: llhttp_errno = llhttp_errno(15);
}
impl llhttp_errno {
    pub const HPE_CB_MESSAGE_BEGIN: llhttp_errno = llhttp_errno(16);
}
impl llhttp_errno {
    pub const HPE_CB_HEADERS_COMPLETE: llhttp_errno = llhttp_errno(17);
}
impl llhttp_errno {
    pub const HPE_CB_MESSAGE_COMPLETE: llhttp_errno = llhttp_errno(18);
}
impl llhttp_errno {
    pub const HPE_CB_CHUNK_HEADER: llhttp_errno = llhttp_errno(19);
}
impl llhttp_errno {
    pub const HPE_CB_CHUNK_COMPLETE: llhttp_errno = llhttp_errno(20);
}
impl llhttp_errno {
    pub const HPE_PAUSED: llhttp_errno = llhttp_errno(21);
}
impl llhttp_errno {
    pub const HPE_PAUSED_UPGRADE: llhttp_errno = llhttp_errno(22);
}
impl llhttp_errno {
    pub const HPE_PAUSED_H2_UPGRADE: llhttp_errno = llhttp_errno(23);
}
impl llhttp_errno {
    pub const HPE_USER: llhttp_errno = llhttp_errno(24);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct llhttp_errno(pub ::libc::c_uint);
pub use self::llhttp_errno as llhttp_errno_t;
impl llhttp_flags {
    pub const F_CONNECTION_KEEP_ALIVE: llhttp_flags = llhttp_flags(1);
}
impl llhttp_flags {
    pub const F_CONNECTION_CLOSE: llhttp_flags = llhttp_flags(2);
}
impl llhttp_flags {
    pub const F_CONNECTION_UPGRADE: llhttp_flags = llhttp_flags(4);
}
impl llhttp_flags {
    pub const F_CHUNKED: llhttp_flags = llhttp_flags(8);
}
impl llhttp_flags {
    pub const F_UPGRADE: llhttp_flags = llhttp_flags(16);
}
impl llhttp_flags {
    pub const F_CONTENT_LENGTH: llhttp_flags = llhttp_flags(32);
}
impl llhttp_flags {
    pub const F_SKIPBODY: llhttp_flags = llhttp_flags(64);
}
impl llhttp_flags {
    pub const F_TRAILING: llhttp_flags = llhttp_flags(128);
}
impl llhttp_flags {
    pub const F_TRANSFER_ENCODING: llhttp_flags = llhttp_flags(512);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct llhttp_flags(pub ::libc::c_uint);
pub use self::llhttp_flags as llhttp_flags_t;
impl llhttp_lenient_flags {
    pub const LENIENT_HEADERS: llhttp_lenient_flags = llhttp_lenient_flags(1);
}
impl llhttp_lenient_flags {
    pub const LENIENT_CHUNKED_LENGTH: llhttp_lenient_flags = llhttp_lenient_flags(2);
}
impl llhttp_lenient_flags {
    pub const LENIENT_KEEP_ALIVE: llhttp_lenient_flags = llhttp_lenient_flags(4);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct llhttp_lenient_flags(pub ::libc::c_uint);
pub use self::llhttp_lenient_flags as llhttp_lenient_flags_t;
impl llhttp_type {
    pub const HTTP_BOTH: llhttp_type = llhttp_type(0);
}
impl llhttp_type {
    pub const HTTP_REQUEST: llhttp_type = llhttp_type(1);
}
impl llhttp_type {
    pub const HTTP_RESPONSE: llhttp_type = llhttp_type(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct llhttp_type(pub ::libc::c_uint);
pub use self::llhttp_type as llhttp_type_t;
pub const llhttp_finish_HTTP_FINISH_SAFE: llhttp_finish = 0;
pub const llhttp_finish_HTTP_FINISH_SAFE_WITH_CB: llhttp_finish = 1;
pub const llhttp_finish_HTTP_FINISH_UNSAFE: llhttp_finish = 2;
pub type llhttp_finish = ::libc::c_uint;
pub use self::llhttp_finish as llhttp_finish_t;
impl llhttp_method {
    pub const HTTP_DELETE: llhttp_method = llhttp_method(0);
}
impl llhttp_method {
    pub const HTTP_GET: llhttp_method = llhttp_method(1);
}
impl llhttp_method {
    pub const HTTP_HEAD: llhttp_method = llhttp_method(2);
}
impl llhttp_method {
    pub const HTTP_POST: llhttp_method = llhttp_method(3);
}
impl llhttp_method {
    pub const HTTP_PUT: llhttp_method = llhttp_method(4);
}
impl llhttp_method {
    pub const HTTP_CONNECT: llhttp_method = llhttp_method(5);
}
impl llhttp_method {
    pub const HTTP_OPTIONS: llhttp_method = llhttp_method(6);
}
impl llhttp_method {
    pub const HTTP_TRACE: llhttp_method = llhttp_method(7);
}
impl llhttp_method {
    pub const HTTP_COPY: llhttp_method = llhttp_method(8);
}
impl llhttp_method {
    pub const HTTP_LOCK: llhttp_method = llhttp_method(9);
}
impl llhttp_method {
    pub const HTTP_MKCOL: llhttp_method = llhttp_method(10);
}
impl llhttp_method {
    pub const HTTP_MOVE: llhttp_method = llhttp_method(11);
}
impl llhttp_method {
    pub const HTTP_PROPFIND: llhttp_method = llhttp_method(12);
}
impl llhttp_method {
    pub const HTTP_PROPPATCH: llhttp_method = llhttp_method(13);
}
impl llhttp_method {
    pub const HTTP_SEARCH: llhttp_method = llhttp_method(14);
}
impl llhttp_method {
    pub const HTTP_UNLOCK: llhttp_method = llhttp_method(15);
}
impl llhttp_method {
    pub const HTTP_BIND: llhttp_method = llhttp_method(16);
}
impl llhttp_method {
    pub const HTTP_REBIND: llhttp_method = llhttp_method(17);
}
impl llhttp_method {
    pub const HTTP_UNBIND: llhttp_method = llhttp_method(18);
}
impl llhttp_method {
    pub const HTTP_ACL: llhttp_method = llhttp_method(19);
}
impl llhttp_method {
    pub const HTTP_REPORT: llhttp_method = llhttp_method(20);
}
impl llhttp_method {
    pub const HTTP_MKACTIVITY: llhttp_method = llhttp_method(21);
}
impl llhttp_method {
    pub const HTTP_CHECKOUT: llhttp_method = llhttp_method(22);
}
impl llhttp_method {
    pub const HTTP_MERGE: llhttp_method = llhttp_method(23);
}
impl llhttp_method {
    pub const HTTP_MSEARCH: llhttp_method = llhttp_method(24);
}
impl llhttp_method {
    pub const HTTP_NOTIFY: llhttp_method = llhttp_method(25);
}
impl llhttp_method {
    pub const HTTP_SUBSCRIBE: llhttp_method = llhttp_method(26);
}
impl llhttp_method {
    pub const HTTP_UNSUBSCRIBE: llhttp_method = llhttp_method(27);
}
impl llhttp_method {
    pub const HTTP_PATCH: llhttp_method = llhttp_method(28);
}
impl llhttp_method {
    pub const HTTP_PURGE: llhttp_method = llhttp_method(29);
}
impl llhttp_method {
    pub const HTTP_MKCALENDAR: llhttp_method = llhttp_method(30);
}
impl llhttp_method {
    pub const HTTP_LINK: llhttp_method = llhttp_method(31);
}
impl llhttp_method {
    pub const HTTP_UNLINK: llhttp_method = llhttp_method(32);
}
impl llhttp_method {
    pub const HTTP_SOURCE: llhttp_method = llhttp_method(33);
}
impl llhttp_method {
    pub const HTTP_PRI: llhttp_method = llhttp_method(34);
}
impl llhttp_method {
    pub const HTTP_DESCRIBE: llhttp_method = llhttp_method(35);
}
impl llhttp_method {
    pub const HTTP_ANNOUNCE: llhttp_method = llhttp_method(36);
}
impl llhttp_method {
    pub const HTTP_SETUP: llhttp_method = llhttp_method(37);
}
impl llhttp_method {
    pub const HTTP_PLAY: llhttp_method = llhttp_method(38);
}
impl llhttp_method {
    pub const HTTP_PAUSE: llhttp_method = llhttp_method(39);
}
impl llhttp_method {
    pub const HTTP_TEARDOWN: llhttp_method = llhttp_method(40);
}
impl llhttp_method {
    pub const HTTP_GET_PARAMETER: llhttp_method = llhttp_method(41);
}
impl llhttp_method {
    pub const HTTP_SET_PARAMETER: llhttp_method = llhttp_method(42);
}
impl llhttp_method {
    pub const HTTP_REDIRECT: llhttp_method = llhttp_method(43);
}
impl llhttp_method {
    pub const HTTP_RECORD: llhttp_method = llhttp_method(44);
}
impl llhttp_method {
    pub const HTTP_FLUSH: llhttp_method = llhttp_method(45);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct llhttp_method(pub ::libc::c_uint);
pub use self::llhttp_method as llhttp_method_t;
pub type llhttp_t = llhttp__internal_t;
pub type llhttp_settings_t = llhttp_settings_s;
pub type llhttp_data_cb = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut llhttp_t, at: *const ::libc::c_char, length: usize) -> ::libc::c_int,
>;
pub type llhttp_cb = ::core::option::Option<unsafe extern "C" fn(arg1: *mut llhttp_t) -> ::libc::c_int>;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct llhttp_settings_s {
    pub on_message_begin: llhttp_cb,
    pub on_url: llhttp_data_cb,
    pub on_status: llhttp_data_cb,
    pub on_header_field: llhttp_data_cb,
    pub on_header_value: llhttp_data_cb,
    pub on_headers_complete: llhttp_cb,
    pub on_body: llhttp_data_cb,
    pub on_message_complete: llhttp_cb,
    pub on_chunk_header: llhttp_cb,
    pub on_chunk_complete: llhttp_cb,
    pub on_url_complete: llhttp_cb,
    pub on_status_complete: llhttp_cb,
    pub on_header_field_complete: llhttp_cb,
    pub on_header_value_complete: llhttp_cb,
}
#[test]
fn bindgen_test_layout_llhttp_settings_s() {
    assert_eq!(
        ::core::mem::size_of::<llhttp_settings_s>(),
        112usize,
        concat!("Size of: ", stringify!(llhttp_settings_s))
    );
    assert_eq!(
        ::core::mem::align_of::<llhttp_settings_s>(),
        8usize,
        concat!("Alignment of ", stringify!(llhttp_settings_s))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp_settings_s>())).on_message_begin as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp_settings_s),
            "::",
            stringify!(on_message_begin)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp_settings_s>())).on_url as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp_settings_s),
            "::",
            stringify!(on_url)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp_settings_s>())).on_status as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp_settings_s),
            "::",
            stringify!(on_status)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp_settings_s>())).on_header_field as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp_settings_s),
            "::",
            stringify!(on_header_field)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp_settings_s>())).on_header_value as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp_settings_s),
            "::",
            stringify!(on_header_value)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp_settings_s>())).on_headers_complete as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp_settings_s),
            "::",
            stringify!(on_headers_complete)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp_settings_s>())).on_body as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp_settings_s),
            "::",
            stringify!(on_body)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp_settings_s>())).on_message_complete as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp_settings_s),
            "::",
            stringify!(on_message_complete)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp_settings_s>())).on_chunk_header as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp_settings_s),
            "::",
            stringify!(on_chunk_header)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp_settings_s>())).on_chunk_complete as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp_settings_s),
            "::",
            stringify!(on_chunk_complete)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp_settings_s>())).on_url_complete as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp_settings_s),
            "::",
            stringify!(on_url_complete)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp_settings_s>())).on_status_complete as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp_settings_s),
            "::",
            stringify!(on_status_complete)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp_settings_s>())).on_header_field_complete as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp_settings_s),
            "::",
            stringify!(on_header_field_complete)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<llhttp_settings_s>())).on_header_value_complete as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(llhttp_settings_s),
            "::",
            stringify!(on_header_value_complete)
        )
    );
}
extern "C" {
    pub fn llhttp_init(parser: *mut llhttp_t, type_: llhttp_type_t, settings: *const llhttp_settings_t);
}
extern "C" {
    pub fn llhttp_reset(parser: *mut llhttp_t);
}
extern "C" {
    pub fn llhttp_settings_init(settings: *mut llhttp_settings_t);
}
extern "C" {
    pub fn llhttp_execute(parser: *mut llhttp_t, data: *const ::libc::c_char, len: usize) -> llhttp_errno_t;
}
extern "C" {
    pub fn llhttp_finish(parser: *mut llhttp_t) -> llhttp_errno_t;
}
extern "C" {
    pub fn llhttp_message_needs_eof(parser: *const llhttp_t) -> ::libc::c_int;
}
extern "C" {
    pub fn llhttp_should_keep_alive(parser: *const llhttp_t) -> ::libc::c_int;
}
extern "C" {
    pub fn llhttp_pause(parser: *mut llhttp_t);
}
extern "C" {
    pub fn llhttp_resume(parser: *mut llhttp_t);
}
extern "C" {
    pub fn llhttp_resume_after_upgrade(parser: *mut llhttp_t);
}
extern "C" {
    pub fn llhttp_get_errno(parser: *const llhttp_t) -> llhttp_errno_t;
}
extern "C" {
    pub fn llhttp_get_error_reason(parser: *const llhttp_t) -> *const ::libc::c_char;
}
extern "C" {
    pub fn llhttp_set_error_reason(parser: *mut llhttp_t, reason: *const ::libc::c_char);
}
extern "C" {
    pub fn llhttp_get_error_pos(parser: *const llhttp_t) -> *const ::libc::c_char;
}
extern "C" {
    pub fn llhttp_errno_name(err: llhttp_errno_t) -> *const ::libc::c_char;
}
extern "C" {
    pub fn llhttp_method_name(method: llhttp_method_t) -> *const ::libc::c_char;
}
extern "C" {
    pub fn llhttp_set_lenient_headers(parser: *mut llhttp_t, enabled: ::libc::c_int);
}
extern "C" {
    pub fn llhttp_set_lenient_chunked_length(parser: *mut llhttp_t, enabled: ::libc::c_int);
}
extern "C" {
    pub fn llhttp_set_lenient_keep_alive(parser: *mut llhttp_t, enabled: ::libc::c_int);
}
