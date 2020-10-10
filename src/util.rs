use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

pub trait StringExt {
    fn to_lpcwstr(&self) -> Vec<u16>;
}

impl StringExt for &str {
    #[inline]
    fn to_lpcwstr(&self) -> Vec<u16> {
        OsStr::new(self)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<_>>()
    }
}
