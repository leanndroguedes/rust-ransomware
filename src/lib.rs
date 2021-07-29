use std::io;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

pub fn registry_autostart() -> Result<(), io::Error> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let path = Path::new("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run");

    let (run, disp) = hklm.create_subkey(&path)?;

    match disp {
        REG_CREATED_NEW_KEY => println!("A new key has been created"),
        REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
    }

    run.set_value("Custom", &"C:\\Windows\\notepad.exe")?; // Replace in future

    Ok(())
}
