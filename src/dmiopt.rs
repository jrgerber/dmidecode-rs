use std::{fmt::{self, Display, Formatter}};
use std::str::FromStr;
use enum_iterator::IntoEnumIterator;

use crate::error::BiosParseError;
use smbioslib::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt, IntoEnumIterator)]
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


impl FromStr for Keyword {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bios-vendor" => Ok(Keyword::BiosVendor),
            "bios-version" => Ok(Keyword::BiosVersion),
            "bios-release-date" => Ok(Keyword::BiosReleaseDate),
            "bios-revision" => Ok(Keyword::BiosRevision),
            "firmware-revision" => Ok(Keyword::FirmwareRevision),
            "system-manufacturer" => Ok(Keyword::SystemManufacturer),
            "system-product-name" => Ok(Keyword::SystemProductName),
            "system-version" => Ok(Keyword::SystemVersion),
            "system-serial-number" => Ok(Keyword::SystemSerialNumber),
            "system-uuid" => Ok(Keyword::SystemUuid),
            "system-sku-number" => Ok(Keyword::SystemSkuNumber),
            "system-family" => Ok(Keyword::SystemFamily),
            "baseboard-manufacturer" => Ok(Keyword::BaseboardManufacturer),
            "baseboard-product-name" => Ok(Keyword::BaseboardProductName),
            "baseboard-version" => Ok(Keyword::BaseboardVersion),
            "baseboard-serial-number" => Ok(Keyword::BaseboardSerialNumber),
            "baseboard-asset-tag" => Ok(Keyword::BaseboardAssetTag),
            "chassis-manufacturer" => Ok(Keyword::ChassisManufacturer),
            "chassis-type" => Ok(Keyword::ChassisType),
            "chassis-version" => Ok(Keyword::ChassisVersion),
            "chassis-serial-number" => Ok(Keyword::ChassisSerialNumber),
            "chassis-asset-tag" => Ok(Keyword::ChassisAssetTag),
            "processor-family" => Ok(Keyword::ProcessorFamily),
            "processor-manufacturer" => Ok(Keyword::ProcessorManufacturer),
            "processor-version" => Ok(Keyword::ProcessorVersion),
            "processor-frequency" => Ok(Keyword::ProcessorFrequency),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, s)),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::BiosVendor => write!(f, "bios-vendor"),
            Keyword::BiosVersion => write!(f, "bios-version"),
            Keyword::BiosReleaseDate => write!(f, "bios-release-date"),
            Keyword::BiosRevision => write!(f, "bios-revision"),
            Keyword::FirmwareRevision => write!(f, "firmware-revision"),
            Keyword::SystemManufacturer => write!(f, "system-manufacturer"),
            Keyword::SystemProductName => write!(f, "system-product-name"),
            Keyword::SystemVersion => write!(f, "system-version"),
            Keyword::SystemSerialNumber => write!(f, "system-serial-number"),
            Keyword::SystemUuid => write!(f, "system-uuid"),
            Keyword::SystemSkuNumber => write!(f, "system-sku-number"),
            Keyword::SystemFamily => write!(f, "system-family"),
            Keyword::BaseboardManufacturer => write!(f, "baseboard-manufacturer"),
            Keyword::BaseboardProductName => write!(f, "baseboard-product-name"),
            Keyword::BaseboardVersion => write!(f, "baseboard-version"),
            Keyword::BaseboardSerialNumber => write!(f, "baseboard-serial-number"),
            Keyword::BaseboardAssetTag => write!(f, "baseboard-asset-tag"),
            Keyword::ChassisManufacturer => write!(f, "chassis-manufacturer"),
            Keyword::ChassisType => write!(f, "chassis-type"),
            Keyword::ChassisVersion => write!(f, "chassis-version"),
            Keyword::ChassisSerialNumber => write!(f, "chassis-serial-number"),
            Keyword::ChassisAssetTag => write!(f, "chassis-asset-tag"),
            Keyword::ProcessorFamily => write!(f, "processor-family"),
            Keyword::ProcessorManufacturer => write!(f, "processor-manufacturer"),
            Keyword::ProcessorVersion => write!(f, "processor-version"),
            Keyword::ProcessorFrequency => write!(f, "processor-frequency"),
        }
    }
}

