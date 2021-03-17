use smbioslib::*;
use crate::error::BiosParseError;

pub fn opt_string_keyword(keyword: String, data: &SMBiosData) -> Result<String, BiosParseError> {
    match keyword.to_lowercase().as_str() {
        "bios-vendor" => data
            .find_map(|bios_info: SMBiosInformation<'_>| bios_info.vendor())
            .ok_or(BiosParseError::BiosVendorNotFound),
        "bios-version" => data
            .find_map(|bios_info: SMBiosInformation<'_>| bios_info.version())
            .ok_or(BiosParseError::BiosVersionNotFound),
        "bios-release-date" => data
            .find_map(|bios_info: SMBiosInformation<'_>| bios_info.release_date())
            .ok_or(BiosParseError::BiosReleaseDateNotFound),
        "bios-revision" => data
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
        "firmware-revision" => data
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
        "system-manufacturer" => data
            .find_map(|system_info: SMBiosSystemInformation<'_>| system_info.manufacturer())
            .ok_or(BiosParseError::SystemManufacturerNotFound),
        "system-product-name" => data
            .find_map(|system_info: SMBiosSystemInformation<'_>| system_info.product_name())
            .ok_or(BiosParseError::SystemProductNameNotFound),
        "system-version" => data
            .find_map(|system_info: SMBiosSystemInformation<'_>| system_info.version())
            .ok_or(BiosParseError::SystemVersionNotFound),
        "system-serial-number" => data
            .find_map(|system_info: SMBiosSystemInformation<'_>| system_info.serial_number())
            .ok_or(BiosParseError::SystemSerialNumberNotFound),
        "system-uuid" => {
            match data.find_map(|system_info: SMBiosSystemInformation<'_>| system_info.uuid()) {
                // SystemUuidData is an enum that can be broken down further if desired
                Some(uuid) => Ok(format!("{:?}", uuid)),
                None => Err(BiosParseError::SystemUuidNotFound),
            }
        }
        "system-sku-number" => data
            .find_map(|system_info: SMBiosSystemInformation<'_>| system_info.sku_number())
            .ok_or(BiosParseError::SystemSkuNumberNotFound),
        "system-family" => data
            .find_map(|system_info: SMBiosSystemInformation<'_>| system_info.family())
            .ok_or(BiosParseError::SystemFamilyNotFound),
        "baseboard-manufacturer" => data
            .find_map(|baseboard_info: SMBiosBaseboardInformation<'_>| baseboard_info.manufacturer())
            .ok_or(BiosParseError::BaseboardManufacturerNotFound),
        "baseboard-product-name" => data
            .find_map(|baseboard_info: SMBiosBaseboardInformation<'_>| baseboard_info.product())
            .ok_or(BiosParseError::BaseboardProductNameNotFound),
        "baseboard-version" => data
            .find_map(|baseboard_info: SMBiosBaseboardInformation<'_>| baseboard_info.version())
            .ok_or(BiosParseError::BaseboardVersionNotFound),
        "baseboard-serial-number" => data
            .find_map(|baseboard_info: SMBiosBaseboardInformation<'_>| baseboard_info.serial_number())
            .ok_or(BiosParseError::BaseboardSerialNumberNotFound),
        "baseboard-asset-tag" => data
            .find_map(|baseboard_info: SMBiosBaseboardInformation<'_>| baseboard_info.asset_tag())
            .ok_or(BiosParseError::BaseboardAssetTagNotFound),
        "chassis-manufacturer" => data
            .find_map(|chassis_info: SMBiosSystemChassisInformation<'_>| chassis_info.manufacturer())
            .ok_or(BiosParseError::ChassisManufacturerNotFound),
        "chassis-type" => match data
            .find_map(|chassis_info: SMBiosSystemChassisInformation<'_>| chassis_info.chassis_type())
        {
            Some(chassis_type) => Ok(format!("{:?}", chassis_type)),
            None => Err(BiosParseError::ChassisTypeNotFound),
        },
        "chassis-version" => data
            .find_map(|chassis_info: SMBiosSystemChassisInformation<'_>| chassis_info.version())
            .ok_or(BiosParseError::ChassisVersionNotFound),
        "chassis-serial-number" => data
            .find_map(|chassis_info: SMBiosSystemChassisInformation<'_>| chassis_info.serial_number())
            .ok_or(BiosParseError::ChassisSerialNumberNotFound),
        "chassis-asset-tag" => data
            .find_map(|chassis_info: SMBiosSystemChassisInformation<'_>| {
                chassis_info.asset_tag_number()
            })
            .ok_or(BiosParseError::ChassisAssetTagNotFound),
        "processor-family" => match data.first::<SMBiosProcessorInformation<'_>>() {
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
        "processor-manufacturer" => data
            .find_map(|processor_info: SMBiosProcessorInformation<'_>| {
                processor_info.processor_manufacturer()
            })
            .ok_or(BiosParseError::ProcessorManufacturerNotFound),
        "processor-version" => data
            .find_map(|processor_info: SMBiosProcessorInformation<'_>| {
                processor_info.processor_version()
            })
            .ok_or(BiosParseError::ProcessorVersionNotFound),
        "processor-frequency" => match data
            .find_map(|processor_info: SMBiosProcessorInformation<'_>| processor_info.current_speed())
        {
            Some(current_speed) => Ok(format!("{:?}", current_speed)),
            None => Err(BiosParseError::ProcessorFrequencyNotFound),
        },
        _ => Err(BiosParseError::InvalidKeywordOnCommandLine),
    }
}