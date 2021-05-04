use crate::Opt;
use smbioslib::*;
use std::fmt::Write;
use std::io::Error;

mod dmiopt;

pub fn table_load(_opt: &Opt) -> Result<(SMBiosData, String), Error> {
    let mut output = String::new();

    writeln!(
        &mut output,
        "Getting SMBIOS data from GetSystemFirmwareTable()."
    )
    .unwrap();

    let smbios_table = table_load_from_device()?;

    Ok((smbios_table, output))
}
