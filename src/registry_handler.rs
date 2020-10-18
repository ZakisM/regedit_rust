use std::convert::TryInto;
use std::ptr::null_mut;

use winapi::um::winnt::{
    KEY_READ, KEY_SET_VALUE, REG_BINARY, REG_DWORD, REG_DWORD_BIG_ENDIAN, REG_EXPAND_SZ, REG_LINK,
    REG_MULTI_SZ, REG_NONE, REG_QWORD, REG_SZ,
};
use winapi::um::winreg::{RegOpenKeyExW, RegQueryValueExW, RegSetValueExW};

use crate::config::RegistryKeyType;
use crate::regedit_error::RegeditError;
use crate::registry_value::RegistryValue;
use crate::util::StringExt;
use crate::Result;

pub fn get_registry_value(
    registry_key_type: &RegistryKeyType,
    registry_key_name: &str,
    registry_value_name: &str,
) -> Result<RegistryValue> {
    let hkey_handle = open_registry_key(registry_key_type, registry_key_name, KEY_READ)?;

    let lp_value = registry_value_name.to_lpcwstr();

    //default type
    let mut raw_type = 0u32;

    let value_type_res = unsafe {
        RegQueryValueExW(
            hkey_handle,
            lp_value.as_ptr(),
            null_mut::<u32>(),
            &mut raw_type,
            null_mut::<u8>(),
            null_mut::<u32>(),
        )
    };

    if value_type_res > 0 {
        return Err(RegeditError::new(format!(
            "Failed to read registry value type for: '{}'",
            registry_key_name
        )));
    }

    let mut buff_size: u32 = match raw_type {
        REG_BINARY => 4096,
        REG_DWORD => 4,
        REG_DWORD_BIG_ENDIAN => 4,
        REG_EXPAND_SZ => 4096,
        REG_LINK => 4096,
        REG_MULTI_SZ => 4096,
        REG_NONE => 4096,
        REG_QWORD => 8,
        REG_SZ => 4096,
        _ => return Err(RegeditError::new("Unknown registry value buffer size")),
    };

    let mut reg_key_value: Vec<u8> = vec![0; buff_size.try_into().map_err(RegeditError::from)?];

    let read_res = unsafe {
        RegQueryValueExW(
            hkey_handle,
            lp_value.as_ptr(),
            null_mut::<u32>(),
            &mut raw_type,
            reg_key_value.as_mut_ptr(),
            &mut buff_size,
        )
    };

    reg_key_value.resize(buff_size.try_into().map_err(RegeditError::from)?, 0);

    match read_res {
        0 => Ok(RegistryValue::new_from_value(raw_type, reg_key_value)?),
        _ => Err(RegeditError::new(format!(
            "Failed to read registry value for: '{}'",
            registry_key_name
        ))),
    }
}

pub fn set_registry_value(
    registry_key_type: &RegistryKeyType,
    registry_key_name: &str,
    registry_value_name: &str,
    registry_value: &RegistryValue,
    mut data: Vec<u8>,
) -> Result<()> {
    let hkey_handle = open_registry_key(registry_key_type, registry_key_name, KEY_SET_VALUE)?;

    let lp_value = registry_value_name.to_lpcwstr();

    let set_res = unsafe {
        RegSetValueExW(
            hkey_handle,
            lp_value.as_ptr(),
            0,
            registry_value.size(),
            data.as_mut_ptr(),
            data.len().try_into().map_err(RegeditError::from)?,
        )
    };

    match set_res {
        0 => Ok(()),
        _ => Err(RegeditError::new(format!(
            "Failed to set registry value for: '{}'",
            registry_key_name
        ))),
    }
}

fn open_registry_key(
    registry_key_type: &RegistryKeyType,
    registry_key_name: &str,
    access_rights: u32,
) -> Result<*mut winapi::shared::minwindef::HKEY__> {
    let lp_sub_key = registry_key_name.to_lpcwstr();

    let mut hkey_handle = unsafe { std::mem::zeroed() };

    let open_key_res = unsafe {
        RegOpenKeyExW(
            registry_key_type.value(),
            lp_sub_key.as_ptr(),
            0,
            access_rights,
            &mut hkey_handle,
        )
    };

    match open_key_res {
        0 => Ok(hkey_handle),
        _ => Err(RegeditError::new(format!(
            "Failed to open registry key: '{}'",
            registry_key_name
        ))),
    }
}
