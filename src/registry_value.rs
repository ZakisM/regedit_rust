use std::convert::TryInto;

use serde::{Deserialize, Serialize};
use winapi::um::winnt::{
    REG_BINARY, REG_DWORD, REG_DWORD_BIG_ENDIAN, REG_EXPAND_SZ, REG_LINK, REG_MULTI_SZ, REG_NONE,
    REG_QWORD, REG_SZ,
};

use crate::regedit_error::RegeditError;
use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
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

impl RegistryValue {
    pub fn new_from_value(value_type: u32, value: Vec<u8>) -> Result<Self> {
        match value_type {
            REG_BINARY => Ok(Self::Binary(value)),
            REG_DWORD => {
                let res =
                    u32::from_ne_bytes(value.as_slice().try_into().map_err(RegeditError::from)?);

                Ok(Self::Dword(res))
            }
            REG_DWORD_BIG_ENDIAN => {
                let res =
                    u32::from_be_bytes(value.as_slice().try_into().map_err(RegeditError::from)?);

                Ok(Self::DwordBE(res))
            }
            REG_NONE => Ok(Self::None(value)),
            REG_QWORD => {
                let res =
                    u64::from_ne_bytes(value.as_slice().try_into().map_err(RegeditError::from)?);

                Ok(Self::Qword(res))
            }
            REG_SZ | REG_EXPAND_SZ | REG_MULTI_SZ | REG_LINK => {
                let res = String::from_utf8(value)?;

                Ok(Self::Sz(res))
            }
            _ => Err(RegeditError::new("Unknown registry value type")),
        }
    }

    pub fn val_as_string(&self) -> String {
        match self {
            RegistryValue::Binary(val) | RegistryValue::None(val) => format!("{:?}", val),
            RegistryValue::Dword(val) | RegistryValue::DwordBE(val) => val.to_string(),
            RegistryValue::Qword(val) => val.to_string(),
            RegistryValue::ExpandSz(val)
            | RegistryValue::Link(val)
            | RegistryValue::MultiSz(val)
            | RegistryValue::Sz(val) => val.to_string(),
        }
    }

    pub fn val_as_vec_u8(&self) -> Vec<u8> {
        match &*self {
            RegistryValue::Binary(val) | RegistryValue::None(val) => val.to_vec(),
            RegistryValue::Dword(val) | RegistryValue::DwordBE(val) => val.to_ne_bytes().to_vec(),
            RegistryValue::Qword(val) => val.to_ne_bytes().to_vec(),
            RegistryValue::ExpandSz(val)
            | RegistryValue::Link(val)
            | RegistryValue::MultiSz(val)
            | RegistryValue::Sz(val) => val.as_bytes().to_vec(),
        }
    }

    pub fn type_as_string(&self) -> String {
        let t = match *self {
            RegistryValue::Binary(_) => "Binary",
            RegistryValue::Dword(_) => "Dword",
            RegistryValue::DwordBE(_) => "DwordBE",
            RegistryValue::ExpandSz(_) => "ExpandSz",
            RegistryValue::Link(_) => "Link",
            RegistryValue::MultiSz(_) => "MultiSz",
            RegistryValue::None(_) => "None",
            RegistryValue::Qword(_) => "Qword",
            RegistryValue::Sz(_) => "Sz",
        };

        format!("REG_{}", t.to_uppercase())
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
