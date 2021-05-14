use smbioslib::*;

pub fn default_dump(data: &SMBiosData) {
    for undefined_struct in data.iter() {
        /*
            # dmidecode 3.1
            Getting SMBIOS data from sysfs.
            SMBIOS 2.3 present.
            338 structures occupying 17307 bytes.
            Table at 0x000F93D0.

            Handle 0x0000, DMI type 0, 20 bytes
            BIOS Information
                    Vendor: American Megatrends Inc.
                    Version: 090008
                    Release Date: 12/07/2018
                    Address: 0xF0000
                    Runtime Size: 64 kB
                    ROM Size: 256 kB
                    Characteristics:
                            ISA is supported
                            PCI is supported
                            PNP is supported
        */
        println!();
        println!(
            "Handle {:#06X}, DMI type {}, {} bytes",
            *undefined_struct.header.handle(),
            undefined_struct.header.struct_type(),
            undefined_struct.fields.len()
        );

        // TODO: https://github.com/mirror/dmidecode/blob/master/dmidecode.c lines 4059+
        match undefined_struct.defined_struct() {
            DefinedStruct::Information(data) => {
                println!("BIOS Information");
                println!("\tVendor: {}", data.vendor().unwrap_or_default());
                println!("\tVersion: {}", data.version().unwrap_or_default());
                println!(
                    "\tRelease Date: {}",
                    data.release_date().unwrap_or_default()
                );
            }
            DefinedStruct::SystemInformation(data) => {
                println!("System Information");
            }
            DefinedStruct::BaseBoardInformation(data) => {
                println!("Base Board Information");
            }
            DefinedStruct::SystemChassisInformation(data) => {
                println!("Chassis Information");
            }
            DefinedStruct::ProcessorInformation(data) => {
                println!("Processor Information");
            }
            DefinedStruct::MemoryControllerInformation(data) => {
                println!("Memory Controller Information");
            }
            DefinedStruct::MemoryModuleInformation(data) => {
                println!("Memory Module Information");
            }
            DefinedStruct::CacheInformation(data) => {
                println!("Cache Information");
            }
            DefinedStruct::PortConnectorInformation(data) => {
                println!("Port Connector Information");
            }
            DefinedStruct::SystemSlot(data) => {
                println!("System Slot Information");
            }
            DefinedStruct::OnBoardDeviceInformation(data) => {
                println!("On Board Devices Information");
            }
            DefinedStruct::OemStrings(data) => {
                println!("OEM Strings");
            }
            DefinedStruct::SystemConfigurationOptions(data) => {
                println!("System Configuration Options");
            }
            DefinedStruct::LanguageInformation(data) => {
                println!("BIOS Language Information");
            }
            DefinedStruct::GroupAssociations(data) => {
                println!("Group Associations");
            }
            DefinedStruct::EventLog(data) => {
                println!("System Event Log");
            }
            DefinedStruct::PhysicalMemoryArray(data) => {
                println!("Physical Memory Array");
            }
            DefinedStruct::MemoryDevice(data) => {
                println!("Memory Device");
            }
            DefinedStruct::MemoryErrorInformation32Bit(data) => {
                println!("32-bit Memory Error Information");
            }
            DefinedStruct::MemoryArrayMappedAddress(data) => {
                println!("Memory Array Mapped Address");
            }
            DefinedStruct::MemoryDeviceMappedAddress(data) => {
                println!("Memory Device Mapped Address");
            }
            DefinedStruct::BuiltInPointingDevice(data) => {
                println!("Built-in Pointing Device");
            }
            DefinedStruct::PortableBattery(data) => {
                println!("Portable Battery");
            }
            DefinedStruct::SystemReset(data) => {
                println!("System Reset");
            }
            DefinedStruct::HardwareSecurity(data) => {
                println!("Hardware Security");
            }
            DefinedStruct::SystemPowerControls(data) => {
                println!("System Power Controls");
            }
            DefinedStruct::VoltageProbe(data) => {
                println!("Voltage Probe");
            }
            DefinedStruct::CoolingDevice(data) => {
                println!("Cooling Device");
            }
            DefinedStruct::TemperatureProbe(data) => {
                println!("Temperature Probe");
            }
            DefinedStruct::ElectricalCurrentProbe(data) => {
                println!("Electrical Current Probe");
            }
            DefinedStruct::OutOfBandRemoteAccess(data) => {
                println!("Out-of-band Remote Access");
            }
            DefinedStruct::BisEntryPoint(data) => {
                println!("Boot Integrity Services Entry Point");
            }
            DefinedStruct::SystemBootInformation(data) => {
                println!("System Boot Information");
            }
            DefinedStruct::MemoryErrorInformation64Bit(data) => {
                println!("64-bit Memory Error Information");
            }
            DefinedStruct::ManagementDevice(data) => {
                println!("Management Device");
            }
            DefinedStruct::ManagementDeviceComponent(data) => {
                println!("Management Device Component");
            }
            DefinedStruct::ManagementDeviceThresholdData(data) => {
                println!("Management Device Threshold Data");
            }
            DefinedStruct::MemoryChannel(data) => {
                println!("Memory Channel");
            }
            DefinedStruct::IpmiDeviceInformation(data) => {
                println!("IPMI Device Information");
            }
            DefinedStruct::SystemPowerSupply(data) => {
                println!("System Power Supply");
            }
            DefinedStruct::AdditionalInformation(data) => {
                println!("Additional Information");
            }
            DefinedStruct::OnboardDevicesExtendedInformation(data) => {
                println!("Onboard Device");
            }
            DefinedStruct::ManagementControllerHostInterface(data) => {
                println!("Management Controller Host Interface");
            }
            DefinedStruct::TpmDevice(data) => {
                println!("TPM Device");
            }
            DefinedStruct::ProcessorAdditionalInformation(data) => {
                println!("Processor Additional Information");
            }
            DefinedStruct::Inactive(data) => {
                println!("Inactive");
            }
            DefinedStruct::EndOfTable(data) => {
                println!("End Of Table");
            }
            DefinedStruct::Undefined(data) => {
                if data.parts().header.struct_type() >= 128 {
                    println!("OEM-specific");
                } else {
                    println!("Unknown");
                }
            }
        }

        // print!("\tHeader and Data:");
        // for item in undefined_struct.fields.iter().enumerate() {
        //     if item.0 % 16 == 0 {
        //         println!();
        //         print!("\t\t");
        //     }
        //     print!("{:02X} ", item.1);
        // }
        // println!();
        // print!("\tStrings:");
        // for string_item in undefined_struct.strings.iter() {
        //     // chain() adds a terminating \0 for parity with the original dmidecode
        //     for item in string_item.iter().chain([0].iter()).enumerate() {
        //         if item.0 % 16 == 0 {
        //             println!();
        //             print!("\t\t");
        //         }
        //         print!("{:02X} ", item.1);
        //     }
        //     println!();
        //     let as_string: String = string_item.iter().map(|x| *x as char).collect();
        //     print!("\t\t\"{}\"", as_string);
        // }
        println!();
    }
}
