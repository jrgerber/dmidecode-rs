//! # dmidecode command-line tool executable

#![warn(missing_docs)]
#![deny(rust_2018_idioms)]

mod dmiopt;
mod error;

use crate::dmiopt::opt_string_keyword;
use dmiopt::{Keyword, list_keywords};
use smbioslib::*;
use std::path::PathBuf;
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

#[derive(Debug, StructOpt)]
#[structopt(name = "dmidecode", about = "DMI table decoder.")]
struct Opt {
    /// Less verbose output
    // short and long flags (-q, --quiet) will be deduced from the field's name
    #[structopt(short, long)]
    quiet: bool,

    /// Only display the value of the given DMI string
    #[structopt(short = "s", long = "string")]
    keyword: Option<Keyword>,

    /// Read the DMI data from a binary file
    #[structopt(long = "from-dump", parse(from_os_str))]
    input: Option<PathBuf>,

    /// Dump the DMI data to a binary file
    #[structopt(long = "dump-bin", parse(from_os_str))]
    output: Option<PathBuf>,

    /// List supported DMI string
    #[structopt(short, long)]
    list: bool,
}

impl Opt {
    fn has_no_args(&self) -> bool {
        self.keyword.is_none() && self.input.is_none() && self.output.is_none() &&
            !self.list
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt: Opt = Opt::from_args();

    if opt.has_no_args() {
        println!("{:#X?}", table_load_from_device()?);
        return Ok(());
    }

    match opt.keyword {
        Some(keyword) => {
            let smbios_data = table_load_from_device()?;
            let output = opt_string_keyword(keyword, &smbios_data)?;
            println!("{}", output);
        }
        None => (),
    }

    match opt.input {
        Some(input) => {
            let filename = input.to_str().ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid filename {:?}", input),
            ))?;
            println!("{:#X?}", load_smbios_data_from_file(filename)?)
        }
        None => (),
    }

    match opt.output {
        Some(output) => {
            let filename = output.to_str().ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid filename {:?}", output),
            ))?;
            dump_raw(raw_smbios_from_device()?, filename)?
        }
        None => (),
    }

    if opt.list {
        list_keywords()
    }

    Ok(())
}
