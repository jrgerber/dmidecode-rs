# dmidecode-rs
![Linux](https://github.com/jrgerber/dmidecode-rs/actions/workflows/linux.yml/badge.svg)
![Windows](https://github.com/jrgerber/dmidecode-rs/actions/workflows/windows.yml/badge.svg)
![MacOS](https://github.com/jrgerber/dmidecode-rs/actions/workflows/macos.yml/badge.svg)
![LOC](https://tokei.rs/b1/github/jrgerber/dmidecode-rs?category=code)

dmidecode command written in Rust

# Help
```
dmidecode-rs 0.1.0
Jeffrey R. Gerber, Juan Zuluaga
DMI Table Decoder, Rust Edition ⛭

USAGE:
    dmidecode.exe [FLAGS] [OPTIONS]

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

            This is mainly useful for debugging.
    -q, --quiet
            Less verbose output

    -u, --dump
            Do not decode the entries, dump their contents as hexadecimal instead.

            Note that this is still a text output, no binary data will be thrown upon you. The strings attached to each
            entry are displayed as both hexadecimal and ASCII. This option is mainly useful for debugging.
    -V, --version
            Prints version information


OPTIONS:
    -d, --dev-mem <FILE>
            Read memory from device FILE (default: /dev/mem)

    -t, --type <bios-types>...
            Only display the entries of given type

            Supply one or more keywords, one or more type values,
            or a combination of the two.

               Keyword     Types
               ------------------------------
               bios        0, 13
               system      1, 12, 15, 23, 32
               baseboard   2, 10, 41
               chassis     3
               processor   4
               memory      5, 6, 16, 17
               cache       7
               connector   8
               slot        9
    -H, --handle <handle>
            Only display the entry whose handle matches `handle`. `handle` is a 16-bit integer in either a decimal or a
            hexadecimal (0xN) form
        --from-dump <input>
            Read the DMI data from a binary file

    -s, --string <keyword>
            Only display the value of the DMI string identified by `keyword`.

            `keyword` must be a keyword from the following list: bios-vendor, bios-version, bios-release-date, system-
            manufacturer, system- product-name, system-version, system-serial-number, system-uuid, system-family,
            baseboard-manufacturer, baseboard-product-name, baseboard-version, baseboard-serial-number, baseboard-asset-
            tag, chassis-manufacturer, chassis-type, chassis-version, chassis- serial-number, chassis-
            asset-tag, processor-family, processor- manufacturer, processor-version, processor-frequency.  Each
            keyword corresponds to a given DMI type and a given offset within this entry type.  Not all strings may be
            meaningful or even defined on all systems. Some keywords may return more than one result on some systems
            (e.g.  processor-version on a multi- processor system).  If KEYWORD is not provided or not valid, a list of
            all valid keywords is printed and dmidecode exits with an error.  This option cannot be used more than once.

            Note: on Linux, most of these strings can alternatively be read directly from sysfs, typically from files
            under /sys/devices/virtual/dmi/id.  Most of these files are even readable by regular users.
        --oem-string <oem-string>
            Only display the value of the OEM string number N. The first OEM string has number 1. With special value
            "count", return the number of OEM strings instead
        --dump-bin <output>
            Dump the DMI data to a binary file
```
