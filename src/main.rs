use crate::regedit_error::RegeditError;

mod regedit_error;
mod registry_handler;
mod registry_value;
mod util;

type Result<T> = std::result::Result<T, RegeditError>;

fn main() -> Result<()> {
    let registry_key_name = "SOFTWARE\\Microsoft\\Internet Explorer\\IEDevTools\\";
    let registry_value_name = "Disabled";

    println!(
        "Registry Key name: {}\nRegistry Value name: {}\nCurrent value: {:?}\n",
        registry_key_name,
        registry_value_name,
        registry_handler::get_registry_value(registry_key_name, registry_value_name)?
    );

    //
    // set_registry_value(
    //     registry_key,
    //     registry_value,
    //     &RegistryValueType::RegDword,
    //     0u32.to_ne_bytes().to_vec(),
    // )?;
    //
    // println!(
    //     "Registry Key name: {}\nRegistry Value name: {}\nNew value: {}\n",
    //     registry_key,
    //     registry_value,
    //     get_registry_value(registry_key, registry_value, &registry_value_type)?
    // );

    Ok(())
}
