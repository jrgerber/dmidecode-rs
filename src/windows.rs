use crate::Opt;
use smbioslib::*;
use std::io::Error;

mod dmiopt;

pub fn table_load(_opt: &Opt) -> Result<SMBiosData, Error> {
    table_load_from_device()
}
