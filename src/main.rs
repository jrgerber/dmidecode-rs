//! # dmidecode command-line tool executable

#![warn(missing_docs)]
#![deny(rust_2018_idioms)]

#[cfg_attr(any(target_os = "linux", target_os = "freebsd"), path = "unix.rs")]
#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(macos, path = "macos.rs")]
mod platform;

mod dmiopt;
mod error;

use dmiopt::{BiosType, Keyword, Opt};
use enum_iterator::IntoEnumIterator;
use smbioslib::*;
use structopt::StructOpt;

/* The original DMI decode command line:

[root@BSDJRF /usr/home/jeff]# dmidecode --help
    Usage: dmidecode [OPTIONS]
    Options are:
    -d, --dev-mem FILE     Read memory from device FILE (default: /dev/mem)
    -h, --help             Display this help text and exit
    -q, --quiet            Less verbose output
    -s, --string KEYWORD   Only display the value of the given DMI string
    -t, --type TYPE        Only display the entries of given type
    -H, --handle HANDLE    Only display the entry of given handle
    -u, --dump             Do not decode the entries
        --dump-bin FILE    Dump the DMI data to a binary file
        --from-dump FILE   Read the DMI data from a binary file
        --no-sysfs         Do not attempt to read DMI data from sysfs files
        --oem-string N     Only display the value of the given OEM string
    -V, --version          Display the version and exit

    Options --string, --type, --dump-bin and --oem-string determine the
    output format and are mutually exclusive.
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt: Opt = Opt::from_args();

    if opt.has_no_args() {
        println!("{:#X?}", platform::table_load(&opt)?);
        return Ok(());
    }

    // Select an input source, file or device.
    let smbios_data = if let Some(input) = opt.input {
        load_smbios_data_from_file(&input.as_path())?
    } else {
        platform::table_load(&opt)?
    };

    // Mutually exclusive output options (only one tuple element is Some()).
    match (
        opt.keyword,
        opt.output,
        opt.bios_types,
        opt.handle,
        opt.oem_string,
        opt.undefined_dump,
        opt.list,
    ) {
        (Some(keyword), None, None, None, None, false, false) => {
            let output = keyword.parse(&smbios_data)?;
            println!("{}", output);
        }
        (None, Some(output), None, None, None, false, false) => {
            dump_raw(raw_smbios_from_device()?, &output.as_path())?
        }
        (None, None, Some(bios_types), None, None, false, false) => {
            BiosType::parse_and_display(bios_types, &smbios_data);
        }
        (None, None, None, Some(handle), None, false, false) => {
            let found_struct = smbios_data
                .find_by_handle(&handle)
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Handle not found: {}", *handle),
                ))?;
            println!("{:#X?}", &found_struct.defined_struct())
        }
        (None, None, None, None, Some(oem), false, false) => {
            fn invalid_num(s: &str) -> Result<(), Box<dyn std::error::Error>> {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid OEM string number {}", s),
                )))
            }
            let oem_val = oem.trim().parse::<u8>();
            let mut index = 0;
            match oem_val {
                Ok(n) => {
                    if n == 0 {
                        invalid_num(oem.as_str())?
                    }
                    index = n
                }
                Err(_) => {
                    if oem != "count" {
                        invalid_num(oem.as_str())?
                    }
                }
            }
            match smbios_data.first::<SMBiosOemStrings<'_>>() {
                Some(v) => {
                    match v.oem_strings().get_string(index) {
                        Some(s) => println!("{}", s),
                        None => {
                            if index != 0 {
                                // count
                                invalid_num(oem.as_str())?
                            }
                            println!("{}", v.count().unwrap());
                        }
                    }
                }
                None => invalid_num(oem.as_str())?,
            }
        }
        (None, None, None, None, None, true, false) => {
            for undefined_struct in smbios_data {
                /*
                    Handle 0x0000, DMI type 0, 20 bytes
                        Header and Data:
                                00 14 00 00 01 02 00 F0 03 03 90 DA CB 7F 00 00
                                00 00 34 01
                        Strings:
                                41 6D 65 72 69 63 61 6E 20 4D 65 67 61 74 72 65
                                6E 64 73 20 49 6E 63 2E 00
                                "American Megatrends Inc."
                                30 39 30 30 30 38 20 00
                                "090008 "
                                31 32 2F 30 37 2F 32 30 31 38 00
                                "12/07/2018"
                */
                println!(
                    "Handle {:#06X}, DMI type {}, {} bytes",
                    *undefined_struct.header.handle(),
                    undefined_struct.header.struct_type(),
                    undefined_struct.fields.len()
                );
                print!("\tHeader and Data:");
                for item in undefined_struct.fields.iter().enumerate() {
                    if item.0 % 16 == 0 {
                        println!();
                        print!("\t\t");
                    }
                    print!("{:02X} ", item.1);
                }
                println!();
                print!("\tStrings:");
                for string_item in undefined_struct.strings.iter() {
                    for item in string_item.iter().enumerate() {
                        if item.0 % 16 == 0 {
                            println!();
                            print!("\t\t");
                        }
                        print!("{:02X} ", item.1);
                    }
                    println!();
                    let as_string: String = string_item.iter().map(|x| *x as char).collect();
                    print!("\t\t\"{}\"", as_string);
                }
                println!();
            }
        }
        (None, None, None, None, None, false, true) => {
            for i in Keyword::into_enum_iter() {
                println!("{}", &i);
            }
        }
        _ => println!("{:#X?}", smbios_data),
    }

    Ok(())
}
