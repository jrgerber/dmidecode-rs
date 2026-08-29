# dmidecode-rs

![dmidecode-rs_ci](https://github.com/jrgerber/dmidecode-rs/actions/workflows/dmidecode_ci.yml/badge.svg)
![LOC](https://tokei.rs/b1/github/jrgerber/dmidecode-rs?category=code)

Rust implementation of the classic dmidecode utility for reporting SMBIOS/DMI data in a human-readable format.

## Features

- Decode SMBIOS tables on supported platforms
- Read data from sysfs, memory devices, or dump files
- Filter output by BIOS type, handle, keyword, or OEM string
- Output plain text, JSON, or raw hexadecimal dump
- Supports Linux, FreeBSD, macOS, and Windows targets

## Installation

Build from source:

```bash
git clone https://github.com/jrgerber/dmidecode-rs.git
cd dmidecode-rs
cargo build --release
./target/release/dmidecode --help
```

Or install directly with Cargo:

```bash
cargo install dmidecode-rs
```

## Usage

```bash
dmidecode --help
```

Common examples:

```bash
# Show all SMBIOS data
sudo dmidecode

# Show only memory-related entries
sudo dmidecode --type memory

# Read a specific DMI string
sudo dmidecode --string bios-version

# Print JSON output
sudo dmidecode --json

# Dump raw SMBIOS data to a file
sudo dmidecode --dump-bin raw.bin

# Read SMBIOS data from a dump file
sudo dmidecode --from-dump raw.bin

# Disable sysfs usage for debugging
sudo dmidecode --no-sysfs
```

## Supported DMI string keywords

The `--string` option supports these keywords:

```text
bios-vendor
bios-version
bios-release-date
bios-revision
firmware-revision
system-manufacturer
system-product-name
system-version
system-serial-number
system-uuid
system-sku-number
system-family
baseboard-manufacturer
baseboard-product-name
baseboard-version
baseboard-serial-number
baseboard-asset-tag
chassis-manufacturer
chassis-type
chassis-version
chassis-serial-number
chassis-asset-tag
processor-family
processor-manufacturer
processor-version
processor-frequency
```

## Command help

```text
dmidecode-rs 0.2.4
Jeffrey R. Gerber, Juan Zuluaga
DMI Table Decoder, Rust Edition ⛭

USAGE:
    dmidecode [FLAGS] [OPTIONS]

FLAGS:
    -h, --help
            Prints help information

    -j, --json
            Display output in JSON compact format

        --json-pretty
            Display output in JSON pretty print format

    -l, --list
            List supported DMI string

        --no-sysfs
            Do not attempt to read DMI data from sysfs files.

    -q, --quiet
            Less verbose output

    -u, --dump
            Do not decode the entries, dump their contents as hexadecimal instead.

    -V, --version
            Prints version information

OPTIONS:
    -d, --dev-mem <FILE>
            Read memory from device FILE (default: /dev/mem)

    -t, --type <bios-types>...
            Only display the entries of given type

    -H, --handle <handle>
            Only display the entry whose handle matches handle

        --from-dump <input>
            Read the DMI data from a binary file

    -s, --string <keyword>
            Only display the value of the DMI string identified by keyword

        --oem-string <oem-string>
            Only display the value of the OEM string number N

        --dump-bin <output>
            Dump the DMI data to a binary file
```

## Notes

This project follows the familiar dmidecode command-line behavior while providing a Rust implementation. It is intended for hardware inspection and debugging, and it can be useful when you want a portable tool with JSON output or script-friendly filtering.

For the latest changes and issues, see the GitHub repository: https://github.com/jrgerber/dmidecode-rs

