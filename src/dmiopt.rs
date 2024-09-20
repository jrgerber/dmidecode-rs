use crate::default_out::dump_undefined_struct;
use crate::error::BiosParseError;
use enum_iterator::Sequence;
use smbioslib::*;
use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
    path::PathBuf,
    str::FromStr,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "dmidecode-rs",
    about = "DMI Table Decoder, Rust Edition ⛭",
    author = "Jeffrey R. Gerber, Juan Zuluaga"
)]
pub struct Opt {
    /// Less verbose output
    // short and long flags (-q, --quiet) will be deduced from the field's name
    #[structopt(short, long)]
    pub quiet: bool,

    /// Read memory from device FILE (default: /dev/mem)
    #[structopt(short, long, name = "FILE", parse(from_os_str))]
    pub dev_mem: Option<PathBuf>,

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
    pub keyword: Option<Keyword>,

    /// Read the DMI data from a binary file
    #[structopt(long = "from-dump", parse(from_os_str))]
    pub input: Option<PathBuf>,

    /// Dump the DMI data to a binary file
    #[structopt(long = "dump-bin", parse(from_os_str))]
    pub output: Option<PathBuf>,

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
    pub bios_types: Option<Vec<BiosType>>,

    /// Only display the entry whose handle matches `handle`. `handle` is a
    /// 16-bit integer in either a decimal or a hexadecimal (0xN) form.
    #[structopt(short = "H", long = "handle")]
    pub handle: Option<Handle>,

    /// Do not decode the entries, dump their contents as hexadecimal
    /// instead.
    ///
    /// Note that this is still a text output, no binary data
    /// will be thrown upon you. The strings attached to each entry are
    /// displayed as both hexadecimal and ASCII. This option is mainly
    /// useful for debugging.
    #[structopt(short = "u", long = "dump")]
    pub undefined_dump: bool,

    /// Only display the value of the OEM string number N. The first OEM string
    /// has number 1. With special value "count", return the number of OEM
    /// strings instead.
    #[structopt(long = "oem-string")]
    pub oem_string: Option<String>,

    /// List supported DMI string
    #[structopt(short, long)]
    pub list: bool,

    /// Do not attempt to read DMI data from sysfs files.
    ///
    /// This is mainly useful for debugging.
    #[structopt(long = "no-sysfs")]
    pub no_sysfs: bool,

    /// Display output in JSON pretty print format.
    #[structopt(long)]
    pub json_pretty: bool,

    /// Display output in JSON compact format.
    #[structopt(short, long)]
    pub json: bool,
}

impl Opt {
    #[allow(unused)]
    pub fn has_no_args(&self) -> bool {
        self.quiet
            && self.dev_mem.is_none()
            && self.keyword.is_none()
            && self.input.is_none()
            && self.output.is_none()
            && self.bios_types.is_none()
            && self.handle.is_none()
            && self.oem_string.is_none()
            && !self.no_sysfs
            && !self.undefined_dump
            && !self.list
            && !self.json_pretty
            && !self.json
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BiosType {
    Bios,
    System,
    Baseboard,
    Chassis,
    Processor,
    Memory,
    Cache,
    Connector,
    Slot,
    Numeric(u8),
}

impl FromStr for BiosType {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bios" => Ok(BiosType::Bios),
            "system" => Ok(BiosType::System),
            "baseboard" => Ok(BiosType::Baseboard),
            "chassis" => Ok(BiosType::Chassis),
            "processor" => Ok(BiosType::Processor),
            "memory" => Ok(BiosType::Memory),
            "cache" => Ok(BiosType::Cache),
            "connector" => Ok(BiosType::Connector),
            "slot" => Ok(BiosType::Slot),
            _ => Ok(BiosType::Numeric(u8::from_str(s)?)),
        }
    }
}

/*
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
*/

impl IntoIterator for BiosType {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            BiosType::Bios => vec![0, 13].into_iter(),
            BiosType::System => vec![1, 12, 15, 23, 32].into_iter(),
            BiosType::Baseboard => vec![2, 10, 41].into_iter(),
            BiosType::Chassis => vec![3].into_iter(),
            BiosType::Processor => vec![4].into_iter(),
            BiosType::Memory => vec![5, 6, 16, 17].into_iter(),
            BiosType::Cache => vec![7].into_iter(),
            BiosType::Connector => vec![8].into_iter(),
            BiosType::Slot => vec![9].into_iter(),
            BiosType::Numeric(number) => vec![number].into_iter(),
        }
    }
}

