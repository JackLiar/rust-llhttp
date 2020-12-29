#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

extern crate llhttp_sys as llhttp;

use std::ffi::CStr;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use num_traits::{FromPrimitive, ToPrimitive};

mod consts;
pub use consts::*;

pub type CallBack = llhttp::llhttp_cb;
pub type DataCallBack = llhttp::llhttp_data_cb;

#[derive(Copy, Clone, Debug, Default)]
pub struct Settings(llhttp::llhttp_settings_t);

unsafe impl Send for Settings {}

impl Deref for Settings {
    type Target = llhttp::llhttp_settings_t;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Settings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Settings {
    pub fn new() -> Settings {
        let mut settings = Settings::default();
        unsafe {
            llhttp::llhttp_settings_init(settings.deref_mut());
        }
        settings
    }
}

/// llhttp parser
#[derive(Clone)]
pub struct Parser {
    _llhttp: llhttp::llhttp_t,
}

impl Parser {
    /// Create a new llhttp parser
    pub fn new() -> Parser {
        let _llhttp = llhttp::llhttp_t {
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
            lenient_flags: LenientFlags::HEADERS.to_u8().unwrap(),
        };
        Parser { _llhttp }
    }

    #[inline]
    pub fn init(&mut self, settings: &Settings, lltype: Type) {
        unsafe {
            llhttp::llhttp_init(self.deref_mut(), lltype.into(), settings.deref());
        }
    }

    #[inline]
    pub fn parse(&mut self, data: &[u8]) -> Error {
        let err;
        unsafe {
            err = llhttp::llhttp_execute(self.deref_mut(), data.as_ptr() as *const i8, data.len());
        }
        match Error::from_u32(err) {
            Some(i) => i,
            None => unreachable!(),
        }
    }

    #[inline]
    pub fn finish(&mut self) -> Error {
        let err = unsafe { llhttp::llhttp_finish(self.deref_mut()) };
        match Error::from_u32(err) {
            Some(i) => i,
            None => unreachable!(),
        }
    }

    #[inline]
    pub fn message_needs_eof(&self) -> bool {
        unsafe {
            match llhttp::llhttp_message_needs_eof(self.deref()) {
                1 => true,
                _ => false,
            }
        }
    }

    #[inline]
    pub fn should_keep_alive(&self) -> bool {
        unsafe {
            match llhttp::llhttp_should_keep_alive(self.deref()) {
                1 => true,
                _ => false,
            }
        }
    }

    #[inline]
    pub fn pause(&mut self) {
        unsafe { llhttp::llhttp_pause(self.deref_mut()) }
    }

    #[inline]
    pub fn resume(&mut self) {
        unsafe { llhttp::llhttp_resume(self.deref_mut()) }
    }

    #[inline]
    pub fn resume_after_upgrade(&mut self) {
        unsafe { llhttp::llhttp_resume_after_upgrade(self.deref_mut()) }
    }

    #[inline]
    pub fn errno(&self) -> Error {
        unsafe { Error::from_u32(llhttp::llhttp_get_errno(self.deref())).unwrap() }
    }

    #[inline]
    pub fn get_error_reason(&self) -> &CStr {
        unsafe { CStr::from_ptr(llhttp::llhttp_get_error_reason(self.deref())) }
    }

    #[inline]
    pub fn get_error_pos(&self) -> &CStr {
        unsafe { CStr::from_ptr(llhttp::llhttp_get_error_pos(self.deref())) }
    }

    #[inline]
    pub fn status_code(&self) -> u16 {
        self.deref().status_code
    }

    #[inline]
    pub fn method(&self) -> Method {
        match Method::from_u8(self.deref().method) {
            Some(m) => m,
            None => unreachable!(),
        }
    }
}

impl Deref for Parser {
    type Target = llhttp::llhttp_t;
    fn deref(&self) -> &Self::Target {
        &self._llhttp
    }
}

impl DerefMut for Parser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._llhttp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method() {
        let settings = Settings::new();
        let mut parser = Parser::new();
        parser.init(&settings, Type::BOTH);

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
        assert!(matches!(parser.method(), Method::NOTIFY));

        let payload = b"GET /ocsp-devid01/ME4wTKADAgEAMEUwQzBBMAkGBSsOAwIaBQAEFDOB0e%2FbaLCFIU0u76%2BMSmlkPCpsBBRXF%2B2iz9x8mKEQ4Py%2Bhy0s8uMXVAIITaFtmUYgLaY%3D HTTP/1.1\r\n
        Host: ocsp.apple.com\r\n
        Accept: */*\r\n
        Accept-Language: zh-cn\r\n
        Connection: keep-alive\r\n
        Accept-Encoding: gzip, deflate\r\n
        User-Agent: com.apple.trustd/2.0\r\n";

        let mut parser = Parser::new();
        parser.init(&settings, Type::BOTH);

        parser.parse(payload);
        println!("{:?}", parser.method());
        assert!(matches!(parser.method(), Method::GET));
    }
}
