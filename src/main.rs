use crate::config::CONFIG_FILE_NAME;
use crate::regedit_error::RegeditError;

mod config;
mod regedit_error;
mod registry_handler;
mod registry_value;
mod util;

type Result<T> = std::result::Result<T, RegeditError>;

fn main() -> Result<()> {
    let conf = config::Config::load()?;

    if conf.is_generated() {
        let conf_path = conf.save()?;

        println!("A sample config file '{}' has been generated at '{}'. Please modify this before re-running the tool.", CONFIG_FILE_NAME, conf_path)
    } else {
        for entry in conf.registry() {
            match entry.current_registry_value_details() {
                Ok(details) => {
                    println!("\n~~~PREVIOUS VALUE~~~\n");
                    println!("{}", details);
                }
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            }

            if let Err(e) = entry.set_registry_value(entry.registry_value().val_as_vec_u8()) {
                println!("{}", e);
                continue;
            }

            match entry.current_registry_value_details() {
                Ok(details) => {
                    println!("\n~~~NEW VALUE~~~\n");
                    println!("{}", details);
                }
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            }
        }
    }

    Ok(())
}