impl BiosType {
    // We could make this return something, or, could create a type as a collection containing Vec<BiosType> and
    // then implement methods for that type to perform more advanced I/O via state.
    // More than likely the style of output will be desirable to change (verbose, debug, JSON, etc).
    #[allow(unused)]
    pub fn parse_and_display(types: &[BiosType], data: &SMBiosData, quiet: bool) {
        let unique_types: HashSet<u8> = types
            .iter()
            .flat_map(|bios_type| bios_type.into_iter())
            .collect();

        let mut first = true;
        for undefined_struct in data.iter().filter(|undefined_struct| {
            unique_types.contains(&undefined_struct.header.struct_type())
        }) {
            match first {
                true => first = false,
                false => println!(),
            }
            dump_undefined_struct(&undefined_struct, data.version, quiet);
        }
        println!();
    }
}

#[derive(Debug, StructOpt, Sequence)]
pub enum Keyword {
    BiosVendor,
    BiosVersion,
    BiosReleaseDate,
    BiosRevision,
    FirmwareRevision,
    SystemManufacturer,
    SystemProductName,
    SystemVersion,
    SystemSerialNumber,
    SystemUuid,
    SystemSkuNumber,
    SystemFamily,
    BaseboardManufacturer,
    BaseboardProductName,
    BaseboardVersion,
    BaseboardSerialNumber,
    BaseboardAssetTag,
    ChassisManufacturer,
    ChassisType,
    ChassisVersion,
    ChassisSerialNumber,
    ChassisAssetTag,
    ProcessorFamily,
    ProcessorManufacturer,
    ProcessorVersion,
    ProcessorFrequency,
}

const BIOS_VENDOR: &'static str = "bios-vendor";
const BIOS_VERSION: &'static str = "bios-version";
const BIOS_RELEASE_DATE: &'static str = "bios-release-date";
const BIOS_REVISION: &'static str = "bios-revision";
const FIRMWARE_REVISION: &'static str = "firmware-revision";
const SYSTEM_MANUFACTURER: &'static str = "system-manufacturer";
const SYSTEM_PRODUCT_NAME: &'static str = "system-product-name";
const SYSTEM_VERSION: &'static str = "system-version";
const SYSTEM_SERIAL_NUMBER: &'static str = "system-serial-number";
const SYSTEM_UUID: &'static str = "system-uuid";
const SYSTEM_SKU_NUMBER: &'static str = "system-sku-number";
const SYSTEM_FAMILY: &'static str = "system-family";
const BASEBOARD_MANUFACTURER: &'static str = "baseboard-manufacturer";
const BASEBOARD_PRODUCT_NAME: &'static str = "baseboard-product-name";
const BASEBOARD_VERSION: &'static str = "baseboard-version";
const BASEBOARD_SERIAL_NUMBER: &'static str = "baseboard-serial-number";
const BASEBOARD_ASSET_TAG: &'static str = "baseboard-asset-tag";
const CHASSIS_MANUFACTURER: &'static str = "chassis-manufacturer";
const CHASSIS_TYPE: &'static str = "chassis-type";
const CHASSIS_VERSION: &'static str = "chassis-version";
const CHASSIS_SERIAL_NUMBER: &'static str = "chassis-serial-number";
const CHASSIS_ASSET_TAG: &'static str = "chassis-asset-tag";
const PROCESSOR_FAMILY: &'static str = "processor-family";
const PROCESSOR_MANUFACTURER: &'static str = "processor-manufacturer";
const PROCESSOR_VERSION: &'static str = "processor-version";
const PROCESSOR_FREQUENCY: &'static str = "processor-frequency";

