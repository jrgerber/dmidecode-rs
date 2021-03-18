use std::str::FromStr;

use crate::error::BiosParseError;
use smbioslib::*;
use structopt::StructOpt;

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

#[derive(Debug)]
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

#[derive(Debug, StructOpt)]
pub enum Keyword {
    BiosVendor,
    BiosVersion,
    BiosReleaseDate,
    BiosRevision,
    FirmewareRevision,
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
            "firmware-revision" => Ok(Keyword::FirmewareRevision),
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

impl Keyword {
    pub fn parse(&self, data: &SMBiosData) -> Result<String, BiosParseError> {
        match self {
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
            Keyword::FirmewareRevision => data
                .find_map(|bios_info: SMBiosInformation<'_>| {
                    match (
                        bios_info.e_c_firmware_major_release(),
                        bios_info.e_c_firmware_minor_release(),
                    ) {
                        (Some(major), Some(minor)) => Some(format!("{}.{}", major, minor)),
                        _ => None,
                    }
                })
                .ok_or(BiosParseError::FirmewareRevisionNotFound),
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
                .find_map(|baseboard_info: SMBiosBaseboardInformation<'_>| {
                    baseboard_info.asset_tag()
                })
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
}
