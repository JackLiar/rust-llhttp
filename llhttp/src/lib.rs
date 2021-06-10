extern crate libc;

extern crate llhttp_sys;

use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::{c_char, c_uint};

mod consts;
pub mod ffi {
    pub use llhttp_sys::*;
}

pub use consts::*;
use ffi::llhttp_method_name;

pub type CallBack = ffi::llhttp_cb;
pub type DataCallBack = ffi::llhttp_data_cb;

#[derive(Copy, Clone, Debug, Default)]
pub struct Settings(ffi::llhttp_settings_t);

unsafe impl Send for Settings {}

#[macro_export]
macro_rules! cb_wrapper {
    ($fname:ident, $func:ident, $data_type:ty) => {
        #[inline]
        unsafe extern "C" fn $fname(arg1: *mut llhttp::ffi::llhttp_t) -> libc::c_int {
            let parser = &mut *(arg1 as *mut llhttp::Parser<$data_type>);
            match $func(parser) {
                Ok(_) => 0,
                Err(_) => -1,
            }
        }
    };
}

#[macro_export]
macro_rules! data_cb_wrapper {
    ($fname:ident, $func:ident, $data_type:ty) => {
        #[inline]
        unsafe extern "C" fn $fname(
            arg1: *mut llhttp::ffi::llhttp_t,
            at: *const ::libc::c_char,
            length: usize,
        ) -> libc::c_int {
            let parser = &mut *(arg1 as *mut llhttp::Parser<$data_type>);
            let data = std::slice::from_raw_parts(at as *const u8, length);
            match $func(parser, data) {
                Ok(_) => 0,
                Err(_) => -1,
            }
        }
    };
}

impl Settings {
    pub fn on_message_begin(&mut self, cb: CallBack) {
        self.0.on_message_begin = cb;
    }
    pub fn on_url(&mut self, cb: DataCallBack) {
        self.0.on_url = cb;
    }
    pub fn on_status(&mut self, cb: DataCallBack) {
        self.0.on_status = cb;
    }
    pub fn on_header_field(&mut self, cb: DataCallBack) {
        self.0.on_header_field = cb;
    }
    pub fn on_header_value(&mut self, cb: DataCallBack) {
        self.0.on_header_value = cb;
    }
    pub fn on_headers_complete(&mut self, cb: CallBack) {
        self.0.on_headers_complete = cb;
    }
    pub fn on_body(&mut self, cb: DataCallBack) {
        self.0.on_body = cb;
    }
    pub fn on_message_complete(&mut self, cb: CallBack) {
        self.0.on_message_complete = cb;
    }
    pub fn on_chunk_header(&mut self, cb: CallBack) {
        self.0.on_chunk_header = cb;
    }
    pub fn on_chunk_complete(&mut self, cb: CallBack) {
        self.0.on_chunk_complete = cb;
    }
    pub fn on_url_complete(&mut self, cb: CallBack) {
        self.0.on_url_complete = cb;
    }
    pub fn on_status_complete(&mut self, cb: CallBack) {
        self.0.on_status_complete = cb;
    }
    pub fn on_header_field_complete(&mut self, cb: CallBack) {
        self.0.on_header_field_complete = cb;
    }
    pub fn on_header_value_complete(&mut self, cb: CallBack) {
        self.0.on_header_value_complete = cb;
    }
}

impl Settings {
    pub fn new() -> Settings {
        let mut settings = Settings::default();
        unsafe {
            ffi::llhttp_settings_init(&mut settings.0);
        }
        settings
    }
}

/// llhttp parser
#[derive(Clone, Default)]
pub struct Parser<'a, T> {
    _llhttp: ffi::llhttp_t,
    _settings: PhantomData<&'a Settings>,
    _data: PhantomData<T>,
}

impl<'a, T> Parser<'a, T> {
    #[inline]
    pub fn init(&mut self, settings: &Settings, lltype: Type) {
        unsafe {
            ffi::llhttp_init(&mut self._llhttp, lltype.into(), &settings.0);
        }
    }

    #[inline]
    pub fn parse(&mut self, data: &[u8]) -> Error {
        unsafe { ffi::llhttp_execute(&mut self._llhttp, data.as_ptr() as *const c_char, data.len()) }
    }

    #[inline]
    pub fn finish(&mut self) -> Error {
        unsafe { ffi::llhttp_finish(&mut self._llhttp) }
    }

    #[inline]
    pub fn message_needs_eof(&self) -> bool {
        unsafe {
            match ffi::llhttp_message_needs_eof(&self._llhttp) {
                1 => true,
                _ => false,
            }
        }
    }

    #[inline]
    pub fn should_keep_alive(&self) -> bool {
        unsafe {
            match ffi::llhttp_should_keep_alive(&self._llhttp) {
                1 => true,
                _ => false,
            }
        }
    }

