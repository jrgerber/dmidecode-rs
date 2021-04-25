use crate::Opt;
use smbioslib::*;
use std::io::Error;

mod dmiopt;

pub fn table_load(opt: &Opt) -> Result<SMBiosData, Error> {
    if opt.no_sysfs {
        return table_load_from_dev_mem();
    }

    table_load_from_device()
}

/// Load from /dev/mem
fn table_load_from_dev_mem() -> Result<SMBiosData, Error> {
    use io::{Error, ErrorKind};
    const RANGE_START: u64 = 0x000F0000u64;
    const RANGE_END: u64 = 0x000FFFFFu64;
    let mut dev_mem = fs::File::open(DEV_MEM_FILE)?;
    let structure_table_address: u64;
    let structure_table_length: u32;
    let version: SMBiosVersion;

    match SMBiosEntryPoint32::try_scan_from_file(&mut dev_mem, RANGE_START..=RANGE_END) {
        Ok(entry_point) => {
            structure_table_address = entry_point.structure_table_address() as u64;
            structure_table_length = entry_point.structure_table_length() as u32;
            version = SMBiosVersion {
                major: entry_point.major_version(),
                minor: entry_point.minor_version(),
                revision: 0,
            };

            println!(
                "SMBIOS {}.{} present.",
                entry_point.major_version(),
                entry_point.minor_version()
            );
            println!(
                "{} structures occupying {} bytes.",
                entry_point.number_of_smbios_structures(),
                entry_point.structure_table_length()
            );
            println!("Table at: {:#010X}.", entry_point.structure_table_address());
        }
        Err(error) => {
            if error.kind() != ErrorKind::UnexpectedEof {
                return Err(Box::new(error));
            }

            let entry_point =
                SMBiosEntryPoint64::try_scan_from_file(&mut dev_mem, RANGE_START..=RANGE_END)?;

            structure_table_address = entry_point.structure_table_address();
            structure_table_length = entry_point.structure_table_maximum_size();
            version = SMBiosVersion {
                major: entry_point.major_version(),
                minor: entry_point.minor_version(),
                revision: entry_point.docrev(),
            };

            println!(
                "SMBIOS {}.{}.{} present.",
                entry_point.major_version(),
                entry_point.minor_version(),
                entry_point.docrev()
            );
            println!(
                "Occupying {} bytes maximum.",
                entry_point.structure_table_maximum_size()
            );
            println!("Table at: {:#010X}.", entry_point.structure_table_address());
        }
    }

    if structure_table_address < RANGE_START || structure_table_address > RANGE_END {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidData,
            format!(
                "The entry point has given an out of range start address for the table: {}",
                structure_table_address
            ),
        )));
    }

    if structure_table_address + structure_table_length as u64 > RANGE_END {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidData,
            format!(
                "The entry point has given a length which exceeds the range: {}",
                structure_table_length
            ),
        )));
    }

    let table = UndefinedStructTable::try_load_from_file_offset(
        &mut dev_mem,
        structure_table_address,
        structure_table_length as usize,
    )?;

    SMBiosData::new(table, Some(version))
}
