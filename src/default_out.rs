use crate::dmifn::*;
use smbioslib::*;

pub const OUT_OF_SPEC: &str = "<OUT OF SPEC>";
const BYTES: &str = "bytes";
const KB: &str = "kB";
const MB: &str = "MB";
const GB: &str = "GB";
pub const OTHER: &str = "Other";
pub const UNKNOWN: &str = "Unknown";
pub const NONE: &str = "None";

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

        dump_undefined_struct(&undefined_struct, smbios_data.version, quiet);
    }
}

pub fn dump_undefined_struct(
    undefined_struct: &UndefinedStruct,
    bios_version: Option<SMBiosVersion>,
    quiet: bool,
) {
    println!(
        "Handle {:#06X}, DMI type {}, {} bytes",
        *undefined_struct.header.handle(),
        undefined_struct.header.struct_type(),
        undefined_struct.fields.len()
    );
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
            if let Some(internal_reference_designator) = data.internal_reference_designator() {
                println!(
                    "\tInternal Reference Designator: {}",
                    internal_reference_designator
                );
            }
            if let Some(internal_connector_type) = data.internal_connector_type() {
                println!(
                    "\tInternal Connector Type: {}",
                    dmi_port_connector_type(&internal_connector_type)
                );
            }
            if let Some(external_reference_designator) = data.external_reference_designator() {
                println!(
                    "\tExternal Reference Designator: {}",
                    external_reference_designator
                );
            }
            if let Some(external_connector_type) = data.external_connector_type() {
                println!(
                    "\tExternal Connector Type: {}",
                    dmi_port_connector_type(&external_connector_type)
                );
            }
            if let Some(port_type) = data.port_type() {
                println!("\tPort Type: {}", dmi_port_type(&port_type));
            }
        }
        DefinedStruct::SystemSlot(data) => {
            println!("System Slot Information");
            if let Some(slot_designation) = data.slot_designation() {
                println!("\tDesignation: {}", slot_designation);
            }
            match (data.slot_data_bus_width(), data.system_slot_type()) {
                (Some(slot_data_bus_width), Some(system_slot_type)) => {
                    println!(
                        "\tType: {} {}",
                        dmi_slot_bus_width(&slot_data_bus_width),
                        dmi_slot_type(&system_slot_type)
                    );
                }
                _ => (),
            }
            if let Some(current_usage) = data.current_usage() {
                println!(
                    "\tCurrent Usage: {}",
                    dmi_slot_current_usage(&current_usage)
                );
            }
            if let Some(slot_length) = data.slot_length() {
                println!("\tSlot Length: {}", dmi_slot_length(&slot_length));
            }
            match (data.slot_id(), data.system_slot_type()) {
                (Some(slot_id), Some(slot_type)) => match slot_type.value {
                    SystemSlotType::Mca => println!("\tID: {}", slot_id.byte_0()),
                    SystemSlotType::Isa => println!("\tID: {}", slot_id.byte_0()),
                    SystemSlotType::Pci => println!("\tID: {}", slot_id.byte_0()),
                    SystemSlotType::Agp(_) => println!("\tID: {}", slot_id.byte_0()),
                    SystemSlotType::PciX => println!("\tID: {}", slot_id.byte_0()),
                    SystemSlotType::PciExpress(_, _) => println!("\tID: {}", slot_id.byte_0()),
                    SystemSlotType::Pcmcia => println!(
                        "\tID: Adapter {}, Socket {}",
                        slot_id.byte_0(),
                        slot_id.byte_1()
                    ),
                    _ => (),
                },
                _ => (),
            }
            dmi_slot_characteristics(
                "Characteristics",
                &data.slot_characteristics_1(),
                &data.slot_characteristics_2(),
            );
            match (
                data.segment_group_number(),
                data.bus_number(),
                data.device_function_number(),
            ) {
                (Some(segment_group_number), Some(bus_number), Some(device_function_number)) => {
                    dmi_slot_segment_bus_func(
                        segment_group_number,
                        bus_number,
                        device_function_number,
                    );
                }
                _ => (),
            }
            if let Some(data_bus_width) = data.data_bus_width() {
                println!("\tData Bus Width: {}", data_bus_width);
            }
            if let Some(peer_group_count) = data.peer_group_count() {
                println!("\tPeer Devices: {}", peer_group_count);
            }
            for slot_peer_group in data.peer_group_iterator().enumerate() {
                let device_function_number = slot_peer_group
                    .1
                    .device_function_number()
                    .unwrap_or_default();
                println!(
                    "\tPeer Device {}: {:04x}:{:02x}:{:02x}.{:x} (Width {})",
                    slot_peer_group.0 + 1,
                    slot_peer_group.1.segment_group_number().unwrap_or_default(),
                    slot_peer_group.1.bus_number().unwrap_or_default(),
                    device_function_number >> 3,
                    device_function_number & 0x07,
                    slot_peer_group.1.data_bus_width().unwrap_or_default()
                );
            }
        }
        DefinedStruct::OnBoardDeviceInformation(data) => {
            let count = data.number_of_devices();
            for onboard_device in data.onboard_device_iterator().enumerate() {
                match count == 1 {
                    true => println!("On Board Device Information"),
                    false => println!("On Board Device {} Information", onboard_device.0 + 1),
                }
                if let Some(device_type) = onboard_device.1.device_type() {
                    println!("\tType: {}", dmi_on_board_devices_type(&device_type));
                    println!(
                        "\tStatus: {}",
                        match device_type.status() {
                            DeviceStatus::Enabled => "Enabled",
                            DeviceStatus::Disabled => "Disabled",
                        }
                    );
                }
                if let Some(description) = onboard_device.1.description() {
                    println!("\tDescription: {}", description);
                }
            }
        }
        DefinedStruct::OemStrings(data) => {
            println!("OEM Strings");
            for oem_string in data.oem_strings().into_iter().enumerate() {
                println!("\tString {}: {}", oem_string.0 + 1, oem_string.1);
            }
        }
        DefinedStruct::SystemConfigurationOptions(data) => {
            println!("System Configuration Options");
            for configuration_option in data.configuration_strings().into_iter().enumerate() {
                println!(
                    "\tOption {}: {}",
                    configuration_option.0 + 1,
                    configuration_option.1
                );
            }
        }
        DefinedStruct::LanguageInformation(data) => {
            println!("BIOS Language Information");
            let two_one_version = SMBiosVersion {
                major: 2,
                minor: 1,
                revision: 0,
            };
            if let Some(version) = bios_version {
                if version >= two_one_version {
                    if let Some(flags) = data.flags() {
                        println!(
                            "\tLanguage Description Format: {}",
                            match flags.language_format() {
                                LanguageFormat::Abbreviated => "Abbreviated",
                                LanguageFormat::Long => "Long",
                            }
                        );
                    }
                }
            }
            if let Some(number_of_installable_languages) = data.number_of_installable_languages() {
                println!(
                    "\tIntallable Languages: {}",
                    number_of_installable_languages
                );
            }
            for installable_language in data.installable_langauges() {
                println!("\t\t{}", installable_language);
            }
            if let Some(current_language) = data.current_language() {
                println!("\tCurrently Installed Language: {}", current_language);
            }
        }
        DefinedStruct::GroupAssociations(data) => {
            println!("Group Associations");
            if let Some(group_name) = data.group_name() {
                println!("\tName: {}", group_name);
            }
            if let Some(number_of_items) = data.number_of_items() {
                println!("\tItems: {}", number_of_items);
            }
            for item in data.item_iterator() {
                match (item.item_handle(), item.struct_type()) {
                    (Some(handle), Some(struct_type)) => {
                        println!(
                            "\t\t{:#06X} {}",
                            *handle,
                            dmi_smbios_structure_type(struct_type)
                        );
                    }
                    _ => (),
                }
            }
        }
        DefinedStruct::EventLog(data) => {
            println!("System Event Log");
            if let Some(log_area_length) = data.log_area_length() {
                println!("\tArea Length: {}", log_area_length);
            }
            match (data.log_header_start_offset(), data.log_data_start_offset()) {
                (Some(log_header_start_offset), Some(log_data_start_offset)) => {
                    println!("\tHeader Start Offset: {:#06X}", log_header_start_offset);
                    let length = log_data_start_offset - log_header_start_offset;
                    if length > 0 {
                        println!(
                            "\tHeader Length: {} {}",
                            length,
                            match length == 1 {
                                true => "byte",
                                false => "bytes",
                            }
                        );
                    }
                    println!("\tData Start Offset: {:#06X}", log_data_start_offset);
                }
                _ => (),
            }
            if let Some(access_method) = data.access_method() {
                println!("\tAccess Method: {}", dmi_event_log_method(&access_method));
            }
            match (data.access_method(), data.access_method_address()) {
                (Some(access_method), Some(access_method_address)) => {
                    dmi_event_log_address(&access_method, access_method_address)
                }
                _ => (),
            }
            if let Some(log_status) = data.log_status() {
                println!(
                    "\tStatus: {}, {}",
                    match log_status.log_area_valid() {
                        true => "Valid",
                        false => "Invalid",
                    },
                    match log_status.log_area_full() {
                        true => "Full",
                        false => "Not Full",
                    }
                );
            }
            if let Some(log_change_token) = data.log_change_token() {
                println!("\tChange Token: {:#10X}", log_change_token);
            }
            if let Some(log_header_format) = data.log_header_format() {
                println!(
                    "\tHeader Format: {}",
                    dmi_event_log_header_type(&log_header_format)
                );
            }
            if let Some(number_of_supported_log_type_descriptors) =
                data.number_of_supported_log_type_descriptors()
            {
                println!(
                    "\tSupported Log Type Descriptors: {}",
                    number_of_supported_log_type_descriptors
                );
            }
            if let Some(type_descriptors) = data.type_descriptors() {
                for type_descriptor in type_descriptors.into_iter().enumerate() {
                    println!(
                        "\tDescriptor {}: {}",
                        type_descriptor.0,
                        dmi_event_log_descriptor_type(&type_descriptor.1.log_type())
                    );
                    println!(
                        "\tData Format {}: {}",
                        type_descriptor.0,
                        dmi_event_log_descriptor_format(
                            &type_descriptor.1.variable_data_format_type()
                        )
                    );
                }
            }
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
            if let Some(capabilities) = data.capabilities() {
                println!(
                    "\tStatus: {}",
                    match capabilities.reset_enabled() {
                        true => "Enabled",
                        false => "Disabled",
                    }
                );

                let has_watchdog_timer = capabilities.has_watchdog_timer();
                println!(
                    "\tWatchdog Timer: {}",
                    match has_watchdog_timer {
                        true => "Present",
                        false => "Not Present",
                    }
                );

                if has_watchdog_timer {
                    println!(
                        "\tBoot Option: {}",
                        match capabilities.boot_option() {
                            BootOption::Reserved => OUT_OF_SPEC,
                            BootOption::OperatingSystem => "Operating System",
                            BootOption::SystemUtilities => "System Utilities",
                            BootOption::DoNotReboot => "Do Not Reboot",
                        }
                    );

                    println!(
                        "\tBoot Option On Limit: {}",
                        match capabilities.boot_option_on_limit() {
                            BootOptionOnLimit::Reserved => OUT_OF_SPEC,
                            BootOptionOnLimit::OperatingSystem => "Operating System",
                            BootOptionOnLimit::SystemUtilities => "System Utilities",
                            BootOptionOnLimit::DoNotReboot => "Do Not Reboot",
                        }
                    );

                    if let Some(reset_count) = data.reset_count() {
                        print!("\tReset Count:");
                        match reset_count {
                            ResetCount::Count(count) => println!("{}", count),
                            ResetCount::Unknown => println!("{}", UNKNOWN),
                        }
                    }

                    if let Some(reset_limit) = data.reset_limit() {
                        print!("\tReset Limit:");
                        match reset_limit {
                            ResetLimit::Count(count) => println!("{}", count),
                            ResetLimit::Unknown => println!("{}", UNKNOWN),
                        }
                    }

                    if let Some(timer_interval) = data.timer_interval() {
                        print!("\tTimer Interval:");
                        match timer_interval {
                            TimerInterval::Minutes(minutes) => println!("{} min", minutes),
                            TimerInterval::Unknown => println!("{}", UNKNOWN),
                        }
                    }

                    if let Some(timeout) = data.timeout() {
                        print!("\tTimeout:");
                        match timeout {
                            Timeout::Minutes(minutes) => println!("{} min", minutes),
                            Timeout::Unknown => println!("{}", UNKNOWN),
                        }
                    }
                }
            }
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
            if let Some(boot_status_data) = data.boot_status_data() {
                println!("\tStatus: {}", dmi_system_boot_status(&boot_status_data));
            }
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