    #[inline]
    pub fn pause(&mut self) {
        unsafe { ffi::llhttp_pause(&mut self._llhttp) }
    }

    #[inline]
    pub fn resume(&mut self) {
        unsafe { ffi::llhttp_resume(&mut self._llhttp) }
    }

    #[inline]
    pub fn resume_after_upgrade(&mut self) {
        unsafe { ffi::llhttp_resume_after_upgrade(&mut self._llhttp) }
    }

    #[inline]
    pub fn errno(&self) -> Error {
        unsafe { ffi::llhttp_get_errno(&self._llhttp) }
    }

    #[inline]
    pub fn get_error_reason(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::llhttp_get_error_reason(&self._llhttp)) }
    }

    #[inline]
    pub fn get_error_pos(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::llhttp_get_error_pos(&self._llhttp)) }
    }

    #[inline]
    pub fn status_code(&self) -> u16 {
        self._llhttp.status_code
    }

    #[inline]
    pub fn data(&self) -> Option<&mut T> {
        if self._llhttp.data.is_null() {
            None
        } else {
            unsafe { Some(&mut *(self._llhttp.data as *mut T)) }
        }
    }

    #[inline]
    /// Retrieve old data, and set new data
    pub fn set_data(&mut self, data: Option<Box<T>>) -> Option<Box<T>> {
        let old = if !self._llhttp.data.is_null() {
            unsafe { Some(Box::from_raw(self._llhttp.data as *mut T)) }
        } else {
            None
        };

        match data {
            Some(data) => self._llhttp.data = Box::into_raw(data) as *mut libc::c_void,
            None => self._llhttp.data = std::ptr::null_mut(),
        }

        old
    }

    #[inline]
    pub fn method(&self) -> Method {
        ffi::llhttp_method(self._llhttp.method as c_uint)
    }

    #[inline]
    pub fn method_name(&self) -> &str {
        unsafe {
            let method = llhttp_method_name(self.method());
            let method = std::ffi::CStr::from_ptr(method);
            match method.to_str() {
                Err(_) => unreachable!(),
                Ok(method) => method,
            }
        }
    }

    #[inline]
    pub fn lltype(&self) -> Type {
        ffi::llhttp_type_t(self._llhttp.type_ as c_uint)
    }

    #[inline]
    pub fn reset(&mut self) {
        unsafe { ffi::llhttp_reset(&self._llhttp as *const _ as *mut _) }
    }

    #[inline]
    pub fn major(&self) -> u8 {
        self._llhttp.http_major
    }

    #[inline]
    pub fn minor(&self) -> u8 {
        self._llhttp.http_minor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method() {
        let settings = Settings::new();
        let mut parser = Parser::<()>::default();
        parser.init(&settings, Type::HTTP_BOTH);

        let payload = r#"NOTIFY * HTTP/1.1\r
        HOST: 239.255.255.250:1900\r
        CACHE-CONTROL: max-age=60\r
        LOCATION: http://192.168.2.1:5000/rootDesc.xml\r
        SERVER: K2A5/OpenWrt/Barrier_Breaker__unknown_ UPnP/1.1 MiniUPnPd/1.8\r
        NT: upnp:rootdevice\r
        USN: uuid:0aeec1da-795c-448c-864b-11b838fe5945::upnp:rootdevice\r
        NTS: ssdp:alive\r
        OPT: "http://schemas.upnp.org/upnp/1/0/"; ns=01\r
        01-NLS: 1\r
        BOOTID.UPNP.ORG: 1\r
        CONFIGID.UPNP.ORG: 1337\r\n\r\n"#;
        parser.parse(payload.as_bytes());
        assert!(matches!(parser.method(), Method::HTTP_NOTIFY));

        let payload = b"GET /ocsp-devid01/ME4wTKADAgEAMEUwQzBBMAkGBSsOAwIaBQAEFDOB0e%2FbaLCFIU0u76%2BMSmlkPCpsBBRXF%2B2iz9x8mKEQ4Py%2Bhy0s8uMXVAIITaFtmUYgLaY%3D HTTP/1.1\r\n
        Host: ocsp.apple.com\r\n
        Accept: */*\r\n
        Accept-Language: zh-cn\r\n
        Connection: keep-alive\r\n
        Accept-Encoding: gzip, deflate\r\n
        User-Agent: com.apple.trustd/2.0\r\n";

        let mut parser = Parser::<()>::default();
        parser.init(&settings, Type::HTTP_BOTH);

        parser.parse(payload);
        println!("{:?}", parser.method());
        assert!(matches!(parser.method(), Method::HTTP_GET));
    }
}
