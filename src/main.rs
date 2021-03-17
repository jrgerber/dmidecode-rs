//! # dmidecode command-line tool executable

#![warn(missing_docs)]
#![deny(rust_2018_idioms)]

mod error;
mod dmiopt;

use crate::dmiopt::opt_string_keyword;
use smbioslib::*;


fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let help_option = "h";
    let file_option = "f";
    let output_option = "o";
    let string_option = "s";

    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let mut opts = getopts::Options::new();
    opts.optflag(help_option, "", "Print help menu");
    opts.optopt(file_option, "", "Read smbios table from file", "FILE");
    opts.optopt(output_option, "", "Dump smbios table to a file", "FILE");
    opts.optopt(
        string_option,
        "",
        "Only display the value of the DMI string identified by KEYWORD.",
        "KEYWORD",
    );

    let matches = opts.parse(&args[1..])?;
    if matches.opt_present(help_option) {
        print_usage(&program, opts);
        return Ok(());
    }
    if !matches.opt_present(file_option)
        && !matches.opt_present(output_option)
        && !matches.opt_present(string_option)
    {
        println!("table_data: {:#?}", table_load_from_device()?);
        return Ok(());
    }

    match matches.opt_str(file_option) {
        Some(filename) => {
            println!(
                "Load table from file: {} \n{:#?}",
                &filename,
                load_smbios_data_from_file(&filename)?
            );
        }
        None => (),
    }

    match matches.opt_str(output_option) {
        Some(filename) => {
            dump_raw(raw_smbios_from_device()?, &filename)?;
        }
        None => (),
    }

    match matches.opt_str(string_option) {
        Some(keyword) => {
            let smbios_data = table_load_from_device()?;
            let output = opt_string_keyword(keyword, &smbios_data)?;
            println!("{}", output);
        }
        None => (),
    }

    Ok(())
}
