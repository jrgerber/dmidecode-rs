use crate::Opt;
use smbioslib::*;
use std::io::Error;

mod main;

pub fn table_load(opt: Opt) -> Result<SMBiosData, Error> {
    table_load_from_device()
}