pub fn opt_string_keyword(keyword: Keyword, data: &SMBiosData) -> Result<String, BiosParseError> {
    match keyword {
        Keyword::BiosVendor => data
            .find_map(|bios_info: SMBiosInformation<'_>| bios_info.vendor())
            .ok_or(BiosParseError::BiosVendorNotFound),
        Keyword::BiosVersion => data
            .find_map(|bios_info: SMBiosInformation<'_>| bios_info.version())
            .ok_or(BiosParseError::BiosVersionNotFound),
        Keyword::BiosReleaseDate => data
            .find_map(|bios_info: SMBiosInformation<'_>| bios_info.release_date())
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
            .find_map(|system_info: SMBiosSystemInformation<'_>| system_info.manufacturer())
            .ok_or(BiosParseError::SystemManufacturerNotFound),
        Keyword::SystemProductName => data
            .find_map(|system_info: SMBiosSystemInformation<'_>| system_info.product_name())
            .ok_or(BiosParseError::SystemProductNameNotFound),
        Keyword::SystemVersion => data
            .find_map(|system_info: SMBiosSystemInformation<'_>| system_info.version())
            .ok_or(BiosParseError::SystemVersionNotFound),
        Keyword::SystemSerialNumber => data
            .find_map(|system_info: SMBiosSystemInformation<'_>| system_info.serial_number())
            .ok_or(BiosParseError::SystemSerialNumberNotFound),
        Keyword::SystemUuid => {
            match data.find_map(|system_info: SMBiosSystemInformation<'_>| system_info.uuid()) {
                // SystemUuidData is an enum that can be broken down further if desired
                Some(uuid) => Ok(format!("{:?}", uuid)),
                None => Err(BiosParseError::SystemUuidNotFound),
            }
        }
        Keyword::SystemSkuNumber => data
            .find_map(|system_info: SMBiosSystemInformation<'_>| system_info.sku_number())
            .ok_or(BiosParseError::SystemSkuNumberNotFound),
        Keyword::SystemFamily => data
            .find_map(|system_info: SMBiosSystemInformation<'_>| system_info.family())
            .ok_or(BiosParseError::SystemFamilyNotFound),
        Keyword::BaseboardManufacturer => data
            .find_map(|baseboard_info: SMBiosBaseboardInformation<'_>| {
                baseboard_info.manufacturer()
            })
            .ok_or(BiosParseError::BaseboardManufacturerNotFound),
        Keyword::BaseboardProductName => data
            .find_map(|baseboard_info: SMBiosBaseboardInformation<'_>| baseboard_info.product())
            .ok_or(BiosParseError::BaseboardProductNameNotFound),
        Keyword::BaseboardVersion => data
            .find_map(|baseboard_info: SMBiosBaseboardInformation<'_>| baseboard_info.version())
            .ok_or(BiosParseError::BaseboardVersionNotFound),
        Keyword::BaseboardSerialNumber => data
            .find_map(|baseboard_info: SMBiosBaseboardInformation<'_>| {
                baseboard_info.serial_number()
            })
            .ok_or(BiosParseError::BaseboardSerialNumberNotFound),
        Keyword::BaseboardAssetTag => data
            .find_map(|baseboard_info: SMBiosBaseboardInformation<'_>| baseboard_info.asset_tag())
            .ok_or(BiosParseError::BaseboardAssetTagNotFound),
        Keyword::ChassisManufacturer => data
            .find_map(|chassis_info: SMBiosSystemChassisInformation<'_>| {
                chassis_info.manufacturer()
            })
            .ok_or(BiosParseError::ChassisManufacturerNotFound),
        Keyword::ChassisType => {
            match data.find_map(|chassis_info: SMBiosSystemChassisInformation<'_>| {
                chassis_info.chassis_type()
            }) {
                Some(chassis_type) => Ok(format!("{:?}", chassis_type)),
                None => Err(BiosParseError::ChassisTypeNotFound),
            }
        }
        Keyword::ChassisVersion => data
            .find_map(|chassis_info: SMBiosSystemChassisInformation<'_>| chassis_info.version())
            .ok_or(BiosParseError::ChassisVersionNotFound),
        Keyword::ChassisSerialNumber => data
            .find_map(|chassis_info: SMBiosSystemChassisInformation<'_>| {
                chassis_info.serial_number()
            })
            .ok_or(BiosParseError::ChassisSerialNumberNotFound),
        Keyword::ChassisAssetTag => data
            .find_map(|chassis_info: SMBiosSystemChassisInformation<'_>| {
                chassis_info.asset_tag_number()
            })
            .ok_or(BiosParseError::ChassisAssetTagNotFound),
        Keyword::ProcessorFamily => match data.first::<SMBiosProcessorInformation<'_>>() {
            Some(processor_info) => match processor_info.processor_family() {
                Some(family) => match family.value {
                    ProcessorFamily::SeeProcessorFamily2 => {
                        match processor_info.processor_family_2() {
                            Some(family) => Ok(format!("{}", family)),
                            None => Err(BiosParseError::ProcessorFamilyNotFound),
                        }
                    }
                    _ => Ok(format!("{}", family)),
                },
                None => Err(BiosParseError::ProcessorFamilyNotFound),
            },
            None => Err(BiosParseError::ProcessorFamilyNotFound),
        },
        Keyword::ProcessorManufacturer => data
            .find_map(|processor_info: SMBiosProcessorInformation<'_>| {
                processor_info.processor_manufacturer()
            })
            .ok_or(BiosParseError::ProcessorManufacturerNotFound),
        Keyword::ProcessorVersion => data
            .find_map(|processor_info: SMBiosProcessorInformation<'_>| {
                processor_info.processor_version()
            })
            .ok_or(BiosParseError::ProcessorVersionNotFound),
        Keyword::ProcessorFrequency => {
            match data.find_map(|processor_info: SMBiosProcessorInformation<'_>| {
                processor_info.current_speed()
            }) {
                Some(current_speed) => Ok(format!("{:?}", current_speed)),
                None => Err(BiosParseError::ProcessorFrequencyNotFound),
            }
        }
    }
}

#[test]
fn enum_display_exist_in_opt_string_keyword() -> Result<(), Box<dyn std::error::Error>> {
    let data = table_load_from_device()?;
    for keyword in Keyword::into_enum_iter() {
        let kstr = format!("{}", &keyword);
        opt_string_keyword(Keyword::from_str(&kstr)?, &data)?;
    }
    Ok(())
}