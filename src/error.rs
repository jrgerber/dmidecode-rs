use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum BiosParseError {
    BiosVendorNotFound,
    BiosVersionNotFound,
    BiosReleaseDateNotFound,
    BiosRevisionNotFound,
    FirmwareRevisionNotFound,
    SystemManufacturerNotFound,
    SystemProductNameNotFound,
    SystemVersionNotFound,
    SystemSerialNumberNotFound,
    SystemUuidNotFound,
    SystemSkuNumberNotFound,
    SystemFamilyNotFound,
    BaseboardManufacturerNotFound,
    BaseboardProductNameNotFound,
    BaseboardVersionNotFound,
    BaseboardSerialNumberNotFound,
    BaseboardAssetTagNotFound,
    ChassisManufacturerNotFound,
    ChassisTypeNotFound,
    ChassisVersionNotFound,
    ChassisSerialNumberNotFound,
    ChassisAssetTagNotFound,
    ProcessorFamilyNotFound,
    ProcessorManufacturerNotFound,
    ProcessorVersionNotFound,
    ProcessorFrequencyNotFound,
}

impl Error for BiosParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for BiosParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Here we can match and turn each arm into a human readable statement.
        // We have other variants to add so we will wait before doing so.
        write!(f, "{:?}", &self)
    }
}
