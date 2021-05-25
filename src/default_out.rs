use std::convert::TryInto;

use smbioslib::*;

const OUT_OF_SPEC: &str = "<OUT OF SPEC>";
const BYTES: &str = "bytes";
const KB: &str = "kB";
const MB: &str = "MB";
const GB: &str = "GB";
const OTHER: &str = "Other";
const UNKNOWN: &str = "Unknown";
const NONE: &str = "None";

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
        dump_defined_struct(
            &undefined_struct.defined_struct(),
            smbios_data.version,
            quiet,
        );
    }
}

pub fn dump_defined_struct(
    defined_struct: &DefinedStruct<'_>,
    bios_version: Option<SMBiosVersion>,
    quiet: bool,
) {
    match defined_struct {
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
                        println!("\t\tJapanese floppy for NEC 9800 1.2 MB is supported (int 13h)");
                    }
                    if characteristics.floppy_toshiba_japanese_supported() {
                        println!("\t\tJapanese floppy for Toshiba 1.2 MB is supported (int 13h)");
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
                    println!("\t\tUSB legacy is supported");
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

                        let two_six_version = SMBiosVersion {
                            major: 2,
                            minor: 6,
                            revision: 0,
                        };
                        if let Some(version) = bios_version {
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
                        println!("{}", OTHER);
                    }
                    SystemWakeUpType::Unknown => {
                        println!("{}", UNKNOWN);
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
                        println!("{}", UNKNOWN);
                    }
                    BoardType::Other => {
                        println!("{}", OTHER);
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
                    ChassisType::Other => OTHER,
                    ChassisType::Unknown => UNKNOWN,
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
                        ChassisSecurityStatus::Other => OTHER.to_string(),
                        ChassisSecurityStatus::Unknown => UNKNOWN.to_string(),
                        ChassisSecurityStatus::StatusNone => NONE.to_string(),
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
                println!("\tContained Elements: {}", contained_element_count);
                if let Some(elements) = data.contained_elements() {
                    for element in elements.into_iter() {
                        let type_description = match element.element_type() {
                            ElementType::BaseboardType(baseboard_type) => {
                                match baseboard_type.value {
                                    BoardType::Unknown => UNKNOWN.to_string(),
                                    BoardType::Other => OTHER.to_string(),
                                    BoardType::ServerBlade => "ServerBlade".to_string(),
                                    BoardType::ConnectivitySwitch => {
                                        "Connectivity Switch".to_string()
                                    }
                                    BoardType::SystemManagementModule => {
                                        "System Management Module".to_string()
                                    }
                                    BoardType::ProcessorModule => "Processor Module".to_string(),
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
                                    BoardType::InterconnectBoard => "InterconnectBoard".to_string(),
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
                            (ElementMinimum::Count(minimum), ElementMaximum::Count(maximum)) => {
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
            if let Some(socket_designation) = data.socket_designation() {
                println!("\tSocket Designation: {}", socket_designation);
            }
            if let Some(processor_type) = data.processor_type() {
                println!("\tType: {}", dmi_processor_type(processor_type));
            }
            if let Some(processor_family) = data.processor_family() {
                if processor_family.value == ProcessorFamily::SeeProcessorFamily2 {
                    if let Some(processor_family_2) = data.processor_family_2() {
                        println!(
                            "\tFamily: {}",
                            dmi_processor_family(processor_family_2.value, processor_family_2.raw)
                        );
                    }
                } else {
                    println!(
                        "\tFamily: {}",
                        dmi_processor_family(processor_family.value, processor_family.raw as u16)
                    );
                }
            }
            if let Some(processor_manufacturer) = data.processor_manufacturer() {
                println!("\tManufacturer: {}", processor_manufacturer);
            }

            dmi_processor_id(&data);

            if let Some(processor_version) = data.processor_version() {
                println!("\tVersion: {}", processor_version);
            }
            if let Some(voltage) = data.voltage() {
                print!("\tVoltage: ");
                match voltage {
                    ProcessorVoltage::CurrentVolts(volts) => println!("{:.1} V", volts),
                    ProcessorVoltage::SupportedVolts(supported) => {
                        let voltages = supported.voltages();
                        match voltages.len() == 0 {
                            true => print!("{}", UNKNOWN),
                            false => {
                                let mut iter = voltages.iter();
                                print!("{:.1} V", iter.next().unwrap());
                                while let Some(voltage) = iter.next() {
                                    // Insert space if not the first value
                                    print!(" {:.1} V", voltage);
                                }
                                println!();
                            }
                        }
                    }
                }
            }
            if let Some(external_clock) = data.external_clock() {
                print!("\tExternal Clock: ");
                match external_clock {
                    ProcessorExternalClock::Unknown => println!("{}", UNKNOWN),
                    ProcessorExternalClock::MHz(mhz) => println!("{} MHz", mhz),
                }
            }
            if let Some(max_speed) = data.max_speed() {
                print!("\tMax Speed: ");
                match max_speed {
                    ProcessorSpeed::Unknown => println!("{}", UNKNOWN),
                    ProcessorSpeed::MHz(mhz) => println!("{} MHz", mhz),
                }
            }
            if let Some(current_speed) = data.current_speed() {
                print!("\tCurrent Speed: ");
                match current_speed {
                    ProcessorSpeed::Unknown => println!("{}", UNKNOWN),
                    ProcessorSpeed::MHz(mhz) => println!("{} MHz", mhz),
                }
            }
            if let Some(status) = data.status() {
                print!("\tStatus: ");
                match status.socket_populated() {
                    true => {
                        print!("Populated, ");
                        let print = match status.cpu_status() {
                            CpuStatus::Unknown => UNKNOWN,
                            CpuStatus::Enabled => "Enabled",
                            CpuStatus::UserDisabled => "Disabled by User",
                            CpuStatus::BiosDisabled => "Disabled by BIOS",
                            CpuStatus::Idle => "Idle",
                            CpuStatus::Other => OTHER,
                            CpuStatus::None => "",
                        };
                        match print == "" {
                            true => println!("{} ({})", OUT_OF_SPEC, status.raw),
                            false => println!("{}", print),
                        }
                    }
                    false => println!("Unpopulated"),
                }
            }
            if let Some(processor_upgrade) = data.processor_upgrade() {
                println!("\tUpgrade: {0}", dmi_processor_upgrade(processor_upgrade));
            }
            if !quiet {
                if let Some(handle) = data.l1cache_handle() {
                    dmi_processor_cache("L1 Cache Handle", handle, "L1", bios_version);
                }
                if let Some(handle) = data.l2cache_handle() {
                    dmi_processor_cache("L2 Cache Handle", handle, "L2", bios_version);
                }
                if let Some(handle) = data.l3cache_handle() {
                    dmi_processor_cache("L3 Cache Handle", handle, "L3", bios_version);
                }
            }
            if let Some(serial_number) = data.serial_number() {
                println!("\tSerial Number: {}", serial_number);
            }
            if let Some(asset_tag) = data.asset_tag() {
                println!("\tAsset Tag: {}", asset_tag);
            }
            if let Some(part_number) = data.part_number() {
                println!("\tPart Number: {}", part_number);
            }
            if let Some(core_count) = data.core_count() {
                print!("\tCore Count: ");
                match core_count {
                    CoreCount::Unknown => println!("{}", UNKNOWN),
                    CoreCount::Count(count) => println!("{}", count),
                    CoreCount::SeeCoreCount2 => match data.core_count_2() {
                        Some(core_count_2) => match core_count_2 {
                            CoreCount2::Unknown => println!("{}", UNKNOWN),
                            CoreCount2::Count(count) => println!("{}", count),
                            CoreCount2::Reserved => println!("Reserved"),
                        },
                        // CoreCount said to read CoreCount2 but CoreCount2 does not exist.
                        None => println!("Error"),
                    },
                }
            }
            if let Some(cores_enabled) = data.cores_enabled() {
                print!("\tCore Enabled: ");
                match cores_enabled {
                    CoresEnabled::Unknown => println!("{}", UNKNOWN),
                    CoresEnabled::Count(count) => println!("{}", count),
                    CoresEnabled::SeeCoresEnabled2 => match data.cores_enabled_2() {
                        Some(cores_enabled_2) => match cores_enabled_2 {
                            CoresEnabled2::Unknown => println!("{}", UNKNOWN),
                            CoresEnabled2::Count(count) => println!("{}", count),
                            CoresEnabled2::Reserved => println!("Reserved"),
                        },
                        // CoreEnabled said to read CoreEnabled2 but CoreEnabled2 does not exist.
                        None => println!("Error"),
                    },
                }
            }
            if let Some(thread_count) = data.thread_count() {
                print!("\tThread Count: ");
                match thread_count {
                    ThreadCount::Unknown => println!("{}", UNKNOWN),
                    ThreadCount::Count(count) => println!("{}", count),
                    ThreadCount::SeeThreadCount2 => match data.thread_count_2() {
                        Some(thread_count_2) => match thread_count_2 {
                            ThreadCount2::Unknown => println!("{}", UNKNOWN),
                            ThreadCount2::Count(count) => println!("{}", count),
                            ThreadCount2::Reserved => println!("Reserved"),
                        },
                        // ThreadCount said to read ThreadCount2 but ThreadCount2 does not exist.
                        None => println!("Error"),
                    },
                }
            }
            if let Some(processor_characteristics) = data.processor_characteristics() {
                dmi_processor_characteristics(processor_characteristics);
            }
        }
        DefinedStruct::MemoryControllerInformation(data) => {
            println!("Memory Controller Information");
            if let Some(error_detecting_method) = data.error_detecting_method() {
                println!(
                    "\tError Detecting Method: {}",
                    dmi_memory_controller_ed_method(error_detecting_method)
                );
            }
            if let Some(error_correcting_capabilities) = data.error_correcting_capability() {
                dmi_memory_controller_ec_capabilities(
                    "Error Correcting Capabilities",
                    error_correcting_capabilities,
                );
            }
            if let Some(supported_interleave) = data.supported_interleave() {
                println!(
                    "\tSupported Interleave: {}",
                    dmi_memory_controller_interleave(supported_interleave)
                );
            }
            if let Some(current_interleave) = data.current_interleave() {
                println!(
                    "\tCurrent Interleave: {}",
                    dmi_memory_controller_interleave(current_interleave)
                );
            }

            match (
                data.maximum_memory_module_size(),
                data.number_of_associated_memory_slots(),
            ) {
                (Some(size_power), Some(count)) => {
                    if let Some(module_size_mb) = 0x1u128.checked_shl(size_power as u32) {
                        println!("\tMaximum Memory Module Size: {} MB", module_size_mb);
                        if let Some(modules_total_size_mb) =
                            module_size_mb.checked_mul(count as u128)
                        {
                            println!("\tMaximum Total Memory Size: {} MB", modules_total_size_mb);
                        }
                    }
                }
                _ => (),
            }
            if let Some(supported_speeds) = data.supported_speeds() {
                dmi_memory_controller_speeds(supported_speeds);
            }
            if let Some(supported_memory_types) = data.supported_memory_types() {
                dmi_memory_module_types("Supported Memory Types", supported_memory_types, false);
            }

            dmi_memory_controller_slots(data.memory_module_handle_iterator());

            for capability in data.error_correcting_capabilities_iterator() {
                dmi_memory_controller_ec_capabilities(
                    "Enabled Error Correcting Capabilities",
                    capability,
                );
            }
        }
        DefinedStruct::MemoryModuleInformation(data) => {
            println!("Memory Module Information");
            if let Some(socket_designation) = data.socket_designation() {
                println!("\tSocket Designation: {}", socket_designation);
            }
            if let Some(bank_connections) = data.bank_connections() {
                dmi_memory_module_connections(bank_connections);
            }
            if let Some(current_speed) = data.current_speed() {
                dmi_memory_module_speed("Current Speed", current_speed);
            }
            if let Some(current_memory_type) = data.current_memory_type() {
                dmi_memory_module_types("Type", current_memory_type, true);
            }
            if let Some(installed_size) = data.installed_size() {
                dmi_memory_module_size("Installed Size", installed_size);
            }
            if let Some(enabled_size) = data.enabled_size() {
                dmi_memory_module_size("Enabled Size", enabled_size);
            }
            if let Some(error_status) = data.error_status() {
                dmi_memory_module_error(error_status);
            }
        }
        DefinedStruct::CacheInformation(data) => {
            println!("Cache Information");
            if let Some(socket_designation) = data.socket_designation() {
                println!("\t{}", socket_designation);
            }
            if let Some(cache_configuration) = data.cache_configuration() {
                println!(
                    "\tConfiguration: {} {} Level {}",
                    match cache_configuration.enabled_at_boot() {
                        true => "Enabled",
                        false => "Disabled",
                    },
                    match cache_configuration.cache_socketed() {
                        true => "Socketed",
                        false => "Not Socketed",
                    },
                    cache_configuration.cache_level()
                );
                println!(
                    "\tOperational Mode: {}",
                    match cache_configuration.operational_mode() {
                        CacheOperationalMode::WriteThrough => "Write Through",
                        CacheOperationalMode::WriteBack => "Write Back",
                        CacheOperationalMode::VariesWithMemoryAddress =>
                            "Varies WithMemory Address",
                        CacheOperationalMode::Unknown => UNKNOWN,
                    }
                );
                println!(
                    "\tLocation: {}",
                    match cache_configuration.location() {
                        CacheLocation::Internal => "Internal",
                        CacheLocation::External => "External",
                        CacheLocation::Reserved => "Reserved",
                        CacheLocation::Unknown => UNKNOWN,
                    }
                );
            }
            dmi_cache_size(
                "Installed Size",
                data.installed_size(),
                data.installed_cache_size_2(),
            );
            dmi_cache_size(
                "Maximum Size",
                data.maximum_cache_size(),
                data.maximum_cache_size_2(),
            );

            if let Some(supported_sram_type) = data.supported_sram_type() {
                dmi_cache_types("Supported SRAM Types", supported_sram_type, false);
            }
            if let Some(current_sram_type) = data.current_sram_type() {
                dmi_cache_types("Installed SRAM Type", current_sram_type, true);
            }
            if let Some(cache_speed) = data.cache_speed() {
                dmi_memory_module_speed("Speed", cache_speed);
            }
            if let Some(error_correction_type) = data.error_correction_type() {
                println!(
                    "\tError Correction Type: {}",
                    dmi_cache_ec_type(error_correction_type)
                );
            }
            if let Some(system_cache_type) = data.system_cache_type() {
                println!("\tSystem Type: {}", dmi_cache_type(system_cache_type));
            }
            if let Some(associativity) = data.associativity() {
                println!(
                    "\tAssociativity: {}",
                    dmi_cache_associativity(associativity)
                );
            }
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
            for oem_string in data.oem_strings().into_iter().enumerate() {
                println!("\tString {}: {}", oem_string.0, oem_string.1);
            }
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
            if let Some(location) = data.location() {
                println!("\tLocation: {}", dmi_memory_array_location(location));
            }
            if let Some(usage) = data.usage() {
                println!("\tUse: {}", dmi_memory_array_use(usage));
            }
            if let Some(memory_error_correction) = data.memory_error_correction() {
                println!(
                    "\tError Correction Type: {}",
                    dmi_memory_array_ec_type(memory_error_correction)
                );
            }
            if let Some(maximum_capacity) = data.maximum_capacity() {
                const MAXIMUM_CAPACITY: &str = "Maximum Capacity";
                match maximum_capacity {
                    MaximumMemoryCapacity::Kilobytes(capacity_kb) => {
                        dmi_print_memory_size(MAXIMUM_CAPACITY, capacity_kb as u64, true)
                    }
                    MaximumMemoryCapacity::SeeExtendedMaximumCapacity => {
                        match data.extended_maximum_capacity() {
                            Some(capacity_bytes) => {
                                dmi_print_memory_size(MAXIMUM_CAPACITY, capacity_bytes, false)
                            }
                            None => println!("\t{}: {}", MAXIMUM_CAPACITY, UNKNOWN),
                        }
                    }
                }
            }
            if !quiet {
                if let Some(memory_error_information_handle) =
                    data.memory_error_information_handle()
                {
                    print!("\tError Information Handle: ");
                    match memory_error_information_handle {
                        0xFFFE => println!("Not Provided"),
                        0xFFFF => println!("No Error"),
                        val => println!("{:#06X}", val),
                    }

                    // TODO: Use this method instead once the library returns a Handle
                    // dmi_memory_array_error_handle(memory_error_information_handle);
                }
            }
            if let Some(number_of_memory_devices) = data.number_of_memory_devices() {
                println!("\tNumber Of Devices: {}", number_of_memory_devices);
            }
        }
        DefinedStruct::MemoryDevice(data) => {
            println!("Memory Device");
            if !quiet {
                if let Some(physical_memory_array_handle) = data.physical_memory_array_handle() {
                    println!("\tArray Handle: {:#06X}", *physical_memory_array_handle);
                }
                if let Some(memory_error_information_handle) =
                    data.memory_error_information_handle()
                {
                    dmi_memory_array_error_handle(memory_error_information_handle);
                }
            }
            if let Some(total_width) = data.total_width() {
                dmi_memory_device_width("Total Width", total_width);
            }
            if let Some(data_width) = data.data_width() {
                dmi_memory_device_width("Data Width", data_width);
            }
            let mut module_present = false;
            match (data.size(), data.extended_size()) {
                (Some(size), None) => {
                    module_present = size != MemorySize::NotInstalled;
                    dmi_memory_device_size(size);
                }
                (Some(size1), Some(size2)) => match size1 == MemorySize::SeeExtendedSize {
                    true => {
                        print!("\tSize: ");
                        let masked_size = size2 & 0x7FFFFFFF;
                        if masked_size == 0 {
                            println!("0 MB");
                        } else {
                            module_present = true;
                            match (31 - masked_size.leading_zeros()) / 10 {
                                0 => println!("{} MB", masked_size),
                                1 => println!("{} GB", masked_size >> 10),
                                _ => println!("{} TB", masked_size >> 20),
                            }
                        }
                    }
                    false => {
                        module_present = size1 != MemorySize::NotInstalled;
                        dmi_memory_device_size(size1);
                    }
                },
                _ => (),
            }
            if let Some(form_factor) = data.form_factor() {
                println!(
                    "\tForm Factor: {}",
                    dmi_memory_device_form_factor(form_factor)
                );
            }
            if let Some(device_set) = data.device_set() {
                dmi_memory_device_set(device_set);
            }
            if let Some(device_locator) = data.device_locator() {
                println!("\tLocator: {}", device_locator);
            }
            if let Some(bank_locator) = data.bank_locator() {
                println!("\tBank Locator: {}", bank_locator);
            }
            if let Some(memory_type) = data.memory_type() {
                println!("\tType: {}", dmi_memory_device_type(memory_type));
            }
            if let Some(type_detail) = data.type_detail() {
                dmi_memory_device_type_detail(type_detail);
            }
            // If a module is present, the remaining fields are relevant
            if module_present {
                dmi_memory_device_speed("Speed", data.speed(), data.extended_speed());
                if let Some(manufacturer) = data.manufacturer() {
                    println!("\tManufacturer: {}", manufacturer);
                }
                if let Some(serial_number) = data.serial_number() {
                    println!("\tSerial Number: {}", serial_number);
                }
                if let Some(asset_tag) = data.asset_tag() {
                    println!("\tAsset Tag: {}", asset_tag);
                }
                if let Some(part_number) = data.part_number() {
                    println!("\tPart Number: {}", part_number);
                }
                if let Some(attributes) = data.attributes() {
                    print!("\tRank: ");
                    match attributes & 0x0F == 0 {
                        true => println!("{}", UNKNOWN),
                        false => println!("{}", attributes),
                    }
                }
                dmi_memory_device_speed(
                    "Configured Speed",
                    data.configured_memory_speed(),
                    data.extended_configured_memory_speed(),
                );
                if let Some(minimum_voltage) = data.minimum_voltage() {
                    dmi_memory_voltage_value("Minimum Voltage", minimum_voltage);
                }
                if let Some(maximum_voltage) = data.maximum_voltage() {
                    dmi_memory_voltage_value("Maximum Voltage", maximum_voltage);
                }
                if let Some(configured_voltage) = data.configured_voltage() {
                    dmi_memory_voltage_value("Configured Voltage", configured_voltage);
                }
                if let Some(memory_technology) = data.memory_technology() {
                    dmi_memory_technology(memory_technology);
                }
                if let Some(memory_operating_mode_capability) =
                    data.memory_operating_mode_capability()
                {
                    dmi_memory_operating_mode_capability(memory_operating_mode_capability);
                }
                if let Some(firmware_version) = data.firmware_version() {
                    println!("\tFirmware Version: {}", firmware_version);
                }
                if let Some(module_manufacturer_id) = data.module_manufacturer_id() {
                    dmi_memory_manufacturer_id("Module Manufacturer ID", module_manufacturer_id);
                }
                if let Some(module_product_id) = data.module_product_id() {
                    dmi_memory_product_id("Module Product ID", module_product_id);
                }
                if let Some(memory_subsystem_controller_manufacturer_id) =
                    data.memory_subsystem_controller_manufacturer_id()
                {
                    dmi_memory_manufacturer_id(
                        "Memory Subsystem Controller Manufacturer ID",
                        memory_subsystem_controller_manufacturer_id,
                    );
                }
                if let Some(memory_subsystem_controller_product_id) =
                    data.memory_subsystem_controller_product_id()
                {
                    dmi_memory_product_id(
                        "Memory Subsystem Controller Product ID",
                        memory_subsystem_controller_product_id,
                    );
                }
                if let Some(non_volatile_size) = data.non_volatile_size() {
                    dmi_memory_size("Non-Volatile Size", non_volatile_size);
                }
                if let Some(volatile_size) = data.volatile_size() {
                    dmi_memory_size("Volatile Size", volatile_size);
                }
                if let Some(cache_size) = data.cache_size() {
                    dmi_memory_size("Cache Size", cache_size);
                }
                if let Some(logical_size) = data.logical_size() {
                    dmi_memory_size("Logical Size", logical_size);
                }
            }
        }
        DefinedStruct::MemoryErrorInformation32Bit(data) => {
            println!("32-bit Memory Error Information");
            if let Some(error_type) = data.error_type() {
                println!("\tType: {}", dmi_memory_error_type(error_type));
            }
            if let Some(error_granularity) = data.error_granularity() {
                println!(
                    "\tGranularity: {}",
                    dmi_memory_error_granularity(error_granularity)
                );
            }
            if let Some(error_operation) = data.error_operation() {
                println!(
                    "\tOperation: {}",
                    dmi_memory_error_operation(error_operation)
                );
            }
            if let Some(vendor_syndrome) = data.vendor_syndrome() {
                dmi_memory_error_syndrome(vendor_syndrome);
            }
            if let Some(memory_array_error_address) = data.memory_array_error_address() {
                dmi_32bit_memory_error_address("Memory Array Address", memory_array_error_address);
            }
            if let Some(device_error_address) = data.device_error_address() {
                dmi_32bit_memory_error_address("Device Address", device_error_address);
            }
            if let Some(error_resolution) = data.error_resolution() {
                dmi_32bit_memory_error_address("Resolution", error_resolution);
            }
        }
        DefinedStruct::MemoryArrayMappedAddress(data) => {
            println!("Memory Array Mapped Address");

            dmi_starting_ending_addresses(
                data.starting_address(),
                data.extended_starting_address(),
                data.ending_address(),
                data.extended_ending_address(),
            );

            if !quiet {
                if let Some(handle) = data.physical_memory_array_handle() {
                    println!("\tPhysical Array Handle: {:#06X}", *handle);
                }
            }
            if let Some(partition_width) = data.partition_width() {
                println!("\tPartition Width: {}", partition_width);
            }
        }
        DefinedStruct::MemoryDeviceMappedAddress(data) => {
            println!("Memory Device Mapped Address");

            dmi_starting_ending_addresses(
                data.starting_address(),
                data.extended_starting_address(),
                data.ending_address(),
                data.extended_ending_address(),
            );

            if !quiet {
                if let Some(memory_device_handle) = data.memory_device_handle() {
                    println!("\tPhysical Device Handle: {:#06X}", *memory_device_handle);
                }
                if let Some(memory_array_mapped_address_handle) =
                    data.memory_array_mapped_address_handle()
                {
                    println!(
                        "\tMemory Array Mapped Address Handle: {:#06X}",
                        *memory_array_mapped_address_handle
                    );
                }
            }
            if let Some(partition_row_position) = data.partition_row_position() {
                dmi_mapped_address_row_position(partition_row_position);
            }
            if let Some(interleave_position) = data.interleave_position() {
                dmi_mapped_address_interleave_position(interleave_position);
            }
            if let Some(interleaved_data_depth) = data.interleaved_data_depth() {
                dmi_mapped_address_interleaved_data_depth(interleaved_data_depth);
            }
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
            if let Some(hardware_security_settings) = data.hardware_security_settings() {
                println!(
                    "\tPower-On Password Status: {}",
                    dmi_hardware_security_status(
                        hardware_security_settings.power_on_password_status
                    )
                );
                println!(
                    "\tKeyboard Password Status: {}",
                    dmi_hardware_security_status(
                        hardware_security_settings.keyboard_password_status
                    )
                );
                println!(
                    "\tAdministrator Password Status: {}",
                    dmi_hardware_security_status(
                        hardware_security_settings.administrator_password_status
                    )
                );
                println!(
                    "\tFront Panel Reset Status: {}",
                    dmi_hardware_security_status(
                        hardware_security_settings.front_panel_reset_status
                    )
                );
            }
        }
        DefinedStruct::SystemPowerControls(data) => {
            println!("System Power Controls");
            match (
                data.next_scheduled_power_on_month(),
                data.next_scheduled_power_on_day_of_month(),
                data.next_scheduled_power_on_hour(),
                data.next_scheduled_power_on_minute(),
                data.next_scheduled_power_on_second(),
            ) {
                (Some(month), Some(day), Some(hour), Some(minute), Some(second)) => {
                    let mut time = String::new();
                    match dmi_bcd_range(month, 0x0, 0x12) {
                        true => time.push_str(format!("{:04X}", month).as_str()),
                        false => time.push_str("*"),
                    }
                    match dmi_bcd_range(day, 0x0, 0x31) {
                        true => time.push_str(format!("-{:04X}", day).as_str()),
                        false => time.push_str("-*"),
                    }
                    match dmi_bcd_range(hour, 0x0, 0x23) {
                        true => time.push_str(format!(" {:04X}", hour).as_str()),
                        false => time.push_str(" *"),
                    }
                    match dmi_bcd_range(minute, 0x0, 0x59) {
                        true => time.push_str(format!(":{:04X}", minute).as_str()),
                        false => time.push_str(":*"),
                    }
                    match dmi_bcd_range(second, 0x0, 0x59) {
                        true => time.push_str(format!(":{:04X}", second).as_str()),
                        false => time.push_str(":*"),
                    }
                    println!("\tNext Scheduled Power-on: {}", time);
                }
                _ => (),
            }
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
        DefinedStruct::Inactive(_) => {
            println!("Inactive");
        }
        DefinedStruct::EndOfTable(_) => {
            println!("End Of Table");
        }
        DefinedStruct::Undefined(data) => {
            if data.parts().header.struct_type() >= 128 {
                println!("OEM-specific");
            } else {
                println!("{}", UNKNOWN);
            }
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
        ChassisState::Other => OTHER.to_string(),
        ChassisState::Unknown => UNKNOWN.to_string(),
        ChassisState::Safe => "Safe".to_string(),
        ChassisState::Warning => "Warning".to_string(),
        ChassisState::Critical => "Critical".to_string(),
        ChassisState::NonRecoverable => "Non-recoverable".to_string(),
        ChassisState::None => format!("{} ({})", OUT_OF_SPEC, state.raw),
    }
}

fn dmi_processor_type(processor_type: ProcessorTypeData) -> String {
    match processor_type.value {
        ProcessorType::Other => OTHER.to_string(),
        ProcessorType::Unknown => UNKNOWN.to_string(),
        ProcessorType::CentralProcessor => "Central Processor".to_string(),
        ProcessorType::MathProcessor => "Math Processor".to_string(),
        ProcessorType::DspProcessor => "DSP Processor".to_string(),
        ProcessorType::VideoProcessor => "VideoProcessor".to_string(),
        ProcessorType::None => format!("{} ({})", OUT_OF_SPEC, processor_type.raw),
    }
}
fn dmi_processor_family(processor_family: ProcessorFamily, raw: u16) -> String {
    let print = match processor_family {
        ProcessorFamily::Other => OTHER,
        ProcessorFamily::Unknown => UNKNOWN,
        ProcessorFamily::I8086 => "8086",
        ProcessorFamily::I80286 => "80286",
        ProcessorFamily::Intel386Processor => "80386",
        ProcessorFamily::Intel486Processor => "80486",
        ProcessorFamily::I8087 => "8087",
        ProcessorFamily::I80287 => "80287",
        ProcessorFamily::I80387 => "80387",
        ProcessorFamily::I80487 => "80487",
        ProcessorFamily::IntelPentiumProcessor => "Pentium",
        ProcessorFamily::PentiumProProcessor => "Pentium Pro",
        ProcessorFamily::PentiumIIProcessor => "Pentium II",
        ProcessorFamily::PentiumprocessorwithMMXtechnology => "Pentium MMX",
        ProcessorFamily::IntelCeleronProcessor => "Celeron",
        ProcessorFamily::PentiumIIXeonProcessor => "Pentium II Xeon",
        ProcessorFamily::PentiumIIIProcessor => "Pentium II",
        ProcessorFamily::M1Family => "M1",
        ProcessorFamily::M2Family => "M2",
        ProcessorFamily::IntelCeleronMProcessor => "Celeron M",
        ProcessorFamily::IntelPentium4HTProcessor => "Pentium 4 HT",
        ProcessorFamily::AMDDuronProcessorFamily => "Duron",
        ProcessorFamily::K5Family => "K5",
        ProcessorFamily::K6Family => "K6",
        ProcessorFamily::K62 => "K6-2",
        ProcessorFamily::K63 => "K6-3",
        ProcessorFamily::AMDAthlonProcessorFamily => "Athlon",
        ProcessorFamily::AMD29000Family => "AMD29000",
        ProcessorFamily::K62Plus => "K6-2+",
        ProcessorFamily::PowerPCFamily => "Power PC",
        ProcessorFamily::PowerPC601 => "Power PC 601",
        ProcessorFamily::PowerPC603 => "Power PC 603",
        ProcessorFamily::PowerPC603Plus => "Power PC 603+",
        ProcessorFamily::PowerPC604 => "Power PC 604",
        ProcessorFamily::PowerPC620 => "Power PC 620",
        ProcessorFamily::PowerPCx704 => "Power PC x704",
        ProcessorFamily::PowerPC750 => "Power PC 750",
        ProcessorFamily::IntelCoreDuoProcessor => "Core Duo",
        ProcessorFamily::IntelCoreDuomobileProcessor => "Core Duo Mobile",
        ProcessorFamily::IntelCoreSolomobileProcessor => "Core Solo Mobile",
        ProcessorFamily::IntelAtomProcessor => "Atom",
        ProcessorFamily::IntelCoreMProcessor => "Core M",
        ProcessorFamily::IntelCorem3Processor => "Core m3",
        ProcessorFamily::IntelCorem5Processor => "Core m5",
        ProcessorFamily::IntelCorem7Processor => "Core m7",
        ProcessorFamily::AlphaFamily => "Alpha",
        ProcessorFamily::Alpha21064 => "Alpha 21064",
        ProcessorFamily::Alpha21066 => "Alpha 21066",
        ProcessorFamily::Alpha21164 => "Alpha 21164",
        ProcessorFamily::Alpha21164PC => "Alpha 21164PC",
        ProcessorFamily::Alpha21164a => "Alpha 21164a",
        ProcessorFamily::Alpha21264 => "Alpha 21264",
        ProcessorFamily::Alpha21364 => "Alpha 21364",
        ProcessorFamily::AMDTurionIIUltraDualCoreMobileMProcessorFamily => {
            "Turion II Ultra Dual-Core Mobile M"
        }
        ProcessorFamily::AMDTurionIIDualCoreMobileMProcessorFamily => {
            "Turion II Dual-Core Mobile M"
        }
        ProcessorFamily::AMDAthlonIIDualCoreMProcessorFamily => "Athlon II Dual-Core M",
        ProcessorFamily::AMDOpteron6100SeriesProcessor => "Opteron 6100",
        ProcessorFamily::AMDOpteron4100SeriesProcessor => "Opteron 4100",
        ProcessorFamily::AMDOpteron6200SeriesProcessor => "Opteron 6200",
        ProcessorFamily::AMDOpteron4200SeriesProcessor => "Opteron 4200",
        ProcessorFamily::AMDFXSeriesProcessor => "FX",
        ProcessorFamily::MIPSFamily => "MIPS",
        ProcessorFamily::MIPSR4000 => "MIPS R4000",
        ProcessorFamily::MIPSR4200 => "MIPS R4200",
        ProcessorFamily::MIPSR4400 => "MIPS R4400",
        ProcessorFamily::MIPSR4600 => "MIPS R4600",
        ProcessorFamily::MIPSR10000 => "MIPS R10000",
        ProcessorFamily::AMDCSeriesProcessor => "C-Series",
        ProcessorFamily::AMDESeriesProcessor => "E-Series",
        ProcessorFamily::AMDASeriesProcessor => "A-Series",
        ProcessorFamily::AMDGSeriesProcessor => "G-Series",
        ProcessorFamily::AMDZSeriesProcessor => "Z-Series",
        ProcessorFamily::AMDRSeriesProcessor => "R-Series",
        ProcessorFamily::AMDOpteron4300SeriesProcessor => "Opteron 4300",
        ProcessorFamily::AMDOpteron6300SeriesProcessor => "Opteron 6300",
        ProcessorFamily::AMDOpteron3300SeriesProcessor => "Opteron 3300",
        ProcessorFamily::AMDFireProSeriesProcessor => "FirePro",
        ProcessorFamily::SPARCFamily => "SPARC",
        ProcessorFamily::SuperSPARC => "SuperSPARC",
        ProcessorFamily::MicroSparcii => "MicroSPARC II",
        ProcessorFamily::MicroSparciiep => "MicroSPARC IIep",
        ProcessorFamily::UltraSPARC => "UltraSPARC",
        ProcessorFamily::UltraSPARCII => "UltraSPARC II",
        ProcessorFamily::UltraSPARCIii => "UltraSPARC IIi",
        ProcessorFamily::UltraSPARCIII => "UltraSPARC III",
        ProcessorFamily::UltraSPARCIIIi => "UltraSPARC IIIi",
        ProcessorFamily::M68040Family => "68040",
        ProcessorFamily::M68xxx => "68xxx",
        ProcessorFamily::M68000 => "68000",
        ProcessorFamily::M68010 => "68010",
        ProcessorFamily::M68020 => "68020",
        ProcessorFamily::M68030 => "68030",
        ProcessorFamily::AMDAthlonX4QuadCoreProcessorFamily => "Athlon X4",
        ProcessorFamily::AMDOpteronX1000SeriesProcessor => "Opteron X1000",
        ProcessorFamily::AMDOpteronX2000SeriesAPU => "Opteron X2000",
        ProcessorFamily::AMDOpteronASeriesProcessor => "Opteron A-Series",
        ProcessorFamily::AMDOpteronX3000SeriesAPU => "Opteron X3000",
        ProcessorFamily::AMDZenProcessorFamily => "Zen",
        ProcessorFamily::HobbitFamily => "Hobbit",
        ProcessorFamily::CrusoeTM5000Family => "Crusoe TM5000",
        ProcessorFamily::CrusoeTM3000Family => "Crusoe TM3000",
        ProcessorFamily::EfficeonTM8000Family => "Efficeon TM8000",
        ProcessorFamily::Weitek => "Weitek",
        ProcessorFamily::Itaniumprocessor => "Itanium",
        ProcessorFamily::AMDAthlon64ProcessorFamily => "Athlon 64",
        ProcessorFamily::AMDOpteronProcessorFamily => "Opteron",
        ProcessorFamily::AMDSempronProcessorFamily => "Sempron",
        ProcessorFamily::AMDTurion64MobileTechnology => "Turion 64",
        ProcessorFamily::DualCoreAMDOpteronProcessorFamily => "Dual-Core Opteron",
        ProcessorFamily::AMDAthlon64X2DualCoreProcessorFamily => "Athlon 64 X2",
        ProcessorFamily::AMDTurion64X2MobileTechnology => "Turion 64 X2",
        ProcessorFamily::QuadCoreAMDOpteronProcessorFamily => "Quad-Core Opteron",
        ProcessorFamily::ThirdGenerationAMDOpteronProcessorFamily => "Third-Generation Opteron",
        ProcessorFamily::AMDPhenomFXQuadCoreProcessorFamily => "Phenom FX",
        ProcessorFamily::AMDPhenomX4QuadCoreProcessorFamily => "Phenom X4",
        ProcessorFamily::AMDPhenomX2DualCoreProcessorFamily => "Phenom X2",
        ProcessorFamily::AMDAthlonX2DualCoreProcessorFamily => "Athlon X2",
        ProcessorFamily::PARISCFamily => "PA-RISC",
        ProcessorFamily::PARISC8500 => "PA-RISC 8500",
        ProcessorFamily::PARISC8000 => "PA-RISC 8000",
        ProcessorFamily::PARISC7300LC => "PA-RISC 7300LC",
        ProcessorFamily::PARISC7200 => "PA-RISC 7200",
        ProcessorFamily::PARISC7100LC => "PA-RISC 7100LC",
        ProcessorFamily::PARISC7100 => "PA-RISC 7100",
        ProcessorFamily::V30Family => "V30",
        ProcessorFamily::QuadCoreIntelXeonProcessor3200Series => "Quad-Core Xeon 3200",
        ProcessorFamily::DualCoreIntelXeonProcessor3000Series => "Dual-Core Xeon 3000",
        ProcessorFamily::QuadCoreIntelXeonProcessor5300Series => "Quad-Core Xeon 5300",
        ProcessorFamily::DualCoreIntelXeonProcessor5100Series => "Dual-Core Xeon 5100",
        ProcessorFamily::DualCoreIntelXeonProcessor5000Series => "Dual-Core Xeon 5000",
        ProcessorFamily::DualCoreIntelXeonProcessorLV => "Dual-Core Xeon LV",
        ProcessorFamily::DualCoreIntelXeonProcessorULV => "Dual-Core Xeon ULV",
        ProcessorFamily::DualCoreIntelXeonProcessor7100Series => "Dual-Core Xeon 7100",
        ProcessorFamily::QuadCoreIntelXeonProcessor5400Series => "Quad-Core Xeon 5400",
        ProcessorFamily::QuadCoreIntelXeonProcessor => "Quad-Core Xeon",
        ProcessorFamily::DualCoreIntelXeonProcessor5200Series => "Dual-Core Xeon 5200",
        ProcessorFamily::DualCoreIntelXeonProcessor7200Series => "Dual-Core Xeon 7200",
        ProcessorFamily::QuadCoreIntelXeonProcessor7300Series => "Quad-Core Xeon 7300",
        ProcessorFamily::QuadCoreIntelXeonProcessor7400Series => "Quad-Core Xeon 7400",
        ProcessorFamily::MultiCoreIntelXeonProcessor7400Series => "Multi-Core Xeon 7400",
        ProcessorFamily::PentiumIIIXeonProcessor => "Pentium III Xeon",
        ProcessorFamily::PentiumIIIProcessorwithIntelSpeedStepTechnology => "Pentium III Speedstep",
        ProcessorFamily::Pentium4Processor => "Pentium 4",
        ProcessorFamily::IntelXeonProcessor => "Xeon",
        ProcessorFamily::AS400Family => "AS400",
        ProcessorFamily::IntelXeonProcessorMP => "Xeon MP",
        ProcessorFamily::AMDAthlonXPProcessorFamily => "Athlon XP",
        ProcessorFamily::AMDAthlonMPProcessorFamily => "Athlon MP",
        ProcessorFamily::IntelItanium2Processor => "Itanium 2",
        ProcessorFamily::IntelPentiumMProcessor => "Pentium M",
        ProcessorFamily::IntelCeleronDProcessor => "Celeron D",
        ProcessorFamily::IntelPentiumDProcessor => "Pentium D",
        ProcessorFamily::IntelPentiumProcessorExtremeEdition => "Pentium EE",
        ProcessorFamily::IntelCoreSoloProcessor => "Core Solo",
        ProcessorFamily::IntelCore2DuoProcessor => "Core 2 Duo",
        ProcessorFamily::IntelCore2SoloProcessor => "Core 2 Solo",
        ProcessorFamily::IntelCore2ExtremeProcessor => "Core 2 Extreme",
        ProcessorFamily::IntelCore2QuadProcessor => "Core 2 Quad",
        ProcessorFamily::IntelCore2ExtremeMobileProcessor => "Core 2 Extreme Mobile",
        ProcessorFamily::IntelCore2DuoMobileProcessor => "Core 2 Duo Mobile",
        ProcessorFamily::IntelCore2SoloMobileProcessor => "Core 2 Solo Mobile",
        ProcessorFamily::IntelCorei7Processor => "Core i7",
        ProcessorFamily::DualCoreIntelCeleronProcessor => "Dual-Core Celeron",
        ProcessorFamily::IBM390Family => "IBM390",
        ProcessorFamily::G4 => "G4",
        ProcessorFamily::G5 => "G5",
        ProcessorFamily::ESA390G6 => "ESA/390 G6",
        ProcessorFamily::ZArchitecturebase => "z/Architecture",
        ProcessorFamily::IntelCorei5processor => "Core i5",
        ProcessorFamily::IntelCorei3processor => "Core i3",
        ProcessorFamily::IntelCorei9processor => "Core i9",
        ProcessorFamily::VIAC7MProcessorFamily => "C7-M",
        ProcessorFamily::VIAC7DProcessorFamily => "C7-D",
        ProcessorFamily::VIAC7ProcessorFamily => "C7",
        ProcessorFamily::VIAEdenProcessorFamily => "Eden",
        ProcessorFamily::MultiCoreIntelXeonProcessor => "Multi-Core Xeon",
        ProcessorFamily::DualCoreIntelXeonProcessor3xxxSeries => "Dual-Core Xeon 3xxx",
        ProcessorFamily::QuadCoreIntelXeonProcessor3xxxSeries => "Quad-Core Xeon 3xxx",
        ProcessorFamily::VIANanoProcessorFamily => "Nano",
        ProcessorFamily::DualCoreIntelXeonProcessor5xxxSeries => "Dual-Core Xeon 5xxx",
        ProcessorFamily::QuadCoreIntelXeonProcessor5xxxSeries => "Quad-Core Xeon 5xxx",
        ProcessorFamily::DualCoreIntelXeonProcessor7xxxSeries => "Dual-Core Xeon 7xxx",
        ProcessorFamily::QuadCoreIntelXeonProcessor7xxxSeries => "Quad-Core Xeon 7xxx",
        ProcessorFamily::MultiCoreIntelXeonProcessor7xxxSeries => "Multi-Core Xeon 7xxx",
        ProcessorFamily::MultiCoreIntelXeonProcessor3400Series => "Multi-Core Xeon 3400",
        ProcessorFamily::AMDOpteron3000SeriesProcessor => "Opteron 3000",
        ProcessorFamily::AMDSempronIIProcessor => "Sempron II",
        ProcessorFamily::EmbeddedAMDOpteronQuadCoreProcessorFamily => "Embedded Opteron Quad-Core",
        ProcessorFamily::AMDPhenomTripleCoreProcessorFamily => "Phenom Triple-Core",
        ProcessorFamily::AMDTurionUltraDualCoreMobileProcessorFamily => {
            "Turion Ultra Dual-Core Mobile"
        }
        ProcessorFamily::AMDTurionDualCoreMobileProcessorFamily => "Turion Dual-Core Mobile",
        ProcessorFamily::AMDAthlonDualCoreProcessorFamily => "Athlon Dual-Core",
        ProcessorFamily::AMDSempronSIProcessorFamily => "Sempron SI",
        ProcessorFamily::AMDPhenomIIProcessorFamily => "Phenom II",
        ProcessorFamily::AMDAthlonIIProcessorFamily => "Athlon II",
        ProcessorFamily::SixCoreAMDOpteronProcessorFamily => "Six-Core Opteron",
        ProcessorFamily::AMDSempronMProcessorFamily => "Sempron M",
        ProcessorFamily::I860 => "i860",
        ProcessorFamily::I960 => "i960",
        ProcessorFamily::SeeProcessorFamily2 => "See Processor Family #2",
        ProcessorFamily::ARMv7 => "ARMv7",
        ProcessorFamily::ARMv8 => "ARMv8",
        ProcessorFamily::SH3 => "SH-3",
        ProcessorFamily::SH4 => "SH-4",
        ProcessorFamily::ARM => "ARM",
        ProcessorFamily::StrongARM => "StrongARM",
        ProcessorFamily::Cyrix6x86 => "6x86",
        ProcessorFamily::MediaGX => "MediaGX",
        ProcessorFamily::MII => "MII",
        ProcessorFamily::WinChip => "WinChip",
        ProcessorFamily::DSP => "DSP",
        ProcessorFamily::VideoProcessor => "Video Processor",
        ProcessorFamily::RISCVRV32 => "RV32",
        ProcessorFamily::RISCVRV64 => "RV64",
        ProcessorFamily::RISCVRV128 => "RV128",
        ProcessorFamily::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, raw),
        false => print.to_string(),
    }
}

fn dmi_processor_upgrade(processor_upgrade: ProcessorUpgradeData) -> String {
    let print = match processor_upgrade.value {
        ProcessorUpgrade::Other => OTHER,
        ProcessorUpgrade::Unknown => UNKNOWN,
        ProcessorUpgrade::DaughterBoard => "Daughter Board",
        ProcessorUpgrade::ZIFSocket => "ZIF Socket",
        ProcessorUpgrade::ReplaceablePiggyBack => "Replaceable Piggy Back",
        ProcessorUpgrade::NoUpgrade => NONE,
        ProcessorUpgrade::LIFSocket => "LIF Socket",
        ProcessorUpgrade::Slot1 => "Slot 1",
        ProcessorUpgrade::Slot2 => "Slot 2",
        ProcessorUpgrade::PinSocket370 => "370-pin Socket",
        ProcessorUpgrade::SlotA => "Slot A",
        ProcessorUpgrade::SlotM => "Slot M",
        ProcessorUpgrade::Socket423 => "Socket 423",
        ProcessorUpgrade::SocketASocket462 => "Socket A (Socket 462)",
        ProcessorUpgrade::Socket478 => "Socket 478",
        ProcessorUpgrade::Socket754 => "Socket 754",
        ProcessorUpgrade::Socket940 => "Socket 940",
        ProcessorUpgrade::Socket939 => "Socket 939",
        ProcessorUpgrade::SocketmPGA604 => "Socket mPGA604",
        ProcessorUpgrade::SocketLGA771 => "Socket LGA771",
        ProcessorUpgrade::SocketLGA775 => "Socket LGA775",
        ProcessorUpgrade::SocketS1 => "Socket S1",
        ProcessorUpgrade::SocketAM2 => "Socket AM2",
        ProcessorUpgrade::SocketF1207 => "Socket F (1207)",
        ProcessorUpgrade::SocketLGA1366 => "Socket LGA1366",
        ProcessorUpgrade::SocketG34 => "Socket G34",
        ProcessorUpgrade::SocketAM3 => "Socket AM3",
        ProcessorUpgrade::SocketC32 => "Socket C32",
        ProcessorUpgrade::SocketLGA1156 => "Socket LGA1156",
        ProcessorUpgrade::SocketLGA1567 => "Socket LGA1567",
        ProcessorUpgrade::SocketPGA988A => "Socket PGA988A",
        ProcessorUpgrade::SocketBGA1288 => "Socket BGA1288",
        ProcessorUpgrade::SocketrPGA988B => "Socket rPGA988B",
        ProcessorUpgrade::SocketBGA1023 => "Socket BGA1023",
        ProcessorUpgrade::SocketBGA1224 => "Socket BGA1224",
        ProcessorUpgrade::SocketLGA1155 => "Socket BGA1155",
        ProcessorUpgrade::SocketLGA1356 => "Socket LGA1356",
        ProcessorUpgrade::SocketLGA2011 => "Socket LGA2011",
        ProcessorUpgrade::SocketFS1 => "Socket FS1",
        ProcessorUpgrade::SocketFS2 => "Socket FS2",
        ProcessorUpgrade::SocketFM1 => "Socket FM1",
        ProcessorUpgrade::SocketFM2 => "Socket FM2",
        ProcessorUpgrade::SocketLGA2011_3 => "Socket LGA2011-3",
        ProcessorUpgrade::SocketLGA1356_3 => "Socket LGA1356-3",
        ProcessorUpgrade::SocketLGA1150 => "Socket LGA1150",
        ProcessorUpgrade::SocketBGA1168 => "Socket BGA1168",
        ProcessorUpgrade::SocketBGA1234 => "Socket BGA1234",
        ProcessorUpgrade::SocketBGA1364 => "Socket BGA1364",
        ProcessorUpgrade::SocketAM4 => "Socket AM4",
        ProcessorUpgrade::SocketLGA1151 => "Socket LGA1151",
        ProcessorUpgrade::SocketBGA1356 => "Socket BGA1356",
        ProcessorUpgrade::SocketBGA1440 => "Socket BGA1440",
        ProcessorUpgrade::SocketBGA1515 => "Socket BGA1515",
        ProcessorUpgrade::SocketLGA3647_1 => "Socket LGA3647-1",
        ProcessorUpgrade::SocketSP3 => "Socket SP3",
        ProcessorUpgrade::SocketSP3r23 => "Socket SP3r2",
        ProcessorUpgrade::SocketLGA2066 => "Socket LGA2066",
        ProcessorUpgrade::SocketBGA1392 => "Socket BGA1392",
        ProcessorUpgrade::SocketBGA1510 => "Socket BGA1510",
        ProcessorUpgrade::SocketBGA1528 => "Socket BGA1528",
        ProcessorUpgrade::SocketLGA4189 => "Socket LGA4189",
        ProcessorUpgrade::SocketLGA1200 => "Socket LGA1200",
        ProcessorUpgrade::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, processor_upgrade.raw),
        false => print.to_string(),
    }
}
fn dmi_processor_cache(label: &str, handle: Handle, level: &str, version: Option<SMBiosVersion>) {
    print!("\t{}: ", label);
    match *handle == 0xFFFF {
        true => {
            if let Some(ver) = version {
                match ver >= SMBiosVersion::new(2, 3, 0) {
                    true => println!("Not Provided"),
                    false => println!("No {} Cache", level),
                }
            }
        }
        false => println!("{:#06X}", *handle),
    }
}
fn dmi_processor_characteristics(characteristics: ProcessorCharacteristics) {
    if characteristics.raw & 0xFC == 0 {
        println!("\tCharacteristics: None");
    } else {
        println!("\tCharacteristics: ");
        if characteristics.unknown() {
            println!("\t\tUnknown");
        }
        if characteristics.bit_64capable() {
            println!("\t\t64-bit capable");
        }
        if characteristics.multi_core() {
            println!("\t\tMulti-core");
        }
        if characteristics.hardware_thread() {
            println!("\t\tHardware Thread");
        }
        if characteristics.execute_protection() {
            println!("\t\tExecute Protection");
        }
        if characteristics.enhanced_virtualization() {
            println!("\t\tEnhanced Virtualization");
        }
        if characteristics.power_performance_control() {
            println!("\t\tPower/Performance Control");
        }
        if characteristics.bit_128capable() {
            println!("\t\t128-bit Capable");
        }
        if characteristics.arm_64soc_id() {
            println!("\t\tArm64 SoC ID");
        }
    }
}
fn dmi_processor_id(data: &SMBiosProcessorInformation<'_>) {
    if let Some(p) = data.processor_id() {
        println!(
            "\tID: {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}",
            p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]
        );

        let option_family = match (data.processor_family(), data.processor_family_2()) {
            (Some(processor_family), None) => {
                Some((processor_family.value, processor_family.raw as u16))
            }
            (Some(_), Some(processor_family_2)) => {
                Some((processor_family_2.value, processor_family_2.raw))
            }
            _ => None,
        };

        match option_family {
            Some(family) => {
                let mut sig = 0;

                if family.0 == ProcessorFamily::Intel386Processor {
                    let dx = u16::from_le_bytes(p[0..=1].try_into().expect("u16 is 2 bytes"));
                    println!(
                        "\tSignature: Type {}, Family {}, Major Stepping {}, Minor Stepping {}",
                        dx >> 12,
                        (dx >> 8) & 0xF,
                        (dx >> 4) & 0xF,
                        dx & 0xF
                    );
                    return;
                } else if family.0 == ProcessorFamily::Intel486Processor {
                    let dx = u16::from_le_bytes(p[0..=1].try_into().expect("u16 is 2 bytes"));

                    // Not all 80486 CPU support the CPUID instruction, we have to find
                    // whether the one we have here does or not. Note that this trick
                    // works only because we know that 80486 must be little-endian.
                    if (dx & 0x0F00) == 0x0400
                        && ((dx & 0x00F0) == 0x0040 || (dx & 0x00F0) >= 0x0070)
                        && ((dx & 0x000F) >= 0x0003)
                    {
                        sig = 1;
                    } else {
                        println!(
                            "\tSignature: Type {}, Family {}, Major Stepping {}, Minor Stepping {}",
                            (dx >> 12) & 0x3,
                            (dx >> 8) & 0xF,
                            (dx >> 4) & 0xF,
                            dx & 0xF
                        );
                        return;
                    }
                }
                // ARM
                else if family.0 == ProcessorFamily::ARMv7
                    || family.0 == ProcessorFamily::ARMv8
                    || (family.1 >= 0x118 && family.1 <= 0x119)
                {
                    let midr = u32::from_le_bytes(p[4..=7].try_into().expect("u32 is 4 bytes"));

                    // The format of this field was not defined for ARM processors
                    // before version 3.1.0 of the SMBIOS specification, so we
                    // silently skip it if it reads all zeroes.
                    if midr == 0 {
                        return;
                    }

                    println!("\tSignature: Implementor {:#04x}, Variant {:#x}, Architecture {}, Part {:#05x}, Revision {}", midr >> 24, (midr >> 20) & 0xF, (midr >> 16) & 0xF, (midr >> 4) & 0xFFF, midr & 0xF);
                    return;
                }
                // Intel
                else if (family.1 >= 0x0B && family.1 <= 0x15)
                    || (family.1 >= 0x28 && family.1 <= 0x2F)
                    || (family.1 >= 0xA1 && family.1 <= 0xB3)
                    || family.0 == ProcessorFamily::IntelXeonProcessorMP
                    || (family.1 >= 0xB9 && family.1 <= 0xC7)
                    || (family.1 >= 0xCD && family.1 <= 0xCF)
                    || (family.1 >= 0xD2 && family.1 <= 0xDB)
                    || (family.1 >= 0xDD && family.1 <= 0xE0)
                {
                    sig = 1;
                }
                // AMD
                else if (family.1 >= 0x18 && family.1 <= 0x1D)
                    || family.0 == ProcessorFamily::K62Plus
                    || (family.1 >= 0x38 && family.1 <= 0x3F)
                    || (family.1 >= 0x46 && family.1 <= 0x4F)
                    || (family.1 >= 0x66 && family.1 <= 0x6B)
                    || (family.1 >= 0x83 && family.1 <= 0x8F)
                    || (family.1 >= 0xB6 && family.1 <= 0xB7)
                    || (family.1 >= 0xE4 && family.1 <= 0xEF)
                {
                    sig = 2;
                }
                // Some X86-class CPU have family "Other" or "Unknown". In this case,
                // we use the version string to determine if they are known to
                // support the CPUID instruction.
                else if family.0 == ProcessorFamily::Other || family.0 == ProcessorFamily::Unknown
                {
                    if let Some(version) = data.processor_version() {
                        match version.as_str() {
                            "Pentium III MMX" => {
                                sig = 1;
                            }
                            "Intel(R) Core(TM)2" => {
                                sig = 1;
                            }
                            "Intel(R) Pentium(R)" => {
                                sig = 1;
                            }
                            "Genuine Intel(R) CPU U1400" => {
                                sig = 1;
                            }
                            "AMD Athlon(TM)" => {
                                sig = 2;
                            }
                            "AMD Opteron(tm)" => {
                                sig = 2;
                            }
                            "Dual-Core AMD Opteron(tm)" => {
                                sig = 2;
                            }
                            _ => return,
                        }
                    }
                } else {
                    // neither X86 nor ARM
                    return;
                }

                // Extra flags are now returned in the ECX register when one calls
                // the CPUID instruction. Their meaning is explained in table 3-5, but
                // DMI doesn't support this yet.
                let eax = u32::from_le_bytes(p[0..=3].try_into().expect("u32 is 4 bytes"));
                let edx = u32::from_le_bytes(p[4..=7].try_into().expect("u32 is 4 bytes"));

                match sig {
                    // Intel
                    1 => {
                        println!(
                            "\tSignature: Type {}, Family {}, Model {}, Stepping {}",
                            (eax >> 12) & 0x3,
                            ((eax >> 20) & 0xFF) + ((eax >> 8) & 0x0F),
                            ((eax >> 12) & 0xF0) + ((eax >> 4) & 0x0F),
                            eax & 0xF
                        );
                    }
                    // AMD
                    2 => {
                        println!(
                            "\tSignature: Family {}, Model {}, Stepping {}",
                            (eax >> 8)
                                & 0xF
                                    + match ((eax >> 8) & 0xF) == 0xF {
                                        true => (eax >> 20) & 0xFF,
                                        false => 0,
                                    },
                            (eax >> 4) & 0xF
                                | match ((eax >> 8) & 0xF) == 0xF {
                                    true => (eax >> 12) & 0xF0,
                                    false => 0,
                                },
                            eax & 0xF
                        );
                    }
                    _ => (),
                }

                // Flags
                match edx & 0xBFEFFBFF == 0 {
                    true => println!("\tFlags: None"),
                    false => {
                        println!("\tFlags:");
                        if (edx & (1 << 0)) != 0 {
                            println!("\t\tFPU (Floating-point unit on-chip)");
                        }
                        if (edx & (1 << 1)) != 0 {
                            println!("\t\tVME (Virtual mode extension)");
                        }
                        if (edx & (1 << 9)) != 0 {
                            println!("\t\tDE (Debugging extension)");
                        }
                        if (edx & (1 << 3)) != 0 {
                            println!("\t\tPSE (Page size extension)");
                        }
                        if (edx & (1 << 4)) != 0 {
                            println!("\t\tTSC (Time stamp counter)");
                        }
                        if (edx & (1 << 5)) != 0 {
                            println!("\t\tMSR (Model specific registers)");
                        }
                        if (edx & (1 << 6)) != 0 {
                            println!("\t\tPAE (Physical address extension)");
                        }
                        if (edx & (1 << 7)) != 0 {
                            println!("\t\tMCE (Machine check exception)");
                        }
                        if (edx & (1 << 8)) != 0 {
                            println!("\t\tCX8 (CMPXCHG8 instruction supported)");
                        }
                        if (edx & (1 << 9)) != 0 {
                            println!("\t\tAPIC (On-chip APIC hardware supported)");
                        }

                        if (edx & (1 << 11)) != 0 {
                            println!("\t\tSEP (Fast system call)");
                        }
                        if (edx & (1 << 12)) != 0 {
                            println!("\t\tMTRR (Memory type range registers)");
                        }
                        if (edx & (1 << 13)) != 0 {
                            println!("\t\tPGE (Page global enable)");
                        }
                        if (edx & (1 << 14)) != 0 {
                            println!("\t\tMCA (Machine check architecture)");
                        }
                        if (edx & (1 << 15)) != 0 {
                            println!("\t\tCMOV (Conditional move instruction supported)");
                        }
                        if (edx & (1 << 16)) != 0 {
                            println!("\t\tPAT (Page attribute table)");
                        }
                        if (edx & (1 << 17)) != 0 {
                            println!("\t\tPSE-36 (36-bit page size extension)");
                        }
                        if (edx & (1 << 18)) != 0 {
                            println!("\t\tPSN (Processor serial number present and enabled)");
                        }
                        if (edx & (1 << 19)) != 0 {
                            println!("\t\tCLFSH (CLFLUSH instruction supported)");
                        }

                        if (edx & (1 << 21)) != 0 {
                            println!("\t\tDS (Debug store)");
                        }
                        if (edx & (1 << 22)) != 0 {
                            println!("\t\tACPI (ACPI supported)");
                        }
                        if (edx & (1 << 23)) != 0 {
                            println!("\t\tMMX (MMX technology supported)");
                        }
                        if (edx & (1 << 24)) != 0 {
                            println!("\t\tFXSR (FXSAVE and FXSTOR instructions supported)");
                        }
                        if (edx & (1 << 25)) != 0 {
                            println!("\t\tSSE (Streaming SIMD extensions)");
                        }
                        if (edx & (1 << 26)) != 0 {
                            println!("\t\tSSE2 (Streaming SIMD extensions 2)");
                        }
                        if (edx & (1 << 27)) != 0 {
                            println!("\t\tSS (Self-snoop)");
                        }
                        if (edx & (1 << 28)) != 0 {
                            println!("\t\tHTT (Multi-threading)");
                        }
                        if (edx & (1 << 29)) != 0 {
                            println!("\t\tTM (Thermal monitor supported)");
                        }
                        if (edx & (1 << 31)) != 0 {
                            println!("\t\tPBE (Pending break enabled)");
                        }
                    }
                }
            }
            None => (),
        }
    }
}
fn dmi_memory_controller_ed_method(error_detecting_method: ErrorDetectingMethodData) -> String {
    let print = match error_detecting_method.value {
        ErrorDetectingMethod::Other => OTHER,
        ErrorDetectingMethod::Unknown => UNKNOWN,
        ErrorDetectingMethod::NoErrorDetection => NONE,
        ErrorDetectingMethod::Parity8Bit => "8-bit Parity",
        ErrorDetectingMethod::Ecc32Bit => "32-bit ECC",
        ErrorDetectingMethod::Ecc64Bit => "64-bit ECC",
        ErrorDetectingMethod::Ecc128Bit => "128-bit ECC",
        ErrorDetectingMethod::Crc => "CRC",
        ErrorDetectingMethod::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, error_detecting_method.raw),
        false => print.to_string(),
    }
}
fn dmi_memory_controller_ec_capabilities(
    attr: &str,
    error_correcting_capabilities: ErrorCorrectingCapabilities,
) {
    if error_correcting_capabilities.raw & 0x3F == 0 {
        println!("\t{}: None", attr);
    } else {
        println!("\t{}:", attr);
        if error_correcting_capabilities.other() {
            println!("\t\tOther");
        }
        if error_correcting_capabilities.unknown() {
            println!("\t\tUnknown");
        }
        if error_correcting_capabilities.no_capabilities() {
            println!("\t\tNone");
        }
        if error_correcting_capabilities.single_bit_error_correcting() {
            println!("\t\tSingle-bit Error Correcting");
        }
        if error_correcting_capabilities.double_bit_error_correcting() {
            println!("\t\tDouble-bit Error Correcting");
        }
        if error_correcting_capabilities.error_scrubbing() {
            println!("\t\tError Scrubbing");
        }
        if error_correcting_capabilities.other() {
            println!("\t\tOther");
        }
    }
}
fn dmi_memory_controller_interleave(interleave: InterleaveSupportData) -> String {
    let print = match interleave.value {
        InterleaveSupport::Other => OTHER,
        InterleaveSupport::Unknown => UNKNOWN,
        InterleaveSupport::OneWay => "One-way Interleave",
        InterleaveSupport::TwoWay => "Two-way Interleave",
        InterleaveSupport::FourWay => "Four-way Interleave",
        InterleaveSupport::EightWay => "Eight-way Interleave",
        InterleaveSupport::SixteenWay => "Sixteen-way Interleave",
        InterleaveSupport::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, interleave.raw),
        false => print.to_string(),
    }
}
fn dmi_memory_controller_speeds(speeds: MemorySpeeds) {
    println!("\tSupported Speeds:");
    if speeds.raw == 0 {
        println!(": None");
    } else {
        println!();
        if speeds.other() {
            println!("\t\tOther");
        }
        if speeds.unknown() {
            println!("\t\tUnknown");
        }
        if speeds.ns70() {
            println!("\t\t70 ns");
        }
        if speeds.ns60() {
            println!("\t\t60 ns");
        }
        if speeds.ns50() {
            println!("\t\t50 ns");
        }
    }
}
fn dmi_memory_module_types(attr: &str, memory_types: MemoryTypes, flat: bool) {
    if memory_types.raw & 0x07FF == 0 {
        println!("\t{}: None", attr);
    } else {
        let mut vec = Vec::new();
        if memory_types.other() {
            vec.push(OTHER)
        }
        if memory_types.unknown() {
            vec.push(UNKNOWN)
        }
        if memory_types.standard() {
            vec.push("Standard")
        }
        if memory_types.fast_page_mode() {
            vec.push("FPM")
        }
        if memory_types.edo() {
            vec.push("EDO")
        }
        if memory_types.parity() {
            vec.push("Parity")
        }
        if memory_types.ecc() {
            vec.push("ECC")
        }
        if memory_types.simm() {
            vec.push("SIMM")
        }
        if memory_types.dimm() {
            vec.push("DIMM")
        }
        if memory_types.burst_edo() {
            vec.push("Burst EDO")
        }
        if memory_types.sdram() {
            vec.push("SDRAM")
        }

        if vec.len() != 0 {
            if flat {
                print!("\t{}: ", attr);
                let mut iter = vec.iter();
                print!("{}", iter.next().unwrap());
                while let Some(memory_type) = iter.next() {
                    // Insert space if not the first value
                    print!(" {}", memory_type);
                }
                println!();
            } else {
                println!("\t{}:", attr);
                for memory_type in vec {
                    println!("\t\t{}", memory_type);
                }
            }
        }
    }
}
fn dmi_memory_controller_slots(associated_slots: ModuleHandleIterator<'_>) {
    let iter: Vec<Handle> = associated_slots.collect();
    println!("\tAssociated Memory Slots: {}", iter.len());
    for handle in iter {
        println!("\t\t{:#06X}", *handle);
    }
}
fn dmi_memory_module_connections(bank_connections: u8) {
    print!("\tBank Connections: ");
    if bank_connections == 0xFF {
        println!("{}", NONE);
    } else if bank_connections & 0xF0 == 0xF0 {
        println!("{}", bank_connections & 0x0F);
    } else if bank_connections & 0x0F == 0x0F {
        println!("{}", bank_connections >> 4);
    } else {
        println!("{} {}", bank_connections >> 4, bank_connections & 0x0F);
    }
}
fn dmi_memory_module_speed(attr: &str, speed: u8) {
    print!("\t{}: ", attr);
    if speed == 0 {
        println!("{}", UNKNOWN);
    } else {
        println!("{} ns", speed);
    }
}
fn dmi_memory_module_size(attr: &str, size: u8) {
    print!("\t{}: ", attr);
    let connection = match size & 0x80 == 0x80 {
        true => "(Double-bank Connection)",
        false => "(Single-bank Connection)",
    };
    match size & 0x7F {
        0x7D => println!("Not Determinable {}", connection),
        0x7E => println!("Disabled {}", connection),
        0x7F => println!("Not Installed"),
        val => match 1u128.checked_shl(val as u32) {
            Some(mb) => println!("{} MB {}", mb, connection),
            None => println!("Overflow MB {}", connection),
        },
    }
}
fn dmi_memory_module_error(error_status: u8) {
    print!("\tError Status: ");
    let print = match error_status {
        0x00 => "OK",
        0x01 => "Uncorrectable Errors",
        0x02 => "Correctable Errors",
        0x03 => "Correctable and Uncorrectable Errors",
        0x04 => "See Event Log",
        _ => "",
    };
    match print == "" {
        true => println!("{} ({})", OUT_OF_SPEC, error_status),
        false => println!("{}", print),
    }
}
fn dmi_cache_size(attr: &str, size1_opt: Option<u16>, size2_opt: Option<u32>) {
    let large_opt = match (size1_opt, size2_opt) {
        (Some(installed_size), None) => {
            // High bit 15 is granularity.  Make it bit 31 to match installed_cache_size_2):
            // 0 == 1K
            // 1 == 16K
            let size_32 = installed_size as u32;
            Some((size_32 & 0x8000u32 << 16) | (size_32 & 0x7FFFu32))
        }
        (Some(_), Some(installed_size_2)) => Some(installed_size_2),
        _ => None,
    };

    if let Some(large) = large_opt {
        // Read bit 31:
        // 0 == 1K
        // 1 == 16K
        // ... then normalize to 1K units.
        let size: u64 = match large & 0x80000000u32 == 0x80000000u32 {
            true => (large as u64 & 0x7FFFFFFFu64) * 16u64,
            false => large as u64,
        };

        dmi_print_memory_size(attr, size, true);
    }
}
fn dmi_print_memory_size(attr: &str, size: u64, shift: bool) {
    // The number 0 has no units, report it as 0 bytes.
    if size == 0 {
        println!("\t{}: 0 bytes", attr);
        return;
    }

    const U_BYTES: &str = "bytes";
    const U_KB: &str = "kB";
    const U_MB: &str = "MB";
    const U_GB: &str = "GB";
    const U_TB: &str = "TB";
    const U_PB: &str = "PB";
    const U_EB: &str = "EB";
    const U_ZB: &str = "ZB";

    // Note: 0n1024 decimal is 0b0100 0000 0000 binary (a 1 with 10 zeros)
    // byte, kb, MB, etc. are n^1024.
    let units = match (63 - size.leading_zeros()) / 10 {
        0 => (
            size,
            match shift {
                true => U_KB,
                false => U_BYTES,
            },
        ),
        1 => (
            size >> 10,
            match shift {
                true => U_MB,
                false => U_KB,
            },
        ),
        2 => (
            size >> 20,
            match shift {
                true => U_GB,
                false => U_MB,
            },
        ),
        3 => (
            size >> 30,
            match shift {
                true => U_TB,
                false => U_GB,
            },
        ),
        4 => (
            size >> 40,
            match shift {
                true => U_PB,
                false => U_TB,
            },
        ),
        5 => (
            size >> 50,
            match shift {
                true => U_EB,
                false => U_PB,
            },
        ),
        _ => (
            size >> 60,
            match shift {
                true => U_ZB,
                false => U_EB,
            },
        ),
    };

    println!("\t{}: {} {}", attr, units.0, units.1);
}
fn dmi_cache_types(attr: &str, types: SramTypes, flat: bool) {
    if types.raw & 0x7F == 0 {
        println!("\t{}: None", attr);
    } else {
        let mut vec = Vec::new();
        if types.other() {
            vec.push(OTHER)
        } else if types.unknown() {
            vec.push(UNKNOWN)
        } else if types.non_burst() {
            vec.push("Non-burst")
        } else if types.pipeline_burst() {
            vec.push("Pipeline Burst")
        } else if types.synchronous() {
            vec.push("Synchronous")
        } else if types.asynchronous() {
            vec.push("Asynchronous")
        }

        if vec.len() != 0 {
            if flat {
                print!("\t{}: ", attr);
                let mut iter = vec.iter();
                print!("{}", iter.next().unwrap());
                while let Some(cache_type) = iter.next() {
                    // Insert space if not the first value
                    print!(" {}", cache_type);
                }
                println!();
            } else {
                println!("\t{}:", attr);
                for cache_type in vec {
                    println!("\t\t{}", cache_type);
                }
            }
        }
    }
}
fn dmi_cache_ec_type(ec_type: ErrorCorrectionTypeData) -> String {
    let print = match ec_type.value {
        ErrorCorrectionType::Other => OTHER,
        ErrorCorrectionType::Unknown => UNKNOWN,
        ErrorCorrectionType::NoCorrection => NONE,
        ErrorCorrectionType::Parity => "Parity",
        ErrorCorrectionType::SingleBitEcc => "Single-bit ECC",
        ErrorCorrectionType::MultiBitEcc => "Multi-bit ECC",
        ErrorCorrectionType::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, ec_type.raw),
        false => print.to_string(),
    }
}
fn dmi_cache_type(cache_type: SystemCacheTypeData) -> String {
    let print = match cache_type.value {
        SystemCacheType::Other => OTHER,
        SystemCacheType::Unknown => UNKNOWN,
        SystemCacheType::Instruction => "Instruction",
        SystemCacheType::Data => "Data",
        SystemCacheType::Unified => "Unified",
        SystemCacheType::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, cache_type.raw),
        false => print.to_string(),
    }
}
fn dmi_cache_associativity(associativity: CacheAssociativityData) -> String {
    let print = match associativity.value {
        CacheAssociativity::Other => OTHER,
        CacheAssociativity::Unknown => UNKNOWN,
        CacheAssociativity::DirectMapped => "Direct Mapped",
        CacheAssociativity::SetAssociative2Way => "2-way Set-associative",
        CacheAssociativity::SetAssociative4Way => "4-way Set-associative",
        CacheAssociativity::FullyAssociative => "Fully Associative",
        CacheAssociativity::SetAssociative8Way => "8-way Set-associative",
        CacheAssociativity::SetAssociative16Way => "16-way Set-associative",
        CacheAssociativity::SetAssociative12Way => "12-way Set-associative",
        CacheAssociativity::SetAssociative24Way => "24-way Set-associative",
        CacheAssociativity::SetAssociative32Way => "32-way Set-associative",
        CacheAssociativity::SetAssociative48Way => "48-way Set-associative",
        CacheAssociativity::SetAssociative64Way => "64-way Set-associative",
        CacheAssociativity::SetAssociative20Way => "20-way Set-associative",
        CacheAssociativity::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, associativity.raw),
        false => print.to_string(),
    }
}
fn dmi_memory_array_error_handle(handle: Handle) {
    print!("\tError Information Handle: ");
    match *handle {
        0xFFFE => println!("Not Provided"),
        0xFFFF => println!("No Error"),
        val => println!("{:#06X}", val),
    }
}
fn dmi_memory_device_width(attr: &str, width: u16) {
    print!("\t{}: ", attr);
    match width == 0xFFFF || width == 0 {
        true => println!("{}", UNKNOWN),
        false => println!("{} bits", width),
    }
}
fn dmi_memory_device_size(size: MemorySize) {
    print!("\tSize: ");
    match size {
        MemorySize::NotInstalled => println!("No Module Installed"),
        MemorySize::Unknown => println!("{}", UNKNOWN),
        MemorySize::SeeExtendedSize => {
            println!("Error, extended Size does not exist.")
        }
        MemorySize::Kilobytes(size_kb) => println!("{} kB", size_kb),
        MemorySize::Megabytes(size_mb) => println!("{} MB", size_mb),
    };
}
fn dmi_memory_device_form_factor(form_factor: MemoryFormFactorData) -> String {
    let print = match form_factor.value {
        MemoryFormFactor::Other => OTHER,
        MemoryFormFactor::Unknown => UNKNOWN,
        MemoryFormFactor::Simm => "SIMM",
        MemoryFormFactor::Sip => "SIP",
        MemoryFormFactor::Chip => "Chip",
        MemoryFormFactor::Dip => "DIP",
        MemoryFormFactor::Zip => "ZIP",
        MemoryFormFactor::ProprietaryCard => "Proprietary Card",
        MemoryFormFactor::Dimm => "DIMM",
        MemoryFormFactor::Tsop => "TSOP",
        MemoryFormFactor::RowOfChips => "Row Of Chips",
        MemoryFormFactor::Rimm => "RIMM",
        MemoryFormFactor::Sodimm => "SODIMM",
        MemoryFormFactor::Srimm => "SRIMM",
        MemoryFormFactor::Fbdimm => "FB-DIMM",
        MemoryFormFactor::Die => "Die",
        MemoryFormFactor::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, form_factor.raw),
        false => print.to_string(),
    }
}
fn dmi_memory_device_set(device_set: u8) {
    print!("\tSet: ");
    match device_set {
        0 => println!("{}", NONE),
        0xFF => println!("{}", UNKNOWN),
        val => println!("{}", val),
    }
}
fn dmi_memory_device_type(memory_type: MemoryDeviceTypeData) -> String {
    let print = match memory_type.value {
        MemoryDeviceType::Other => OTHER,
        MemoryDeviceType::Unknown => UNKNOWN,
        MemoryDeviceType::Dram => "DRAM",
        MemoryDeviceType::Edram => "EDRAM",
        MemoryDeviceType::Vram => "VRAM",
        MemoryDeviceType::Sram => "SRAM",
        MemoryDeviceType::Ram => "RAM",
        MemoryDeviceType::Rom => "ROM",
        MemoryDeviceType::Flash => "Flash",
        MemoryDeviceType::Eeprom => "EEPROM",
        MemoryDeviceType::Feprom => "FEPROM",
        MemoryDeviceType::Eprom => "EPROM",
        MemoryDeviceType::Cdram => "CDRAM",
        MemoryDeviceType::ThreeDram => "3DRAM",
        MemoryDeviceType::Sdram => "SDRAM",
        MemoryDeviceType::Sgram => "SGRAM",
        MemoryDeviceType::Rdram => "RDRAM",
        MemoryDeviceType::Ddr => "DDR",
        MemoryDeviceType::Ddr2 => "DDR2",
        MemoryDeviceType::Ddr2Fbdimm => "DDR2 FB-DIMM",
        MemoryDeviceType::Ddr3 => "DDR3",
        MemoryDeviceType::Fbd2 => "FBD2",
        MemoryDeviceType::Ddr4 => "DDR4",
        MemoryDeviceType::Lpddr => "LPDDR",
        MemoryDeviceType::Lpddr2 => "LPDDR2",
        MemoryDeviceType::Lpddr3 => "LPDDR3",
        MemoryDeviceType::Lpddr4 => "LPDDR4",
        MemoryDeviceType::LogicalNonVolatileDevice => "Logical non-volatile device",
        MemoryDeviceType::Hbm => "HBM",
        MemoryDeviceType::Hbm2 => "HBM2",
        MemoryDeviceType::Ddr5 => "DDR5",
        MemoryDeviceType::Lpddr5 => "LPDDR5",
        MemoryDeviceType::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, memory_type.raw),
        false => print.to_string(),
    }
}
fn dmi_memory_device_type_detail(type_detail: MemoryTypeDetails) {
    print!("\tType Detail: ");
    if type_detail.raw & 0xFFFE == 0 {
        println!("{}", NONE);
    } else {
        let mut vec = Vec::new();
        if type_detail.other() {
            vec.push(OTHER)
        } else if type_detail.unknown() {
            vec.push(UNKNOWN)
        } else if type_detail.fast_paged() {
            vec.push("Fast-paged")
        } else if type_detail.static_column() {
            vec.push("Static Column")
        } else if type_detail.ram_bus() {
            vec.push("RAMBus")
        } else if type_detail.synchronous() {
            vec.push("Synchronous")
        }
        if vec.len() != 0 {
            let mut iter = vec.iter();
            print!("{}", iter.next().unwrap());
            while let Some(detail) = iter.next() {
                // Insert space if not the first value
                print!(" {}", detail);
            }
            println!();
        }
    }
}
fn dmi_memory_device_speed(attr: &str, speed_short: Option<MemorySpeed>, speed_long: Option<u32>) {
    let val_opt = match (speed_short, speed_long) {
        (Some(short), Some(long)) => {
            match short {
                MemorySpeed::Unknown => Some(UNKNOWN.to_string()),
                MemorySpeed::SeeExtendedSpeed => {
                    // Bit 31 is reserved for future use and must be set to 0
                    let mts = long & 0x7FFFFFFFu32;
                    Some(format!("{} MT/s", mts))
                }
                MemorySpeed::MTs(mts) => Some(format!("{} MT/s", mts)),
            }
        }
        (Some(short), None) => match short {
            MemorySpeed::Unknown => Some(UNKNOWN.to_string()),
            MemorySpeed::SeeExtendedSpeed => {
                Some("Error, extended speed required but not present".to_string())
            }
            MemorySpeed::MTs(mts) => Some(format!("{} MT/s", mts)),
        },
        _ => None,
    };
    if let Some(val) = val_opt {
        println!("\t{}: {}", attr, val);
    }
}
fn dmi_memory_voltage_value(attr: &str, millivolts: u16) {
    match millivolts == 0 {
        true => println!("\t{}: Unknown", attr),
        false => {
            let volts = (millivolts as f32) / 1000f32;
            match millivolts % 100 == 0 {
                true => println!("\t{}: {:.1} V", attr, volts),
                false => println!("\t{}: {:e} V", attr, volts),
            }
        }
    }
}
fn dmi_memory_technology(technology: MemoryDeviceTechnologyData) {
    print!("\tMemory Technology: ");
    let print = match technology.value {
        MemoryDeviceTechnology::Other => OTHER,
        MemoryDeviceTechnology::Unknown => UNKNOWN,
        MemoryDeviceTechnology::Dram => "DRAM",
        MemoryDeviceTechnology::NvdimmN => "NVDIMM-N",
        MemoryDeviceTechnology::NvdimmF => "NVDIMM-F",
        MemoryDeviceTechnology::NvdimmP => "NVDIMM-P",
        MemoryDeviceTechnology::IntelOptaneDcPersistentMemory => {
            "Intel Optane DC persistent memory"
        }
        MemoryDeviceTechnology::None => "",
    };
    match print == "" {
        true => println!("{} ({})", OUT_OF_SPEC, technology.raw),
        false => println!("{}", print),
    }
}
fn dmi_memory_operating_mode_capability(mode: MemoryOperatingModeCapabilities) {
    print!("\tMemory Operating Mode Capability: ");
    if mode.raw & 0xFFFE == 0 {
        println!("None");
    } else {
        let mut vec = Vec::new();
        if mode.other() {
            vec.push(OTHER)
        } else if mode.unknown() {
            vec.push(UNKNOWN)
        } else if mode.volatile_memory() {
            vec.push("Volatile memory")
        } else if mode.byte_accessible_persistent_memory() {
            vec.push("Byte-accessible persistent memory")
        } else if mode.block_accessible_persistent_memory() {
            vec.push("Block-accessible persistent memory")
        }

        if vec.len() != 0 {
            let mut iter = vec.iter();
            print!("{}", iter.next().unwrap());
            while let Some(mode) = iter.next() {
                // Insert space if not the first value
                print!(" {}", mode);
            }
            println!();
        }
    }
}
fn dmi_memory_manufacturer_id(attr: &str, id: u16) {
    print!("\t{}: ", attr);
    match id == 0 {
        true => println!("{}", UNKNOWN),
        false => println!("Bank {}, Hex {:#04X}", (id & 0x7F) + 1, id >> 8),
    }
}
fn dmi_memory_product_id(attr: &str, id: u16) {
    print!("\t{}: ", attr);
    match id == 0 {
        true => println!("{}", UNKNOWN),
        false => println!("{:#06X}", id),
    }
}
fn dmi_memory_size(attr: &str, size: MemoryIndicatedSize) {
    match size {
        MemoryIndicatedSize::Unknown => {
            println!("\t{}: Unknown", attr);
        }
        MemoryIndicatedSize::Bytes(bytes) => match bytes {
            0u64 => {
                println!("\t{}: None", attr);
            }
            _ => {
                dmi_print_memory_size(attr, bytes, false);
            }
        },
    }
}
fn dmi_memory_error_type(error_type: MemoryErrorTypeData) -> String {
    let print = match error_type.value {
        MemoryErrorType::Other => OTHER,
        MemoryErrorType::Unknown => UNKNOWN,
        MemoryErrorType::OK => "OK",
        MemoryErrorType::BadRead => "Bad Read",
        MemoryErrorType::ParityError => "Parity Error",
        MemoryErrorType::SingleBitError => "Single-bit Error",
        MemoryErrorType::DoubleBitError => "Double-bit Error",
        MemoryErrorType::MultiBitError => "Multi-bit Error",
        MemoryErrorType::NibbleError => "Nibble Error",
        MemoryErrorType::ChecksumError => "Checksum Error",
        MemoryErrorType::CrcError => "CRC Error",
        MemoryErrorType::CorrectedSingleBitError => "Corrected Single-bit Error",
        MemoryErrorType::CorrectedError => "Corrected Error",
        MemoryErrorType::UncorrectableError => "Uncorrectable Error",
        MemoryErrorType::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, error_type.raw),
        false => print.to_string(),
    }
}
fn dmi_memory_error_granularity(granularity: MemoryErrorGranularityData) -> String {
    let print = match granularity.value {
        MemoryErrorGranularity::Other => OTHER,
        MemoryErrorGranularity::Unknown => UNKNOWN,
        MemoryErrorGranularity::DeviceLevel => "Device Level",
        MemoryErrorGranularity::MemoryPartitionLevel => "Memory Partition Level",
        MemoryErrorGranularity::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, granularity.raw),
        false => print.to_string(),
    }
}
fn dmi_memory_error_operation(operation: MemoryErrorOperationData) -> String {
    let print = match operation.value {
        MemoryErrorOperation::Other => OTHER,
        MemoryErrorOperation::Unknown => UNKNOWN,
        MemoryErrorOperation::Read => "Read",
        MemoryErrorOperation::Write => "Write",
        MemoryErrorOperation::PartialWrite => "Partial Write",
        MemoryErrorOperation::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, operation.raw),
        false => print.to_string(),
    }
}
fn dmi_memory_error_syndrome(syndrome: u32) {
    print!("\tVendor Syndrome: ");
    match syndrome == 0 {
        true => println!("{}", UNKNOWN),
        false => println!("{:#10X}", syndrome),
    }
}
fn dmi_32bit_memory_error_address(attr: &str, address: u32) {
    print!("\t{}: ", attr);
    match address == 0x80000000u32 {
        true => println!("{}", UNKNOWN),
        false => println!("{:#10X}", address),
    }
}
fn dmi_mapped_address_extended_size(start: u64, end: u64) {
    const ATTR: &str = "Range Size";
    match start >= end {
        true => println!("\t{}: Invalid", ATTR),
        false => dmi_print_memory_size(ATTR, end - start + 1, false),
    }
}
fn dmi_memory_array_location(location: MemoryArrayLocationData) -> String {
    let print = match location.value {
        MemoryArrayLocation::Other => OTHER,
        MemoryArrayLocation::Unknown => UNKNOWN,
        MemoryArrayLocation::SystemBoardOrMotherboard => "System Board Or Motherboard",
        MemoryArrayLocation::IsaAddOnCard => "ISA Add-on Card",
        MemoryArrayLocation::EisaAddOnCard => "EISA Add-on Card",
        MemoryArrayLocation::PciAddOnCard => "PCI Add-on Card",
        MemoryArrayLocation::McaAddOnCard => "MCA Add-on Card",
        MemoryArrayLocation::PcmciaAddOnCard => "PCMCIA Add-on Card",
        MemoryArrayLocation::ProprietaryAddOnCard => "Proprietary Add-on Card",
        MemoryArrayLocation::NuBus => "NuBus",
        MemoryArrayLocation::PC98C20AddOnCard => "PC-98/C20 Add-on Card",
        MemoryArrayLocation::PC98C24AddOnCard => "PC-98/C24 Add-on Card",
        MemoryArrayLocation::PC98EAddOnCard => "PC-98/E Add-on Card",
        MemoryArrayLocation::PC98LocalBusAddOnCard => "PC-98/Local Bus Add-on Card",
        MemoryArrayLocation::CxlFlexbus10AddOnCard => "CXL Flexbus 1.0",
        MemoryArrayLocation::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, location.raw),
        false => print.to_string(),
    }
}
fn dmi_memory_array_use(usage: MemoryArrayUseData) -> String {
    let print = match usage.value {
        MemoryArrayUse::Other => OTHER,
        MemoryArrayUse::Unknown => UNKNOWN,
        MemoryArrayUse::SystemMemory => "System Memory",
        MemoryArrayUse::VideoMemory => "Video Memory",
        MemoryArrayUse::FlashMemory => "Flash Memory",
        MemoryArrayUse::NonVolatileRam => "Non-volatile RAM",
        MemoryArrayUse::CacheMemory => "Cache Memory",
        MemoryArrayUse::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, usage.raw),
        false => print.to_string(),
    }
}
fn dmi_memory_array_ec_type(memory_error_correction: MemoryArrayErrorCorrectionData) -> String {
    let print = match memory_error_correction.value {
        MemoryArrayErrorCorrection::Other => OTHER,
        MemoryArrayErrorCorrection::Unknown => UNKNOWN,
        MemoryArrayErrorCorrection::NoCorrection => NONE,
        MemoryArrayErrorCorrection::Parity => "Parity",
        MemoryArrayErrorCorrection::SingleBitEcc => "Single-bit ECC",
        MemoryArrayErrorCorrection::MultiBitEcc => "Multi-bit ECC",
        MemoryArrayErrorCorrection::Crc => "CRC",
        MemoryArrayErrorCorrection::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, memory_error_correction.raw),
        false => print.to_string(),
    }
}
fn dmi_starting_ending_addresses(
    starting: Option<u32>,
    extended_starting: Option<u64>,
    ending: Option<u32>,
    extended_ending: Option<u64>,
) {
    // Convert a 32-bit address in kB to a 64-bit address in bytes
    // Shift left 10 multiplies by 1024 (kB to bytes)
    // The 10 zeros this produces are replaced with 1's (0x3FF) if the original
    // value ended in a 1 (a way of binary rounding).
    let address_32_kb_to_64_bytes = |address: u32| -> u64 {
        let address_64 = (address as u64) << 10;
        match address | 1 == address {
            true => address_64 + 0x3FFu64,
            false => address_64,
        }
    };

    let starting_address = match (starting, extended_starting) {
        (Some(address), Some(extended_address)) => match address == 0xFFFFFFFF {
            true => Some(extended_address),
            false => Some(address_32_kb_to_64_bytes(address)),
        },
        (Some(address), None) => Some(address_32_kb_to_64_bytes(address)),
        _ => None,
    };

    let ending_address = match (ending, extended_ending) {
        (Some(address), Some(extended_address)) => match address == 0xFFFFFFFF {
            true => Some(extended_address),
            false => Some(address_32_kb_to_64_bytes(address)),
        },
        (Some(address), None) => Some(address_32_kb_to_64_bytes(address)),
        _ => None,
    };

    match (starting_address, ending_address) {
        (Some(start), Some(end)) => {
            println!("\tStarting Address: {:#018X}", start);
            println!("\tEnding Address: {:#018X}", end);
            dmi_mapped_address_extended_size(start, end);
        }
        _ => (),
    }
}
fn dmi_mapped_address_row_position(position: u8) {
    print!("\tPartition Row Position: ");
    match position {
        0 => println!("{}", OUT_OF_SPEC),
        0xFF => println!("{}", UNKNOWN),
        _ => println!("{}", position),
    }
}
fn dmi_mapped_address_interleave_position(position: u8) {
    if position != 0 {
        print!("\tInterleave Position: ");
        match position {
            0xFF => println!("{}", UNKNOWN),
            _ => println!("{}", position),
        }
    }
}
fn dmi_mapped_address_interleaved_data_depth(position: u8) {
    if position != 0 {
        print!("\tInterleaved Data Depth: ");
        match position {
            0xFF => println!("{}", UNKNOWN),
            _ => println!("{}", position),
        }
    }
}
fn dmi_hardware_security_status(status: HardwareSecurityStatus) -> String {
    match status {
        HardwareSecurityStatus::Disabled => "Disabled",
        HardwareSecurityStatus::Enabled => "Enabled",
        HardwareSecurityStatus::NotImplemented => "Not Implemented",
        HardwareSecurityStatus::Unknown => UNKNOWN,
    }
    .to_string()
}
fn dmi_bcd_range(value: u8, low: u8, high: u8) -> bool {
    if value > 0x99 || (value & 0x0F) > 0x09 {
        false
    } else if value < low || value > high {
        false
    } else {
        true
    }
}
