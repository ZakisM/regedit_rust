use std::env::current_exe;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use winapi::shared::minwindef::HKEY;
use winapi::um::winreg::{
    HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS,
};

use crate::regedit_error::RegeditError;
use crate::registry_value::RegistryValue;
use crate::{registry_handler, Result};

pub const CONFIG_FILE_NAME: &str = "RegConfig.yml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip)]
    is_generated: bool,
    registry: Vec<RegistryEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryEntry {
    registry_key_type: RegistryKeyType,
    registry_key_name: String,
    registry_value_name: String,
    registry_value: RegistryValue,
}

impl RegistryEntry {
    pub fn registry_key_type(&self) -> &RegistryKeyType {
        &self.registry_key_type
    }

    pub fn registry_key_name(&self) -> &str {
        &self.registry_key_name
    }

    pub fn registry_value_name(&self) -> &str {
        &self.registry_value_name
    }

    pub fn registry_value(&self) -> &RegistryValue {
        &self.registry_value
    }

    pub fn current_registry_value_details(&self) -> Result<String> {
        let mut details = String::new();

        let curr_val = self.current_registry_value()?;

        details.push_str(&format!(
            "Registry Key Type: {:?}\n",
            self.registry_key_type()
        ));
        details.push_str(&format!(
            "Registry Key Name: {}\n",
            self.registry_key_name()
        ));
        details.push_str(&format!(
            "Registry Value Name: {}\n",
            self.registry_value_name()
        ));
        details.push_str(&format!(
            "Registry Value Type: {}\n",
            curr_val.type_as_string()
        ));
        details.push_str(&format!("Registry Value: {}\n", curr_val.val_as_string()));

        Ok(details)
    }

    pub fn current_registry_value(&self) -> Result<RegistryValue> {
        registry_handler::get_registry_value(
            &self.registry_key_type,
            &self.registry_key_name,
            &self.registry_value_name,
        )
    }

    pub fn set_registry_value(&self, data: Vec<u8>) -> Result<()> {
        registry_handler::set_registry_value(
            &self.registry_key_type,
            &self.registry_key_name,
            &self.registry_value_name,
            &self.registry_value,
            data,
        )
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RegistryKeyType {
    HkeyClassesRoot,
    HkeyCurrentUser,
    HkeyLocalMachine,
    HkeyUsers,
    HkeyCurrentConfig,
}

impl RegistryKeyType {
    pub fn value(&self) -> HKEY {
        match *self {
            RegistryKeyType::HkeyClassesRoot => HKEY_CLASSES_ROOT,
            RegistryKeyType::HkeyCurrentUser => HKEY_CURRENT_USER,
            RegistryKeyType::HkeyLocalMachine => HKEY_LOCAL_MACHINE,
            RegistryKeyType::HkeyUsers => HKEY_USERS,
            RegistryKeyType::HkeyCurrentConfig => HKEY_CURRENT_CONFIG,
        }
    }
}

impl Config {
    pub fn is_generated(&self) -> bool {
        self.is_generated
    }

    pub fn generate() -> Self {
        println!("\nGenerating new config...\n");

        let sample_val_1 = RegistryEntry {
            registry_key_type: RegistryKeyType::HkeyCurrentUser,
            registry_key_name: "Sample Key Name".to_string(),
            registry_value_name: "Sample Key Value".to_string(),
            registry_value: RegistryValue::Dword(1),
        };

        let sample_val_2 = RegistryEntry {
            registry_key_type: RegistryKeyType::HkeyCurrentUser,
            registry_key_name: "Other Sample Key Name".to_string(),
            registry_value_name: "Other Sample Key Value".to_string(),
            registry_value: RegistryValue::Sz("Example string".to_owned()),
        };

        Self {
            is_generated: true,
            registry: vec![sample_val_1, sample_val_2],
        }
    }

    pub fn save(&self) -> Result<PathBuf> {
        let yaml_str = serde_yaml::to_string(&self)?;

        let conf_path = Self::get_conf_path()?;

        fs::write(&conf_path, yaml_str)?;

        Ok(conf_path)
    }

    pub fn load() -> Result<Self> {
        let conf_path = Self::get_conf_path()?;

        if conf_path.exists() {
            println!("\nLoading existing config...\n");

            let config_file = fs::read(conf_path)?;

            Ok(serde_yaml::from_slice::<Config>(config_file.as_slice())?)
        } else {
            Ok(Self::generate())
        }
    }

    pub fn registry(&self) -> &Vec<RegistryEntry> {
        self.registry.as_ref()
    }

    fn get_conf_path() -> Result<PathBuf> {
        let exe_path = current_exe()?
            .parent()
            .ok_or_else(|| RegeditError::new("Failed to get current executable path so that a config file can be created there"))?
            .to_path_buf();

        Ok(exe_path.join(&CONFIG_FILE_NAME))
    }
}