impl FromStr for Keyword {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            BIOS_VENDOR => Ok(Keyword::BiosVendor),
            BIOS_VERSION => Ok(Keyword::BiosVersion),
            BIOS_RELEASE_DATE => Ok(Keyword::BiosReleaseDate),
            BIOS_REVISION => Ok(Keyword::BiosRevision),
            FIRMWARE_REVISION => Ok(Keyword::FirmwareRevision),
            SYSTEM_MANUFACTURER => Ok(Keyword::SystemManufacturer),
            SYSTEM_PRODUCT_NAME => Ok(Keyword::SystemProductName),
            SYSTEM_VERSION => Ok(Keyword::SystemVersion),
            SYSTEM_SERIAL_NUMBER => Ok(Keyword::SystemSerialNumber),
            SYSTEM_UUID => Ok(Keyword::SystemUuid),
            SYSTEM_SKU_NUMBER => Ok(Keyword::SystemSkuNumber),
            SYSTEM_FAMILY => Ok(Keyword::SystemFamily),
            BASEBOARD_MANUFACTURER => Ok(Keyword::BaseboardManufacturer),
            BASEBOARD_PRODUCT_NAME => Ok(Keyword::BaseboardProductName),
            BASEBOARD_VERSION => Ok(Keyword::BaseboardVersion),
            BASEBOARD_SERIAL_NUMBER => Ok(Keyword::BaseboardSerialNumber),
            BASEBOARD_ASSET_TAG => Ok(Keyword::BaseboardAssetTag),
            CHASSIS_MANUFACTURER => Ok(Keyword::ChassisManufacturer),
            CHASSIS_TYPE => Ok(Keyword::ChassisType),
            CHASSIS_VERSION => Ok(Keyword::ChassisVersion),
            CHASSIS_SERIAL_NUMBER => Ok(Keyword::ChassisSerialNumber),
            CHASSIS_ASSET_TAG => Ok(Keyword::ChassisAssetTag),
            PROCESSOR_FAMILY => Ok(Keyword::ProcessorFamily),
            PROCESSOR_MANUFACTURER => Ok(Keyword::ProcessorManufacturer),
            PROCESSOR_VERSION => Ok(Keyword::ProcessorVersion),
            PROCESSOR_FREQUENCY => Ok(Keyword::ProcessorFrequency),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, s)),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::BiosVendor => write!(f, "{}", BIOS_VENDOR),
            Keyword::BiosVersion => write!(f, "{}", BIOS_VERSION),
            Keyword::BiosReleaseDate => write!(f, "{}", BIOS_RELEASE_DATE),
            Keyword::BiosRevision => write!(f, "{}", BIOS_REVISION),
            Keyword::FirmwareRevision => write!(f, "{}", FIRMWARE_REVISION),
            Keyword::SystemManufacturer => write!(f, "{}", SYSTEM_MANUFACTURER),
            Keyword::SystemProductName => write!(f, "{}", SYSTEM_PRODUCT_NAME),
            Keyword::SystemVersion => write!(f, "{}", SYSTEM_VERSION),
            Keyword::SystemSerialNumber => write!(f, "{}", SYSTEM_SERIAL_NUMBER),
            Keyword::SystemUuid => write!(f, "{}", SYSTEM_UUID),
            Keyword::SystemSkuNumber => write!(f, "{}", SYSTEM_SKU_NUMBER),
            Keyword::SystemFamily => write!(f, "{}", SYSTEM_FAMILY),
            Keyword::BaseboardManufacturer => write!(f, "{}", BASEBOARD_MANUFACTURER),
            Keyword::BaseboardProductName => write!(f, "{}", BASEBOARD_PRODUCT_NAME),
            Keyword::BaseboardVersion => write!(f, "{}", BASEBOARD_VERSION),
            Keyword::BaseboardSerialNumber => write!(f, "{}", BASEBOARD_SERIAL_NUMBER),
            Keyword::BaseboardAssetTag => write!(f, "{}", BASEBOARD_ASSET_TAG),
            Keyword::ChassisManufacturer => write!(f, "{}", CHASSIS_MANUFACTURER),
            Keyword::ChassisType => write!(f, "{}", CHASSIS_TYPE),
            Keyword::ChassisVersion => write!(f, "{}", CHASSIS_VERSION),
            Keyword::ChassisSerialNumber => write!(f, "{}", CHASSIS_SERIAL_NUMBER),
            Keyword::ChassisAssetTag => write!(f, "{}", CHASSIS_ASSET_TAG),
            Keyword::ProcessorFamily => write!(f, "{}", PROCESSOR_FAMILY),
            Keyword::ProcessorManufacturer => write!(f, "{}", PROCESSOR_MANUFACTURER),
            Keyword::ProcessorVersion => write!(f, "{}", PROCESSOR_VERSION),
            Keyword::ProcessorFrequency => write!(f, "{}", PROCESSOR_FREQUENCY),
        }
    }
}

