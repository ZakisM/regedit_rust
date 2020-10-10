use std::convert::TryInto;
use std::fmt::Formatter;

use winapi::_core::fmt;
use winapi::um::winnt::{
    REG_BINARY, REG_DWORD, REG_DWORD_BIG_ENDIAN, REG_EXPAND_SZ, REG_LINK, REG_MULTI_SZ, REG_NONE,
    REG_QWORD, REG_SZ,
};

use crate::regedit_error::RegeditError;
use crate::Result;

#[allow(dead_code)]
pub enum RegistryValue {
    Binary(Vec<u8>),
    Dword(u32),
    DwordBE(u32),
    ExpandSz(String),
    Link(String),
    MultiSz(String),
    None(Vec<u8>),
    Qword(u64),
    Sz(String),
}

impl std::fmt::Debug for RegistryValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::fmt::Display for RegistryValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RegistryValue::Dword(val) | RegistryValue::DwordBE(val) => write!(f, "{}", val),
            RegistryValue::ExpandSz(val)
            | RegistryValue::Link(val)
            | RegistryValue::MultiSz(val)
            | RegistryValue::Sz(val) => write!(f, "{}", val),
            RegistryValue::Binary(val) | RegistryValue::None(val) => write!(f, "{:?}", val),
            RegistryValue::Qword(val) => write!(f, "{}", val),
        }
    }
}

impl RegistryValue {
    pub fn new_from_value(value_type: u32, value: Vec<u8>) -> Result<Self> {
        match value_type {
            // REG_BINARY => Ok(RegistryValueType::Binary),
            REG_DWORD => {
                let res =
                    u32::from_ne_bytes(value.as_slice().try_into().map_err(RegeditError::from)?);

                Ok(Self::Dword(res))
            }
            // REG_DWORD_BIG_ENDIAN => Ok(RegistryValueType::DwordBE),
            // REG_EXPAND_SZ => Ok(RegistryValueType::ExpandSz),
            // REG_LINK => Ok(RegistryValueType::Link),
            // REG_MULTI_SZ => Ok(RegistryValueType::MultiSz),
            // REG_NONE => Ok(RegistryValueType::None),
            REG_QWORD => {
                let res =
                    u64::from_ne_bytes(value.as_slice().try_into().map_err(RegeditError::from)?);

                Ok(Self::Qword(res))
            }
            REG_SZ | REG_EXPAND_SZ | REG_MULTI_SZ => {
                let mut value_non_zero = value.to_vec();

                //remove all empty bytes from string
                value_non_zero.retain(|i| *i != 0);

                let res = String::from_utf8(value_non_zero)?;

                Ok(Self::Sz(res))
            }
            _ => Err(RegeditError::new("Unknown registry value type.")),
        }
    }

    pub fn size(&self) -> u32 {
        match *self {
            RegistryValue::Binary(_) => REG_BINARY,
            RegistryValue::Dword(_) => REG_DWORD,
            RegistryValue::DwordBE(_) => REG_DWORD_BIG_ENDIAN,
            RegistryValue::ExpandSz(_) => REG_EXPAND_SZ,
            RegistryValue::Link(_) => REG_LINK,
            RegistryValue::MultiSz(_) => REG_MULTI_SZ,
            RegistryValue::None(_) => REG_NONE,
            RegistryValue::Qword(_) => REG_QWORD,
            RegistryValue::Sz(_) => REG_SZ,
        }
    }
}
