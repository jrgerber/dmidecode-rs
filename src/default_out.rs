use std::str::FromStr;

use smbioslib::*;

const OUT_OF_SPEC: &str = "<OUT OF SPEC>";
const BYTES: &str = "bytes";
const KB: &str = "kB";
const MB: &str = "MB";
const GB: &str = "GB";

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
                                    println!("{} {}", size, OUT_OF_SPEC);
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
                // pr_list_start("Characteristics", NULL);
                // dmi_bios_characteristics(QWORD(data + 0x0A));
                // pr_list_end();
                // if (h->length < 0x13) break;
                // dmi_bios_characteristics_x1(data[0x12]);
                // if (h->length < 0x14) break;
                // dmi_bios_characteristics_x2(data[0x13]);
                // if (h->length < 0x18) break;
                // if (data[0x14] != 0xFF && data[0x15] != 0xFF)
                // 	pr_attr("BIOS Revision", "%u.%u",
                // 		data[0x14], data[0x15]);
                // if (data[0x16] != 0xFF && data[0x17] != 0xFF)
                // 	pr_attr("Firmware Revision", "%u.%u",
                // 		data[0x16], data[0x17]);
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
    }
}
