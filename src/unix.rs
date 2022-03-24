use crate::Opt;
use io::{Error, ErrorKind};
use smbioslib::*;
use std::{fmt::Write, path::Path};

mod dmiopt;

#[cfg(target_os = "linux")]
pub fn table_load(opt: &Opt) -> Result<(SMBiosData, String), Error> {
    if !opt.no_sysfs {
        // read from /sys/firmware/dmi/tables/DMI
        if let Ok(smbios_data) = table_load_from_sysfs() {
            return Ok(smbios_data);
        }
    }

    // read from /dev/mem (default) or a given device file.
    let path = match &opt.dev_mem {
        Some(given_file) => given_file.as_path(),
        None => std::path::Path::new(DEV_MEM_FILE),
    };

    table_load_from_dev_mem(&path)
}

#[cfg(target_os = "freebsd")]
pub fn table_load(_opt: &Opt) -> Result<(SMBiosData, String), Error> {
    // FreeBSD only has /dev/mem and does not have sysfs (/sys/firmware/dmi/tables/DMI)
    let path = match &opt.dev_mem {
        Some(given_file) => given_file.as_path(),
        None => std::path::Path::new(DEV_MEM_FILE),
    };

    table_load_from_dev_mem(&path)
}

/// Load from /sys/firmware/dmi/tables/DMI
fn table_load_from_sysfs() -> Result<(SMBiosData, String), Error> {
    let mut output = String::new();

    writeln!(&mut output, "Getting SMBIOS data from sysfs.").unwrap();

    let version: SMBiosVersion;
    let entry_path = std::path::Path::new(SYS_ENTRY_FILE);

    match SMBiosEntryPoint64::try_load_from_file(entry_path) {
        Ok(entry_point) => {
            version = SMBiosVersion {
                major: entry_point.major_version(),
                minor: entry_point.minor_version(),
                revision: entry_point.docrev(),
            };

            writeln!(
                &mut output,
                "SMBIOS {}.{}.{} present.",
                entry_point.major_version(),
                entry_point.minor_version(),
                entry_point.docrev()
            )
            .unwrap();

            writeln!(
                &mut output,
                "Table at {:#010X}.",
                entry_point.structure_table_address()
            )
            .unwrap();
        }
        Err(err) => match err.kind() {
            ErrorKind::InvalidData => match SMBiosEntryPoint32::try_load_from_file(entry_path) {
                Ok(entry_point) => {
                    version = SMBiosVersion {
                        major: entry_point.major_version(),
                        minor: entry_point.minor_version(),
                        revision: 0,
                    };

                    writeln!(
                        &mut output,
                        "SMBIOS {}.{} present.",
                        entry_point.major_version(),
                        entry_point.minor_version()
                    )
                    .unwrap();

                    writeln!(
                        &mut output,
                        "{} structures occupying {} bytes.",
                        entry_point.number_of_smbios_structures(),
                        entry_point.structure_table_length()
                    )
                    .unwrap();

                    writeln!(
                        &mut output,
                        "Table at {:#010X}.",
                        entry_point.structure_table_address()
                    )
                    .unwrap();
                }
                Err(err) => return Err(err),
            },
            _ => return Err(err),
        },
    }

    let smbios_data = SMBiosData::try_load_from_file(SYS_TABLE_FILE, Some(version))?;

    Ok((smbios_data, output))
}

/// Load from /dev/mem
fn table_load_from_dev_mem(path: &Path) -> Result<(SMBiosData, String), Error> {
    const RANGE_START: u64 = 0x000F0000u64;
    const RANGE_END: u64 = 0x000FFFFFu64;
    let mut dev_mem = fs::File::open(path)?;
    let structure_table_address: u64;
    let structure_table_length: u32;
    let version: SMBiosVersion;
    let mut output = String::new();

    writeln!(
        &mut output,
        "Scanning {} for entry point.",
        path.to_string_lossy()
    )
    .unwrap();

    // First try 32 bit entry point
    match SMBiosEntryPoint32::try_scan_from_file(&mut dev_mem, RANGE_START..=RANGE_END) {
        Ok(entry_point) => {
            structure_table_address = entry_point.structure_table_address() as u64;
            structure_table_length = entry_point.structure_table_length() as u32;
            version = SMBiosVersion {
                major: entry_point.major_version(),
                minor: entry_point.minor_version(),
                revision: 0,
            };

            writeln!(
                &mut output,
                "SMBIOS {}.{} present.",
                entry_point.major_version(),
                entry_point.minor_version()
            )
            .unwrap();

            writeln!(
                &mut output,
                "{} structures occupying {} bytes.",
                entry_point.number_of_smbios_structures(),
                entry_point.structure_table_length()
            )
            .unwrap();
        }
        Err(error) => {
            // UnexpectedEof means the 32 bit entry point was not found and
            // the 64 bit entry point can be tried next.  Any other failure we report.
            if error.kind() != ErrorKind::UnexpectedEof {
                return Err(error);
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

            writeln!(
                &mut output,
                "SMBIOS {}.{}.{} present.",
                entry_point.major_version(),
                entry_point.minor_version(),
                entry_point.docrev()
            )
            .unwrap();

            writeln!(
                &mut output,
                "Occupying {} bytes maximum.",
                entry_point.structure_table_maximum_size()
            )
            .unwrap();
        }
    }

    writeln!(&mut output, "Table at {:#010X}.", structure_table_address).unwrap();

    if structure_table_address + structure_table_length as u64 > RANGE_END {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "The entry point has given a length which exceeds the range: {}",
                structure_table_length
            ),
        ));
    }

    let table = UndefinedStructTable::try_load_from_file_offset(
        &mut dev_mem,
        structure_table_address,
        structure_table_length as usize,
    )?;

    Ok((SMBiosData::new(table, Some(version)), output))
}
