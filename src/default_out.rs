use std::str::FromStr;

use smbioslib::*;

const OUT_OF_SPEC: &str = "<OUT OF SPEC>";
const BYTES: &str = "bytes";
const KB: &str = "kB";
const MB: &str = "MB";
const GB: &str = "GB";

pub fn default_dump(smbios_data: &SMBiosData, quiet: bool) {
    for undefined_struct in smbios_data.iter() {
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
                if let Some(vendor) = data.vendor() {
                    println!("\tVendor: {}", vendor);
                }
                if let Some(version) = data.version() {
                    println!("\tVersion: {}", version);
                }
                if let Some(release_date) = data.release_date() {
                    println!("\tRelease Date: {}", release_date);
                }

                /*
                 * On IA-64, the BIOS base address will read 0 because
                 * there is no BIOS. Skip the base address and the
                 * runtime size in this case.
                 */
                if let Some(starting_address_segment) = data.starting_address_segment() {
                    if starting_address_segment != 0 {
                        println!("\tAddress: {:#06X}0", starting_address_segment);

                        let mut code = (0x10000u32 - starting_address_segment as u32) << 4;
                        let units;
                        if code & 0x000003FF != 0 {
                            units = BYTES;
                        } else {
                            units = KB;
                            code >>= 10;
                        }
                        println!("\tRuntime Size: {} {}", code, units);
                    }
                }

                if let Some(rom_size) = data.rom_size() {
                    if rom_size != 0xFF {
                        println!("\tROM Size: {} {}", ((rom_size + 1) as u64) << 6, KB);
                    } else {
                        if let Some(extended_rom_size) = data.extended_rom_size() {
                            print!("\tROM Size: ");
                            match extended_rom_size {
                                ExtendedRomSize::Megabytes(size) => {
                                    println!("{} {}", size, MB);
                                }
                                ExtendedRomSize::Gigabytes(size) => {
                                    println!("{} {}", size, GB);
                                }
                                ExtendedRomSize::Undefined(size) => {
                                    println!("{} ({})", OUT_OF_SPEC, size);
                                }
                            }
                        }
                    }
                }

                println!("\tCharacteristics:");
                if let Some(characteristics) = data.characteristics() {
                    // This isn't very clear what this bit is supposed to mean
                    if characteristics.bios_characteristics_not_supported() {
                        println!("\t\tBIOS characteristics not supported");
                    } else {
                        if characteristics.isa_supported() {
                            println!("\t\tISA is supported");
                        }
                        if characteristics.mca_supported() {
                            println!("\t\tMCA is supported");
                        }
                        if characteristics.eisa_supported() {
                            println!("\t\tEISA is supported");
                        }
                        if characteristics.pci_supported() {
                            println!("\t\tPCI is supported");
                        }
                        if characteristics.pcmcia_supported() {
                            println!("\t\tPC Card (PCMCIA) is supported");
                        }
                        if characteristics.plug_and_play_supported() {
                            println!("\t\tPNP is supported");
                        }
                        if characteristics.apm_supported() {
                            println!("\t\tAPM is supported");
                        }
                        if characteristics.bios_upgradeable() {
                            println!("\t\tBIOS is upgradeable");
                        }
                        if characteristics.bios_shadowing_allowed() {
                            println!("\t\tBIOS shadowing is allowed");
                        }
                        if characteristics.vlvesa_supported() {
                            println!("\t\tVLB is supported");
                        }
                        if characteristics.escd_support_available() {
                            println!("\t\tESCD support is available");
                        }
                        if characteristics.boot_from_cdsupported() {
                            println!("\t\tBoot from CD is supported");
                        }
                        if characteristics.selectable_boot_supported() {
                            println!("\t\tSelectable boot is supported");
                        }
                        if characteristics.bios_rom_socketed() {
                            println!("\t\tBIOS ROM is socketed");
                        }
                        if characteristics.boot_from_pcmcia_supported() {
                            println!("\t\tBoot from PC Card (PCMCIA) is supported");
                        }
                        if characteristics.edd_specification_supported() {
                            println!("\t\tEDD is supported");
                        }
                        if characteristics.floppy_nec_japanese_supported() {
                            println!(
                                "\t\tJapanese floppy for NEC 9800 1.2 MB is supported (int 13h)"
                            );
                        }
                        if characteristics.floppy_toshiba_japanese_supported() {
                            println!(
                                "\t\tJapanese floppy for Toshiba 1.2 MB is supported (int 13h)"
                            );
                        }
                        if characteristics.floppy_525_360_supported() {
                            println!("\t\t5.25\"/360 kB floppy services are supported (int 13h)");
                        }
                        if characteristics.floppy_525_12_supported() {
                            println!("\t\t5.25\"/1.2 MB floppy services are supported (int 13h)");
                        }
                        if characteristics.floppy_35_720_supported() {
                            println!("\t\t3.5\"/720 kB floppy services are supported (int 13h)");
                        }
                        if characteristics.floppy_35_288_supported() {
                            println!("\t\t3.5\"/2.88 MB floppy services are supported (int 13h)");
                        }
                        if characteristics.print_screen_service_supported() {
                            println!("\t\tPrint screen service is supported (int 5h)");
                        }
                        if characteristics.keyboard_8042services_supported() {
                            println!("\t\t8042 keyboard services are supported (int 9h)");
                        }
                        if characteristics.serial_services_supported() {
                            println!("\t\tSerial services are supported (int 14h)");
                        }
                        if characteristics.printer_services_supported() {
                            println!("\t\tPrinter services are supported (int 17h)");
                        }
                        if characteristics.printer_services_supported() {
                            println!("\t\tPrinter services are supported (int 17h)");
                        }
                        if characteristics.cga_mono_video_services_supported() {
                            println!("\t\tCGA/mono video services are supported (int 10h)");
                        }
                        if characteristics.nec_pc_98supported() {
                            println!("\t\tNEC PC-98");
                        }
                    }
                }

                if let Some(characteristics) = data.characteristics_extension0() {
                    if characteristics.acpi_is_supported() {
                        println!("\t\tACPI is supported");
                    }
                    if characteristics.usb_legacy_is_supported() {
                        println!("\t\\tUSB legacy is supported");
                    }
                    if characteristics.i2oboot_is_supported() {
                        println!("\t\tI2O boot is supported");
                    }
                    if characteristics.ls120super_disk_boot_is_supported() {
                        println!("\t\tLS-120 boot is supported");
                    }
                    if characteristics.atapi_zip_drive_boot_is_supported() {
                        println!("\t\tATAPI Zip drive boot is supported");
                    }
                    if characteristics.boot_1394is_supported() {
                        println!("\t\tIEEE 1394 boot is supported");
                    }
                    if characteristics.smart_battery_is_supported() {
                        println!("\t\tSmart battery is supported");
                    }
                }

                if let Some(characteristics) = data.characteristics_extension1() {
                    if characteristics.bios_boot_specification_is_supported() {
                        println!("\t\tBIOS boot specification is supported");
                    }
                    if characteristics.fkey_initiated_network_boot_is_supported() {
                        println!("\t\tFunction key-initiated network boot is supported");
                    }
                    if characteristics.targeted_content_distribution_is_supported() {
                        println!("\t\tTargeted content distribution is supported");
                    }
                    if characteristics.uefi_specification_is_supported() {
                        println!("\t\tUEFI is supported");
                    }
                    if characteristics.smbios_table_describes_avirtual_machine() {
                        println!("\t\tSystem is a virtual machine");
                    }
                }

                match (
                    data.system_bios_major_release(),
                    data.system_bios_minor_release(),
                ) {
                    (Some(major_release), Some(minor_release)) => {
                        if major_release != 0xFF && minor_release != 0xFF {
                            println!("\tBIOS Revision: {}.{}", major_release, minor_release);
                        }
                    }
                    _ => {}
                }

                match (
                    data.e_c_firmware_major_release(),
                    data.e_c_firmware_minor_release(),
                ) {
                    (Some(major_release), Some(minor_release)) => {
                        if major_release != 0xFF && minor_release != 0xFF {
                            println!("\tFirmware Revision: {}.{}", major_release, minor_release);
                        }
                    }
                    _ => {}
                }
            }
            DefinedStruct::SystemInformation(data) => {
                println!("System Information");
                if let Some(manufacturer) = data.manufacturer() {
                    println!("\tManufacturer: {}", manufacturer);
                }
                if let Some(product_name) = data.product_name() {
                    println!("\tProduct Name: {}", product_name);
                }
                if let Some(version) = data.version() {
                    println!("\tVersion: {}", version);
                }
                if let Some(serial_number) = data.serial_number() {
                    println!("\tSerial Number: {}", serial_number);
                }
                if let Some(uuid) = data.uuid() {
                    print!("\tUUID: ");
                    match uuid {
                        SystemUuidData::IdNotPresentButSettable => {
                            println!("Not Present");
                        }
                        SystemUuidData::IdNotPresent => {
                            println!("Not Settable");
                        }
                        SystemUuidData::Uuid(val) => {
                            /*
                             * As of version 2.6 of the SMBIOS specification, the first 3
                             * fields of the UUID are supposed to be encoded on little-endian.
                             * The specification says that this is the defacto standard,
                             * however I've seen systems following RFC 4122 instead and use
                             * network byte order, so I am reluctant to apply the byte-swapping
                             * for older versions.
                             */

                            let two_six_version = &SMBiosVersion {
                                major: 2,
                                minor: 6,
                                revision: 0,
                            };
                            if let Some(version) = &smbios_data.version {
                                if version < two_six_version {
                                    let p = val.raw;
                                    println!("{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}", 
                                    p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7], p[8], p[9], p[10], p[11], p[12], p[13], p[14], p[15]);
                                } else {
                                    println!("{}", val);
                                }
                            } else {
                                println!("{}", val);
                            }
                        }
                    }
                }

                if let Some(wakeup_type) = data.wakeup_type() {
                    print!("\tWake-up Type: ");
                    match wakeup_type.value {
                        SystemWakeUpType::Other => {
                            println!("Other");
                        }
                        SystemWakeUpType::Unknown => {
                            println!("Unknown");
                        }
                        SystemWakeUpType::ApmTimer => {
                            println!("APM Timer");
                        }
                        SystemWakeUpType::ModernRing => {
                            println!("Modem Ring");
                        }
                        SystemWakeUpType::LanRemote => {
                            println!("LAN Remote");
                        }
                        SystemWakeUpType::PowerSwitch => {
                            println!("Power Switch");
                        }
                        SystemWakeUpType::PciPme => {
                            println!("PCI PME#");
                        }
                        SystemWakeUpType::ACPowerRestored => {
                            println!("AC Power Restored");
                        }
                        SystemWakeUpType::None => {
                            println!("{} ({})", OUT_OF_SPEC, wakeup_type.raw);
                        }
                    }
                }
                if let Some(sku_number) = data.sku_number() {
                    println!("\tSKU Number: {}", sku_number);
                }
                if let Some(family) = data.family() {
                    println!("\tFamily: {}", family);
                }
            }
            DefinedStruct::BaseBoardInformation(data) => {
                println!("Base Board Information");
                if let Some(manufacturer) = data.manufacturer() {
                    println!("\tManufacturer: {}", manufacturer);
                }
                if let Some(product) = data.product() {
                    println!("\tProduct: {}", product);
                }
                if let Some(version) = data.version() {
                    println!("\tVersion: {}", version);
                }
                if let Some(serial_number) = data.serial_number() {
                    println!("\tSerial Number: {}", serial_number);
                }
                if let Some(asset_tag) = data.asset_tag() {
                    println!("\tAsset Tag: {}", asset_tag);
                }
                if let Some(feature_flags) = data.feature_flags() {
                    println!("\tFeatures:");
                    if feature_flags.hosting_board() {
                        println!("\t\tBoard is a hosting board");
                    }
                    if feature_flags.requires_daughterboard() {
                        println!("\t\tBoard requires at least one daughter board");
                    }
                    if feature_flags.is_removable() {
                        println!("\t\tBoard is removable");
                    }
                    if feature_flags.is_replaceable() {
                        println!("\t\tBoard is replaceable");
                    }
                    if feature_flags.is_hot_swappable() {
                        println!("\t\tBoard is hot swappable");
                    }
                }
                if let Some(location_in_chassis) = data.location_in_chassis() {
                    println!("\tLocation in Chassis: {}", location_in_chassis);
                }
                if !quiet {
                    if let Some(chassis_handle) = data.chassis_handle() {
                        println!("\tChassis Handle: {:#06X}", chassis_handle.0);
                    }
                }
                if let Some(board_type) = data.board_type() {
                    print!("\tType: ");
                    match board_type.value {
                        BoardType::Unknown => {
                            println!("Unknown");
                        }
                        BoardType::Other => {
                            println!("Other");
                        }
                        BoardType::ServerBlade => {
                            println!("Server Blade");
                        }
                        BoardType::ConnectivitySwitch => {
                            println!("Connectivity Switch");
                        }
                        BoardType::SystemManagementModule => {
                            println!("System Management Module");
                        }
                        BoardType::ProcessorModule => {
                            println!("Processor Module");
                        }
                        BoardType::IOModule => {
                            println!("I/O Module");
                        }
                        BoardType::MemoryModule => {
                            println!("Memory Module");
                        }
                        BoardType::Daughterboard => {
                            println!("Daughter Board");
                        }
                        BoardType::Motherboard => {
                            println!("Motherboard");
                        }
                        BoardType::ProcessorMemoryModule => {
                            println!("Processor+Memory Module");
                        }
                        BoardType::ProcessorIOModule => {
                            println!("Processor+I/O Module");
                        }
                        BoardType::InterconnectBoard => {
                            println!("Interconnect Board");
                        }
                        BoardType::None => {
                            println!("{} ({})", OUT_OF_SPEC, board_type.raw);
                        }
                    }
                }

                if !quiet {
                    if let Some(handle_count) = data.number_of_contained_object_handles() {
                        if handle_count > 0 {
                            println!("Contained Object Handles: {}", handle_count);
                            for handle in data.contained_object_handle_iterator() {
                                println!("\t\t{:#06X}", handle.0);
                            }
                        }
                    }
                }
            }
            DefinedStruct::SystemChassisInformation(data) => {
                println!("Chassis Information");
                if let Some(manufacturer) = data.manufacturer() {
                    println!("\tManufacturer: {}", manufacturer);
                }
                if let Some(chassis_type) = data.chassis_type() {
                    print!("\tType: ");
                    let print = match chassis_type.value {
                        ChassisType::Other => "Other",
                        ChassisType::Unknown => "Unknown",
                        ChassisType::Desktop => "Desktop",
                        ChassisType::LowProfileDesktop => "Low Profile Desktop",
                        ChassisType::PizzaBox => "Pizza Box",
                        ChassisType::MiniTower => "Mini Tower",
                        ChassisType::Tower => "Tower",
                        ChassisType::Portable => "Portable",
                        ChassisType::Laptop => "Laptop",
                        ChassisType::Notebook => "Notebook",
                        ChassisType::HandHeld => "Hand Held",
                        ChassisType::DockingStation => "Docking Station",
                        ChassisType::AllInOne => "All In One",
                        ChassisType::SubNotebook => "Sub Notebook",
                        ChassisType::SpaceSaving => "Space-saving",
                        ChassisType::LunchBox => "Lunch Box",
                        ChassisType::MainServerChassis => "Main Server Chassis",
                        ChassisType::ExpansionChassis => "Expansion Chassis",
                        ChassisType::SubChassis => "Sub Chassis",
                        ChassisType::BusExpansionChassis => "Bus Expansion Chassis",
                        ChassisType::PeripheralChassis => "Peripheral Chassis",
                        ChassisType::RaidChassis => "RAID Chassis",
                        ChassisType::RackMountChassis => "Rack Mount Chassis",
                        ChassisType::SealedCasePC => "Sealed-case PC",
                        ChassisType::MultiSystemChassis => "Multi-system",
                        ChassisType::CompactPci => "CompactPCI",
                        ChassisType::AdvancedTca => "AdvancedTCA",
                        ChassisType::Blade => "Blade",
                        ChassisType::BladeEnclosure => "Blade Enclosing",
                        ChassisType::Tablet => "Tablet",
                        ChassisType::Convertible => "Convertible",
                        ChassisType::Detachable => "Detachable",
                        ChassisType::IoTGateway => "IoT Gateway",
                        ChassisType::EmbeddedPC => "Embedded PC",
                        ChassisType::MiniPC => "Mini PC",
                        ChassisType::StickPC => "Stick PC",
                        ChassisType::None => "",
                    };

                    if print == "" {
                        println!("{} ({})", OUT_OF_SPEC, chassis_type.raw);
                    } else {
                        println!("{}", print);
                    }

                    print!("\tLock: ");
                    if chassis_type.raw & 0x80 == 0x80 {
                        println!("Present");
                    } else {
                        println!("Not Present");
                    }
                }
                if let Some(version) = data.version() {
                    println!("\tVersion: {}", version);
                }
                if let Some(serial_number) = data.serial_number() {
                    println!("\tSerial Number: {}", serial_number);
                }
                if let Some(asset_tag_number) = data.asset_tag_number() {
                    println!("\tAsset Tag: {}", asset_tag_number);
                }
                if let Some(bootup_state) = data.bootup_state() {
                    println!("\tBoot-up State: {}", dmi_chassis_state(bootup_state));
                }
                if let Some(power_supply_state) = data.power_supply_state() {
                    println!(
                        "\tPower Supply State: {}",
                        dmi_chassis_state(power_supply_state)
                    );
                }

                if let Some(thermal_state) = data.thermal_state() {
                    println!("\tThermal State: {}", dmi_chassis_state(thermal_state));
                }

                if let Some(security_status) = data.security_status() {
                    println!(
                        "\tSecurity Status: {}",
                        match security_status.value {
                            ChassisSecurityStatus::Other => "Other".to_string(),
                            ChassisSecurityStatus::Unknown => "Unknown".to_string(),
                            ChassisSecurityStatus::StatusNone => "None".to_string(),
                            ChassisSecurityStatus::ExternalInterfaceLockedOut =>
                                "External Interface Locked Out".to_string(),
                            ChassisSecurityStatus::ExternalInterfaceEnabled =>
                                "External Interface Enabled".to_string(),
                            ChassisSecurityStatus::None =>
                                format!("{} ({})", OUT_OF_SPEC, security_status.raw),
                        }
                    );
                }
                if let Some(oem_defined) = data.oem_defined() {
                    println!("\tOEM Information: {:#010X}", oem_defined);
                }
                if let Some(height) = data.height() {
                    match height {
                        ChassisHeight::Unspecified => {
                            println!("\tHeight: Unspecified");
                        }
                        ChassisHeight::U(units) => {
                            println!("\tHeight: {} U", units);
                        }
                    }
                }
                if let Some(number_of_power_cords) = data.number_of_power_cords() {
                    match number_of_power_cords {
                        PowerCords::Unspecified => {
                            println!("\tNumber Of Power Cords: Unspecified");
                        }
                        PowerCords::Count(count) => {
                            println!("\tNumber Of Power Cords: {}", count);
                        }
                    }
                }
                if let Some(contained_element_count) = data.contained_element_count() {
                    println!("Contained Elements: {}", contained_element_count);
                    if let Some(elements) = data.contained_elements() {
                        for element in elements.into_iter() {
                            let type_description = match element.element_type() {
                                ElementType::BaseboardType(baseboard_type) => {
                                    match baseboard_type.value {
                                        BoardType::Unknown => "Unknown".to_string(),
                                        BoardType::Other => "Other".to_string(),
                                        BoardType::ServerBlade => "ServerBlade".to_string(),
                                        BoardType::ConnectivitySwitch => {
                                            "Connectivity Switch".to_string()
                                        }
                                        BoardType::SystemManagementModule => {
                                            "System Management Module".to_string()
                                        }
                                        BoardType::ProcessorModule => {
                                            "Processor Module".to_string()
                                        }
                                        BoardType::IOModule => "I/O Module".to_string(),
                                        BoardType::MemoryModule => "Memory Module".to_string(),
                                        BoardType::Daughterboard => "Daughter Board".to_string(),
                                        BoardType::Motherboard => "Motherboard".to_string(),
                                        BoardType::ProcessorMemoryModule => {
                                            "Processor Memory Module".to_string()
                                        }
                                        BoardType::ProcessorIOModule => {
                                            "Processor+I/O Module".to_string()
                                        }
                                        BoardType::InterconnectBoard => {
                                            "InterconnectBoard".to_string()
                                        }
                                        BoardType::None => {
                                            format!("{} ({})", OUT_OF_SPEC, baseboard_type.raw)
                                                .to_string()
                                        }
                                    }
                                }
                                ElementType::SMBiosType(bios_type) => {
                                    dmi_smbios_structure_type(*bios_type)
                                }
                            };
                            match (element.element_minimum(), element.element_maximum()) {
                                (
                                    ElementMinimum::Count(minimum),
                                    ElementMaximum::Count(maximum),
                                ) => {
                                    let range = match minimum == maximum {
                                        true => format!("{}", minimum),
                                        false => format!("{}-{}", minimum, maximum),
                                    };
                                    println!("\t\t{} {}", type_description, range);
                                }
                                _ => (),
                            }
                        }
                    }
                }
                if let Some(sku_number) = data.sku_number() {
                    println!("\tSKU Number: {}", sku_number);
                }
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

        fn dmi_smbios_structure_type(code: u8) -> String {
            let description = match code {
                0 => "BIOS",
                1 => "System",
                2 => "Base Board",
                3 => "Chassis",
                4 => "Processor",
                5 => "Memory Controller",
                6 => "Memory Module",
                7 => "Cache",
                8 => "Port Connector",
                9 => "System Slots",
                10 => "On Board Devices",
                11 => "OEM Strings",
                12 => "System Configuration Options",
                13 => "BIOS Language",
                14 => "Group Associations",
                15 => "System Event Log",
                16 => "Physical Memory Array",
                17 => "Memory Device",
                18 => "32-bit Memory Error",
                19 => "Memory Array Mapped Address",
                20 => "Memory Device Mapped Address",
                21 => "Built-in Pointing Device",
                22 => "Portable Battery",
                23 => "System Reset",
                24 => "Hardware Security",
                25 => "System Power Controls",
                26 => "Voltage Probe",
                27 => "Cooling Device",
                28 => "Temperature Probe",
                29 => "Electrical Current Probe",
                30 => "Out-of-band Remote Access",
                31 => "Boot Integrity Services",
                32 => "System Boot",
                33 => "64-bit Memory Error",
                34 => "Management Device",
                35 => "Management Device Component",
                36 => "Management Device Threshold Data",
                37 => "Memory Channel",
                38 => "IPMI Device",
                39 => "Power Supply",
                40 => "Additional Information",
                41 => "Onboard Device",
                42 => "Management Controller Host Interface",
                43 => "TPM Device",
                _ => "",
            };

            match description == "" {
                true => match code >= 128 {
                    true => "OEM-specific".to_string(),
                    false => format!("{} ({})", OUT_OF_SPEC, code),
                },
                false => description.to_string(),
            }
        }
        fn dmi_chassis_state(state: ChassisStateData) -> String {
            match state.value {
                ChassisState::Other => "Other".to_string(),
                ChassisState::Unknown => "Unknown".to_string(),
                ChassisState::Safe => "Safe".to_string(),
                ChassisState::Warning => "Warning".to_string(),
                ChassisState::Critical => "Critical".to_string(),
                ChassisState::NonRecoverable => "Non-recoverable".to_string(),
                ChassisState::None => format!("{} ({})", OUT_OF_SPEC, state.raw),
            }
        }
    }
}