impl Keyword {
    #[allow(unused)]
    pub fn parse(&self, data: &SMBiosData) -> Result<String, BiosParseError> {
        // Note: Some structures are single instance and some can be multi-instance.
        // Therefore, multiple strings may be returned in some cases.
        //
        // BIOS Information (type 0): single
        // System Information (type 1): single
        // Baseboard Information (type 2): multi
        // Chassis Information (type 3): multi
        // Processor Information (type 4): multi

        match self {
            Keyword::BiosVendor => data
                .find_map(|bios_info: SMBiosInformation<'_>| bios_info.vendor().to_utf8_lossy())
                .ok_or(BiosParseError::BiosVendorNotFound),
            Keyword::BiosVersion => data
                .find_map(|bios_info: SMBiosInformation<'_>| bios_info.version().to_utf8_lossy())
                .ok_or(BiosParseError::BiosVersionNotFound),
            Keyword::BiosReleaseDate => data
                .find_map(|bios_info: SMBiosInformation<'_>| {
                    bios_info.release_date().to_utf8_lossy()
                })
                .ok_or(BiosParseError::BiosReleaseDateNotFound),
            Keyword::BiosRevision => data
                .find_map(|bios_info: SMBiosInformation<'_>| {
                    match (
                        bios_info.system_bios_major_release(),
                        bios_info.system_bios_minor_release(),
                    ) {
                        (Some(major), Some(minor)) => Some(format!("{}.{}", major, minor)),
                        _ => None,
                    }
                })
                .ok_or(BiosParseError::BiosRevisionNotFound),
            Keyword::FirmwareRevision => data
                .find_map(|bios_info: SMBiosInformation<'_>| {
                    match (
                        bios_info.e_c_firmware_major_release(),
                        bios_info.e_c_firmware_minor_release(),
                    ) {
                        (Some(major), Some(minor)) => Some(format!("{}.{}", major, minor)),
                        _ => None,
                    }
                })
                .ok_or(BiosParseError::FirmwareRevisionNotFound),
            Keyword::SystemManufacturer => data
                .find_map(|system_info: SMBiosSystemInformation<'_>| {
                    system_info.manufacturer().to_utf8_lossy()
                })
                .ok_or(BiosParseError::SystemManufacturerNotFound),
            Keyword::SystemProductName => data
                .find_map(|system_info: SMBiosSystemInformation<'_>| {
                    system_info.product_name().to_utf8_lossy()
                })
                .ok_or(BiosParseError::SystemProductNameNotFound),
            Keyword::SystemVersion => data
                .find_map(|system_info: SMBiosSystemInformation<'_>| {
                    system_info.version().to_utf8_lossy()
                })
                .ok_or(BiosParseError::SystemVersionNotFound),
            Keyword::SystemSerialNumber => data
                .find_map(|system_info: SMBiosSystemInformation<'_>| {
                    system_info.serial_number().to_utf8_lossy()
                })
                .ok_or(BiosParseError::SystemSerialNumberNotFound),
            Keyword::SystemUuid => {
                match data.find_map(|system_info: SMBiosSystemInformation<'_>| system_info.uuid()) {
                    // SystemUuidData is an enum that can be broken down further if desired
                    Some(uuid) => Ok(format!("{}", uuid)),
                    None => Err(BiosParseError::SystemUuidNotFound),
                }
            }
            Keyword::SystemSkuNumber => data
                .find_map(|system_info: SMBiosSystemInformation<'_>| {
                    system_info.sku_number().to_utf8_lossy()
                })
                .ok_or(BiosParseError::SystemSkuNumberNotFound),
            Keyword::SystemFamily => data
                .find_map(|system_info: SMBiosSystemInformation<'_>| {
                    system_info.family().to_utf8_lossy()
                })
                .ok_or(BiosParseError::SystemFamilyNotFound),
            Keyword::BaseboardManufacturer => data
                .map(|baseboard_info: SMBiosBaseboardInformation<'_>| {
                    baseboard_info.manufacturer().to_utf8_lossy()
                })
                .try_fold(String::new(), |mut acc, item| match item {
                    Some(val) => Some({
                        if !acc.is_empty() {
                            acc.push_str("\n");
                        };
                        acc.push_str(&val);
                        acc
                    }),
                    None => None,
                })
                .ok_or(BiosParseError::BaseboardManufacturerNotFound),
            Keyword::BaseboardProductName => data
                .map(|baseboard_info: SMBiosBaseboardInformation<'_>| {
                    baseboard_info.product().to_utf8_lossy()
                })
                .try_fold(String::new(), |mut acc, item| match item {
                    Some(val) => Some({
                        if !acc.is_empty() {
                            acc.push_str("\n");
                        };
                        acc.push_str(&val);
                        acc
                    }),
                    None => None,
                })
                .ok_or(BiosParseError::BaseboardProductNameNotFound),
            Keyword::BaseboardVersion => data
                .map(|baseboard_info: SMBiosBaseboardInformation<'_>| {
                    baseboard_info.version().to_utf8_lossy()
                })
                .try_fold(String::new(), |mut acc, item| match item {
                    Some(val) => Some({
                        if !acc.is_empty() {
                            acc.push_str("\n");
                        };
                        acc.push_str(&val);
                        acc
                    }),
                    None => None,
                })
                .ok_or(BiosParseError::BaseboardVersionNotFound),
            Keyword::BaseboardSerialNumber => data
                .map(|baseboard_info: SMBiosBaseboardInformation<'_>| {
                    baseboard_info.serial_number().to_utf8_lossy()
                })
                .try_fold(String::new(), |mut acc, item| match item {
                    Some(val) => Some({
                        if !acc.is_empty() {
                            acc.push_str("\n");
                        };
                        acc.push_str(&val);
                        acc
                    }),
                    None => None,
                })
                .ok_or(BiosParseError::BaseboardSerialNumberNotFound),
            Keyword::BaseboardAssetTag => data
                .map(|baseboard_info: SMBiosBaseboardInformation<'_>| {
                    baseboard_info.asset_tag().to_utf8_lossy()
                })
                .try_fold(String::new(), |mut acc, item| match item {
                    Some(val) => Some({
                        if !acc.is_empty() {
                            acc.push_str("\n");
                        };
                        acc.push_str(&val);
                        acc
                    }),
                    None => None,
                })
                .ok_or(BiosParseError::BaseboardAssetTagNotFound),
            Keyword::ChassisManufacturer => data
                .map(|chassis_info: SMBiosSystemChassisInformation<'_>| {
                    chassis_info.manufacturer().to_utf8_lossy()
                })
                .try_fold(String::new(), |mut acc, item| match item {
                    Some(val) => Some({
                        if !acc.is_empty() {
                            acc.push_str("\n");
                        };
                        acc.push_str(&val);
                        acc
                    }),
                    None => None,
                })
                .ok_or(BiosParseError::ChassisManufacturerNotFound),
            Keyword::ChassisType => data
                .map(|chassis_info: SMBiosSystemChassisInformation<'_>| chassis_info.chassis_type())
                .try_fold(String::new(), |mut acc, item| match item {
                    Some(val) => Some({
                        if !acc.is_empty() {
                            acc.push_str("\n");
                        };
                        acc.push_str(&format!("{}", &val).to_string());
                        acc
                    }),
                    None => None,
                })
                .ok_or(BiosParseError::ChassisTypeNotFound),
            Keyword::ChassisVersion => data
                .map(|chassis_info: SMBiosSystemChassisInformation<'_>| {
                    chassis_info.version().to_utf8_lossy()
                })
                .try_fold(String::new(), |mut acc, item| match item {
                    Some(val) => Some({
                        if !acc.is_empty() {
                            acc.push_str("\n");
                        };
                        acc.push_str(&val);
                        acc
                    }),
                    None => None,
                })
                .ok_or(BiosParseError::ChassisVersionNotFound),
            Keyword::ChassisSerialNumber => data
                .map(|chassis_info: SMBiosSystemChassisInformation<'_>| {
                    chassis_info.serial_number().to_utf8_lossy()
                })
                .try_fold(String::new(), |mut acc, item| match item {
                    Some(val) => Some({
                        if !acc.is_empty() {
                            acc.push_str("\n");
                        };
                        acc.push_str(&val);
                        acc
                    }),
                    None => None,
                })
                .ok_or(BiosParseError::ChassisSerialNumberNotFound),
            Keyword::ChassisAssetTag => data
                .map(|chassis_info: SMBiosSystemChassisInformation<'_>| {
                    chassis_info.asset_tag_number().to_utf8_lossy()
                })
                .try_fold(String::new(), |mut acc, item| match item {
                    Some(val) => Some({
                        if !acc.is_empty() {
                            acc.push_str("\n");
                        };
                        acc.push_str(&val);
                        acc
                    }),
                    None => None,
                })
                .ok_or(BiosParseError::ChassisAssetTagNotFound),
            Keyword::ProcessorFamily => data
                .map(|processor_info: SMBiosProcessorInformation<'_>| {
                    if let Some(family) = processor_info.processor_family() {
                        match family.value {
                            ProcessorFamily::SeeProcessorFamily2 => {
                                if let Some(family2) = processor_info.processor_family_2() {
                                    Some(format!("{}", family2))
                                } else {
                                    None
                                }
                            }
                            _ => Some(format!("{}", family)),
                        }
                    } else {
                        None
                    }
                })
                .try_fold(String::new(), |mut acc, item| match item {
                    Some(val) => Some({
                        if !acc.is_empty() {
                            acc.push_str("\n");
                        };
                        acc.push_str(&val);
                        acc
                    }),
                    None => None,
                })
                .ok_or(BiosParseError::ProcessorFamilyNotFound),
            Keyword::ProcessorManufacturer => data
                .map(|processor_info: SMBiosProcessorInformation<'_>| {
                    processor_info.processor_manufacturer().to_utf8_lossy()
                })
                .try_fold(String::new(), |mut acc, item| match item {
                    Some(val) => Some({
                        if !acc.is_empty() {
                            acc.push_str("\n");
                        };
                        acc.push_str(&val);
                        acc
                    }),
                    None => None,
                })
                .ok_or(BiosParseError::ProcessorManufacturerNotFound),
            Keyword::ProcessorVersion => data
                .map(|processor_info: SMBiosProcessorInformation<'_>| {
                    processor_info.processor_version().to_utf8_lossy()
                })
                .try_fold(String::new(), |mut acc, item| match item {
                    Some(val) => Some({
                        if !acc.is_empty() {
                            acc.push_str("\n");
                        };
                        acc.push_str(&val);
                        acc
                    }),
                    None => None,
                })
                .ok_or(BiosParseError::ProcessorVersionNotFound),
            Keyword::ProcessorFrequency => data
                .map(|processor_info: SMBiosProcessorInformation<'_>| {
                    processor_info.current_speed()
                })
                .try_fold(String::new(), |mut acc, item| match item {
                    Some(val) => Some({
                        if !acc.is_empty() {
                            acc.push_str("\n");
                        };
                        let output = match &val {
                            ProcessorSpeed::Unknown => String::from("Unknown"),
                            ProcessorSpeed::MHz(frequency) => format!("{} MHz", frequency),
                        };
                        acc.push_str(output.as_str());
                        acc
                    }),
                    None => None,
                })
                .ok_or(BiosParseError::ProcessorFrequencyNotFound),
        }
    }
}

#[test]
fn test_enum_display_exist_in_opt_string_keyword() -> Result<(), Box<dyn std::error::Error>> {
    use enum_iterator::all;

    let keywords = all::<Keyword>().collect::<Vec<_>>();
    for keyword in keywords {
        let kstr = format!("{}", &keyword);
        Keyword::from_str(&kstr)?;
    }
    Ok(())
}

#[test]
fn test_keyword_invalid_error_expected() {
    let result = Keyword::from_str("invalid");
    assert!(result.is_err());
    let got = result.unwrap_err();
    let want = std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid");
    assert_eq!(want.to_string(), got.to_string());
}
