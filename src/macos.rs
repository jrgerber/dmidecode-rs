use crate::Opt;
use smbioslib::*;
use std::io::Error;

mod dmiopt;

pub fn table_load(_opt: &Opt) -> Result<(SMBiosData, String), Error> {
    let mut output = String::new();

    writeln!(&mut output, "Getting SMBIOS data from IOKit.").unwrap();

    Ok((table_load_from_device(), output))
}
