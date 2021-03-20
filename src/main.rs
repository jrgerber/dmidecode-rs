//! # dmidecode command-line tool executable

#![warn(missing_docs)]
#![deny(rust_2018_idioms)]

mod dmiopt;
mod error;

use dmiopt::{BiosType, Keyword};
use enum_iterator::IntoEnumIterator;
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
#[structopt(
    name = "dmidecode-rs",
    about = "DMI Table Decoder, Rust Edition ⛭",
    author = "Jeffrey R. Gerber, Juan Zuluaga"
)]
struct Opt {
    /// Less verbose output
    // short and long flags (-q, --quiet) will be deduced from the field's name
    #[structopt(short, long)]
    quiet: bool,

    /// Only display the value of the DMI string identified by `keyword`.
    ///
    /// `keyword` must be a keyword from the following list: bios-vendor,
    /// bios-version, bios-release-date, system-manufacturer, system-
    /// product-name, system-version, system-serial-number, system-uuid,
    /// system-family, baseboard-manufacturer, baseboard-product-name,
    /// baseboard-version, baseboard-serial-number, baseboard-asset-tag,
    /// chassis-manufacturer, chassis-type, chassis-version, chassis-
    /// serial-number, chassis-asset-tag, processor-family, processor-
    /// manufacturer, processor-version, processor-frequency.  Each
    /// keyword corresponds to a given DMI type and a given offset
    /// within this entry type.  Not all strings may be meaningful or
    /// even defined on all systems. Some keywords may return more than
    /// one result on some systems (e.g.  processor-version on a multi-
    /// processor system).  If KEYWORD is not provided or not valid, a
    /// list of all valid keywords is printed and dmidecode exits with
    /// an error.  This option cannot be used more than once.
    ///
    /// Note: on Linux, most of these strings can alternatively be read
    /// directly from sysfs, typically from files under
    /// /sys/devices/virtual/dmi/id.  Most of these files are even
    /// readable by regular users.    
    #[structopt(short = "s", long = "string")]
    keyword: Option<Keyword>,

    /// Read the DMI data from a binary file
    #[structopt(long = "from-dump", parse(from_os_str))]
    input: Option<PathBuf>,

    /// Dump the DMI data to a binary file
    #[structopt(long = "dump-bin", parse(from_os_str))]
    output: Option<PathBuf>,

    /// Only display the entries of given type
    ///
    /// Supply one or more keywords, one or more type values,
    /// or a combination of the two.
    ///
    ///    Keyword     Types
    ///    ------------------------------
    ///    bios        0, 13
    ///    system      1, 12, 15, 23, 32
    ///    baseboard   2, 10, 41
    ///    chassis     3
    ///    processor   4
    ///    memory      5, 6, 16, 17
    ///    cache       7
    ///    connector   8
    ///    slot        9
    #[structopt(short = "t", long = "type", verbatim_doc_comment)]
    bios_types: Option<Vec<BiosType>>,

    /// Only display the entry whose handle matches `handle`. `handle` is a
    /// 16-bit integer in either a decimal or a hex (0xN) radix.
    #[structopt(short = "H", long = "handle")]
    handle: Option<Handle>,

    /// List supported DMI string
    #[structopt(short, long)]
    list: bool,
}

impl Opt {
    fn has_no_args(&self) -> bool {
        self.keyword.is_none()
            && self.input.is_none()
            && self.output.is_none()
            && self.bios_types.is_none()
            && self.handle.is_none()
            && !self.list
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt: Opt = Opt::from_args();

    if opt.has_no_args() {
        println!("{:#X?}", table_load_from_device()?);
        return Ok(());
    }

    // Select an input source, file or device.
    let smbios_data = if let Some(input) = opt.input {
        let filename = input.to_str().ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Invalid filename {:?}", input),
        ))?;
        load_smbios_data_from_file(filename)?
    } else {
        table_load_from_device()?
    };

    // Mutually exclusive output options (only one tuple element is Some()).
    match (opt.keyword, opt.output, opt.bios_types, opt.handle, opt.list) {
        (Some(keyword), None, None, None, false) => {
            let output = keyword.parse(&smbios_data)?;
            println!("{}", output);
        }
        (None, Some(output), None, None, false) => {
            let filename = output.to_str().ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid filename {:?}", output),
            ))?;
            dump_raw(raw_smbios_from_device()?, filename)?
        }
        (None, None, Some(bios_types), None, false) => {
            BiosType::parse_and_display(bios_types, &smbios_data);
        }
        (None, None, None, Some(handle), false) => {
            let found_struct = smbios_data
                .find_by_handle(&handle)
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Handle not found: {}", *handle),
                ))?;
            println!("{:#X?}", &found_struct.defined_struct())
        }
        (None, None, None, None, true) => {
            for i in Keyword::into_enum_iter() {
                println!("{}", &i);
            }
        }
        _ => println!("{:#X?}", smbios_data),
    }

    Ok(())
}
