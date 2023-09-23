//! # dmidecode command-line tool executable

#![warn(missing_docs)]
#![deny(rust_2018_idioms)]

#[cfg_attr(any(target_os = "linux", target_os = "freebsd"), path = "unix.rs")]
#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(target_os = "macos", path = "macos.rs")]
mod platform;

mod default_out;
mod dmifn;
mod dmiopt;
mod error;

use default_out::default_dump;
use dmiopt::{BiosType, Keyword, Opt};
use enum_iterator::all;
use smbioslib::*;
use std::fmt::Write;
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

/// print dmidecode version
pub fn print_dmidecode_version() {
    println!("# {} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt: Opt = Opt::from_args();

    // Select an input source, file or device.
    let smbios_data = if let Some(path) = opt.input.as_ref() {
        let mut output = String::new();

        writeln!(
            &mut output,
            "Getting SMBIOS data from {}.",
            path.to_string_lossy()
        )
        .unwrap();

        let data = load_smbios_data_from_file(path)?;

        (data, output)
    } else {
        platform::table_load(&opt)?
    };

    // Mutually exclusive output options (only one tuple element is Some()).
    match (
        opt.keyword.as_ref(),
        opt.output.as_ref(),
        opt.bios_types.as_ref(),
        opt.handle.as_ref(),
        opt.oem_string.as_ref(),
        opt.undefined_dump,
        opt.list,
        opt.json_pretty,
        opt.json,
    ) {
        // opt.keyword, -s, --string KEYWORD   Only display the value of the given DMI string
        (Some(keyword), None, None, None, None, false, false, false, false) => {
            let output = keyword.parse(&smbios_data.0)?;
            println!("{}", output);
        }
        // opt.output, --dump-bin FILE    Dump the DMI data to a binary file
        (None, Some(output), None, None, None, false, false, false, false) => {
            print_dmidecode_version();
            // TODO: create stdout output.  dump_raw() and raw_smbios_from_device() do not output.
            dump_raw(raw_smbios_from_device()?, &output.as_path())?
        }
        // opt.bios_types, -t, --type TYPE        Only display the entries of given type
        (None, None, Some(bios_types), None, None, false, false, false, false) => {
            print_dmidecode_version();
            println!("{}", smbios_data.1);
            BiosType::parse_and_display(bios_types, &smbios_data.0, opt.quiet);
        }
        // opt.handle, -H, --handle HANDLE    Only display the entry of given handle
        (None, None, None, Some(handle), None, false, false, false, false) => {
            print_dmidecode_version();
            println!("{}", smbios_data.1);
            let found_struct = smbios_data
                .0
                .find_by_handle(&handle)
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Handle not found: {}", *handle),
                ))?;
            println!("{:#X?}", &found_struct.defined_struct())
        }
        // opt.oem_string, --oem-string N     Only display the value of the given OEM string
        (None, None, None, None, Some(oem), false, false, false, false) => {
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
            match smbios_data.0.first::<SMBiosOemStrings<'_>>() {
                Some(v) => {
                    match v.oem_strings().get_string(index).to_utf8_lossy() {
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
                None => {
                    if index != 0 {
                        invalid_num(oem.as_str())?
                    } else {
                        // When no structure exists and --oem-string is "count", return "0"
                        println!("0")
                    }
                }
            }
        }
        // opt.undefined_dump, -u, --dump             Do not decode the entries
        (None, None, None, None, None, true, false, false, false) => {
            print_dmidecode_version();
            println!("{}", smbios_data.1);
            for undefined_struct in smbios_data.0 {
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
                println!();
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
                    // chain() adds a terminating \0 for parity with the original dmidecode
                    for item in string_item.iter().chain([0].iter()).enumerate() {
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
        // opt.list, -l, --list        List supported DMI string
        (None, None, None, None, None, false, true, false, false) => {
            let keywords = all::<Keyword>().collect::<Vec<_>>();
            for keyword in keywords {
                let kstr = format!("{}", &keyword);
                println!("{}", kstr);
            }
        }
        // opt.json, -j, --json        Display output in JSON pretty print format.
        (None, None, None, None, None, false, false, true, false) => {
            if let Ok(output) = serde_json::to_string_pretty(&smbios_data.0) {
                println!("{}", output)
            }
        }
        // opt.json_compat, --json-compact        Display output in JSON compact format.
        (None, None, None, None, None, false, false, false, true) => {
            if let Ok(output) = serde_json::to_string(&smbios_data.0) {
                println!("{}", output)
            }
        }
        _ => {
            print_dmidecode_version();
            let smbios_data = platform::table_load(&opt)?;
            print!("{}", smbios_data.1);
            default_dump(&smbios_data.0, opt.quiet);
        }
    }

    Ok(())
}
