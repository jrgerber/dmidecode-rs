use crate::default_out::{NONE, OTHER, OUT_OF_SPEC, UNKNOWN};
use smbioslib::*;
use std::convert::TryInto;

pub fn dmi_smbios_structure_type(code: u8) -> String {
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
pub fn dmi_chassis_state(state: ChassisStateData) -> String {
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

pub fn dmi_processor_type(processor_type: ProcessorTypeData) -> String {
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
pub fn dmi_processor_family(processor_family: ProcessorFamily, raw: u16) -> String {
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

pub fn dmi_processor_upgrade(processor_upgrade: ProcessorUpgradeData) -> String {
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
pub fn dmi_processor_cache(
    label: &str,
    handle: Handle,
    level: &str,
    version: Option<SMBiosVersion>,
) {
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
pub fn dmi_processor_characteristics(characteristics: ProcessorCharacteristics) {
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
pub fn dmi_processor_id(data: &SMBiosProcessorInformation<'_>) {
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
pub fn dmi_memory_controller_ed_method(error_detecting_method: ErrorDetectingMethodData) -> String {
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
pub fn dmi_memory_controller_ec_capabilities(
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
pub fn dmi_memory_controller_interleave(interleave: InterleaveSupportData) -> String {
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
pub fn dmi_memory_controller_speeds(speeds: MemorySpeeds) {
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
pub fn dmi_memory_module_types(attr: &str, memory_types: MemoryTypes, flat: bool) {
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
pub fn dmi_memory_controller_slots(associated_slots: ModuleHandleIterator<'_>) {
    let iter: Vec<Handle> = associated_slots.collect();
    println!("\tAssociated Memory Slots: {}", iter.len());
    for handle in iter {
        println!("\t\t{:#06X}", *handle);
    }
}
pub fn dmi_memory_module_connections(bank_connections: u8) {
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
pub fn dmi_memory_module_speed(attr: &str, speed: u8) {
    print!("\t{}: ", attr);
    if speed == 0 {
        println!("{}", UNKNOWN);
    } else {
        println!("{} ns", speed);
    }
}
pub fn dmi_memory_module_size(attr: &str, size: u8) {
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
pub fn dmi_memory_module_error(error_status: u8) {
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
pub fn dmi_cache_size(attr: &str, size1_opt: Option<u16>, size2_opt: Option<u32>) {
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
pub fn dmi_print_memory_size(attr: &str, size: u64, shift: bool) {
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
pub fn dmi_cache_types(attr: &str, types: SramTypes, flat: bool) {
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
pub fn dmi_cache_ec_type(ec_type: ErrorCorrectionTypeData) -> String {
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
pub fn dmi_cache_type(cache_type: SystemCacheTypeData) -> String {
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
pub fn dmi_cache_associativity(associativity: CacheAssociativityData) -> String {
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
pub fn dmi_memory_array_error_handle(handle: Handle) {
    print!("\tError Information Handle: ");
    match *handle {
        0xFFFE => println!("Not Provided"),
        0xFFFF => println!("No Error"),
        val => println!("{:#06X}", val),
    }
}
pub fn dmi_memory_device_width(attr: &str, width: u16) {
    print!("\t{}: ", attr);
    match width == 0xFFFF || width == 0 {
        true => println!("{}", UNKNOWN),
        false => println!("{} bits", width),
    }
}
pub fn dmi_memory_device_size(size: MemorySize) {
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
pub fn dmi_memory_device_form_factor(form_factor: MemoryFormFactorData) -> String {
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
pub fn dmi_memory_device_set(device_set: u8) {
    print!("\tSet: ");
    match device_set {
        0 => println!("{}", NONE),
        0xFF => println!("{}", UNKNOWN),
        val => println!("{}", val),
    }
}
pub fn dmi_memory_device_type(memory_type: MemoryDeviceTypeData) -> String {
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
pub fn dmi_memory_device_type_detail(type_detail: MemoryTypeDetails) {
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
pub fn dmi_memory_device_speed(
    attr: &str,
    speed_short: Option<MemorySpeed>,
    speed_long: Option<u32>,
) {
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
pub fn dmi_memory_voltage_value(attr: &str, millivolts: u16) {
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
pub fn dmi_memory_technology(technology: MemoryDeviceTechnologyData) {
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
pub fn dmi_memory_operating_mode_capability(mode: MemoryOperatingModeCapabilities) {
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
pub fn dmi_memory_manufacturer_id(attr: &str, id: u16) {
    print!("\t{}: ", attr);
    match id == 0 {
        true => println!("{}", UNKNOWN),
        false => println!("Bank {}, Hex {:#04X}", (id & 0x7F) + 1, id >> 8),
    }
}
pub fn dmi_memory_product_id(attr: &str, id: u16) {
    print!("\t{}: ", attr);
    match id == 0 {
        true => println!("{}", UNKNOWN),
        false => println!("{:#06X}", id),
    }
}
pub fn dmi_memory_size(attr: &str, size: MemoryIndicatedSize) {
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
pub fn dmi_memory_error_type(error_type: MemoryErrorTypeData) -> String {
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
pub fn dmi_memory_error_granularity(granularity: MemoryErrorGranularityData) -> String {
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
pub fn dmi_memory_error_operation(operation: MemoryErrorOperationData) -> String {
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
pub fn dmi_memory_error_syndrome(syndrome: u32) {
    print!("\tVendor Syndrome: ");
    match syndrome == 0 {
        true => println!("{}", UNKNOWN),
        false => println!("{:#10X}", syndrome),
    }
}
pub fn dmi_32bit_memory_error_address(attr: &str, address: u32) {
    print!("\t{}: ", attr);
    match address == 0x80000000u32 {
        true => println!("{}", UNKNOWN),
        false => println!("{:#10X}", address),
    }
}
pub fn dmi_mapped_address_extended_size(start: u64, end: u64) {
    const ATTR: &str = "Range Size";
    match start >= end {
        true => println!("\t{}: Invalid", ATTR),
        false => dmi_print_memory_size(ATTR, end - start + 1, false),
    }
}
pub fn dmi_memory_array_location(location: MemoryArrayLocationData) -> String {
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
pub fn dmi_memory_array_use(usage: MemoryArrayUseData) -> String {
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
pub fn dmi_memory_array_ec_type(memory_error_correction: MemoryArrayErrorCorrectionData) -> String {
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
pub fn dmi_starting_ending_addresses(
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
pub fn dmi_mapped_address_row_position(position: u8) {
    print!("\tPartition Row Position: ");
    match position {
        0 => println!("{}", OUT_OF_SPEC),
        0xFF => println!("{}", UNKNOWN),
        _ => println!("{}", position),
    }
}
pub fn dmi_mapped_address_interleave_position(position: u8) {
    if position != 0 {
        print!("\tInterleave Position: ");
        match position {
            0xFF => println!("{}", UNKNOWN),
            _ => println!("{}", position),
        }
    }
}
pub fn dmi_mapped_address_interleaved_data_depth(position: u8) {
    if position != 0 {
        print!("\tInterleaved Data Depth: ");
        match position {
            0xFF => println!("{}", UNKNOWN),
            _ => println!("{}", position),
        }
    }
}
pub fn dmi_hardware_security_status(status: HardwareSecurityStatus) -> String {
    match status {
        HardwareSecurityStatus::Disabled => "Disabled",
        HardwareSecurityStatus::Enabled => "Enabled",
        HardwareSecurityStatus::NotImplemented => "Not Implemented",
        HardwareSecurityStatus::Unknown => UNKNOWN,
    }
    .to_string()
}
pub fn dmi_bcd_range(value: u8, low: u8, high: u8) -> bool {
    if value > 0x99 || (value & 0x0F) > 0x09 {
        false
    } else if value < low || value > high {
        false
    } else {
        true
    }
}
pub fn dmi_system_boot_status(boot_status_data: &SystemBootStatusData<'_>) -> String {
    let print = match boot_status_data.system_boot_status() {
        SystemBootStatus::NoErrors => "No errors detected",
        SystemBootStatus::NoBootableMedia => "No bootable media",
        SystemBootStatus::NormalOSFailedToLoad => "Operating system failed to load",
        SystemBootStatus::FirmwareDetectedFailure => "Firmware-detected hardware failure",
        SystemBootStatus::OSDetectedFailure => "Operating system-detected hardware failure",
        SystemBootStatus::UserRequestedBoot => "User-requested boot",
        SystemBootStatus::SystemSecurityViolation => "System security violation",
        SystemBootStatus::PreviouslyRequestedImage => "Previously-requested image",
        SystemBootStatus::SystemWatchdogTimerExpired => "System watchdog timer expired",
        SystemBootStatus::None => "",
    };

    match print == "" {
        true => match boot_status_data.raw.len() == 0 {
            true => OUT_OF_SPEC.to_string(),
            false => {
                let byte = boot_status_data.raw[0];
                if byte >= 128u8 && byte <= 191u8 {
                    "OEM-specific".to_string()
                } else if byte >= 192u8 {
                    "Product-specific".to_string()
                } else {
                    OUT_OF_SPEC.to_string()
                }
            }
        },
        false => format!("{}", print),
    }
}
pub fn dmi_port_connector_type(port_connector_type: &PortInformationConnectorTypeData) -> String {
    let print = match port_connector_type.value {
        PortInformationConnectorType::NoConnector => NONE,
        PortInformationConnectorType::Centronics => "Centronics",
        PortInformationConnectorType::MiniCentronics => "Mini Centronics",
        PortInformationConnectorType::Proprietary => "Proprietary",
        PortInformationConnectorType::DB25PinMale => "DB-25 male",
        PortInformationConnectorType::DB25PinFemale => "DB-25 female",
        PortInformationConnectorType::DB15PinMale => "DB-15 male",
        PortInformationConnectorType::DB15PinFemale => "DB-15 female",
        PortInformationConnectorType::DB9PinMale => "DB-9 male",
        PortInformationConnectorType::DB8PinFemale => "DB-9 female",
        PortInformationConnectorType::RJ11 => "RJ-11",
        PortInformationConnectorType::RJ45 => "RJ-45",
        PortInformationConnectorType::MiniScsi50Pin => "50 Pin MiniSCSI",
        PortInformationConnectorType::MiniDin => "Mini DIN",
        PortInformationConnectorType::MicroDin => "Micro DIN",
        PortInformationConnectorType::Ps2 => "PS/2",
        PortInformationConnectorType::Infrared => "Infrared",
        PortInformationConnectorType::HpHil => "HP-HIL",
        PortInformationConnectorType::AccessBusUsb => "Access Bus (USB)",
        PortInformationConnectorType::SsaScsi => "SSA SCSI",
        PortInformationConnectorType::CircularDin8Male => "Circular DIN-8 male",
        PortInformationConnectorType::CircularDin8Female => "Circular DIN-8 female",
        PortInformationConnectorType::OnBoardIde => "On Board IDE",
        PortInformationConnectorType::OnBoardFloppy => "On Board Floppy",
        PortInformationConnectorType::DualInline9Pin => "9 Pin Dual Inline (pin 10 cut)",
        PortInformationConnectorType::DualInline25Pin => "25 Pin Dual Inline (pin 26 cut)",
        PortInformationConnectorType::DualInline50Pin => "50 Pin Dual Inline",
        PortInformationConnectorType::DualInline68Pin => "68 Pin Dual Inline",
        PortInformationConnectorType::OnBoardSoundInputCDRom => "On Board Sound Input From CD-ROM",
        PortInformationConnectorType::MiniCentronicsType14 => "Mini Centronics Type-14",
        PortInformationConnectorType::MiniCentronicsTyp26 => "Mini Centronics Type-26",
        PortInformationConnectorType::MiniJackHeadphones => "Mini Jack (headphones)",
        PortInformationConnectorType::Bnc => "BNC",
        PortInformationConnectorType::Port1394 => "IEEE 1394",
        PortInformationConnectorType::SasSataPlugReceptacle => "SAS/SATA Plug Receptacle",
        PortInformationConnectorType::UsbTypeCReceptacle => "USB Type-C Receptacle",
        PortInformationConnectorType::PC98 => "PC-98",
        PortInformationConnectorType::PC98Hireso => "PC-98 Hireso",
        PortInformationConnectorType::PCH88 => "PC-H98",
        PortInformationConnectorType::PC98Note => "PC-98 Note",
        PortInformationConnectorType::PC98Full => "PC-98 Full",
        PortInformationConnectorType::Other => OTHER,
        PortInformationConnectorType::None => "",
    };

    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, port_connector_type.raw),
        false => print.to_string(),
    }
}
pub fn dmi_port_type(port_type_data: &PortInformationPortTypeData) -> String {
    let print = match port_type_data.value {
        PortInformationPortType::NoPort => NONE,
        PortInformationPortType::ParallelPortXTATCompatible => "Parallel Port XT/AT Compatible",
        PortInformationPortType::ParallelPortPS2 => "Parallel Port PS/2",
        PortInformationPortType::ParallelPortEcp => "Parallel Port ECP",
        PortInformationPortType::ParallelPortEpp => "Parallel Port EPP",
        PortInformationPortType::ParallelPortEcpEpp => "Parallel Port ECP/EPP",
        PortInformationPortType::SerialPortXTATCompatible => "Serial Port XT/AT Compatible",
        PortInformationPortType::SerialPort16450Compatible => "Serial Port 16450 Compatible",
        PortInformationPortType::SerialPort16550Compatible => "Serial Port 16550 Compatible",
        PortInformationPortType::SerialPort16550ACompatible => "Serial Port 16550A Compatible",
        PortInformationPortType::ScsiPort => "SCSI Port",
        PortInformationPortType::MidiPort => "MIDI Port",
        PortInformationPortType::JoyStickPort => "Joystick Port",
        PortInformationPortType::KeyboardPort => "Keyboard Port",
        PortInformationPortType::MousePort => "Mouse Port",
        PortInformationPortType::SsaScsi => "SSA SCSI",
        PortInformationPortType::Usb => "USB",
        PortInformationPortType::Firewire => "Firewire (IEEE P1394)",
        PortInformationPortType::PcmciaTypeI => "PCMCIA Type I",
        PortInformationPortType::PcmcialTypeII => "PCMCIA Type II",
        PortInformationPortType::PcmciaTypeIii => "PCMCIA Type III",
        PortInformationPortType::Cardbus => "Cardbus",
        PortInformationPortType::AccessBusPort => "Access Bus Port",
        PortInformationPortType::ScsiII => "SCSI II",
        PortInformationPortType::ScsiWide => "SCSI Wide",
        PortInformationPortType::PC98 => "PC-98",
        PortInformationPortType::PC98Hireso => "PC-98 Hireso",
        PortInformationPortType::PCH98 => "PC-H98",
        PortInformationPortType::VideoPort => "Video Port",
        PortInformationPortType::AudioPort => "Audio Port",
        PortInformationPortType::ModemPort => "Modem Port",
        PortInformationPortType::NetworkPort => "Network Port",
        PortInformationPortType::Sata => "SATA",
        PortInformationPortType::Sas => "SAS",
        PortInformationPortType::Mfdp => " MFDP (Multi-Function Display Port)",
        PortInformationPortType::Thunderbolt => "Thunderbolt",
        PortInformationPortType::Port8251Compatible => "8251 Compatible",
        PortInformationPortType::Port8251FifoCompatible => "8251 FIFO Compatible",
        PortInformationPortType::Other => OTHER,
        PortInformationPortType::None => "",
    };

    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, port_type_data.raw),
        false => print.to_string(),
    }
}
pub fn dmi_slot_bus_width(width: &SlotWidthData) -> String {
    let print = match width.value {
        SlotWidth::Other => OTHER,
        SlotWidth::Unknown => UNKNOWN,
        SlotWidth::Bit8 => "8-bit",
        SlotWidth::Bit16 => "16-bit",
        SlotWidth::Bit32 => "32-bit",
        SlotWidth::Bit64 => "64-bit",
        SlotWidth::Bit128 => "128-bit",
        SlotWidth::X1 => "x1",
        SlotWidth::X2 => "x2",
        SlotWidth::X4 => "x4",
        SlotWidth::X8 => "x8",
        SlotWidth::X12 => "x12",
        SlotWidth::X16 => "x16",
        SlotWidth::X32 => "x32",
        SlotWidth::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, width.raw),
        false => print.to_string(),
    }
}
pub fn dmi_slot_type(system_slot_type: &SystemSlotTypeData) -> String {
    let print = match &system_slot_type.value {
        SystemSlotType::Other => OTHER,
        SystemSlotType::Unknown => UNKNOWN,
        SystemSlotType::Isa => "ISA",
        SystemSlotType::Mca => "MCA",
        SystemSlotType::Eisa => "EISA",
        SystemSlotType::Pci => "PCI",
        SystemSlotType::Pcmcia => "PC Card (PCMCIA)",
        SystemSlotType::VlVesa => "VLB",
        SystemSlotType::Proprietary => "Proprietary",
        SystemSlotType::ProcessorCardSlot => "Processor Card",
        SystemSlotType::ProprietaryMemoryCardSlot => "Proprietary Memory Card",
        SystemSlotType::IORiserCardSlot => "I/O Riser Card",
        SystemSlotType::NuBus => "NuBus",
        SystemSlotType::Pci66MhzCapable => "PCI-66",
        SystemSlotType::Agp(width) => match width {
            AgpSlotWidth::X1 => "AGP",
            AgpSlotWidth::X2 => "AGP 2x",
            AgpSlotWidth::X4 => "AGP 4x",
            AgpSlotWidth::X8 => "AGP 8x",
        },
        SystemSlotType::Mxm(slot_type) => match slot_type {
            MXMSlotType::MxmTypeI => "MXM Type I",
            MXMSlotType::MxmTypeII => "MXM Type II",
            MXMSlotType::MxmTypeIIIStandard => "MXM Type III",
            MXMSlotType::MxmTypeIIIHE => "MXM Type III-HE",
            MXMSlotType::MxmTypeIV => "MXM Type IV",
            MXMSlotType::Mxm3TypeA => "MXM 3.0 Type A",
            MXMSlotType::Mxm3TypeB => "MXM 3.0 Type B",
        },
        SystemSlotType::PciX => "PCI-X",
        SystemSlotType::M2(slot_type) => match slot_type {
            M2SlotType::M2Socket1DP => "M.2 Socket 1-DP",
            M2SlotType::M2Socket1SD => "M.2 Socket 1-SD",
            M2SlotType::M2Socket2 => "M.2 Socket 2",
            M2SlotType::M2Socket3 => "M.2 Socket 3",
        },
        SystemSlotType::OcpNic30SmallFormFactor => "OCP NIC 3.0 Small Form Factor (SFF)",
        SystemSlotType::OcpNic30LargeFormFactor => "OCP NIC 3.0 Large Form Factor (LFF)",
        SystemSlotType::OcpNicPriorTo30 => "OCP NIC Prior to 3.0",
        SystemSlotType::CxlFlexbus1 => "CXL FLexbus 1.0",
        SystemSlotType::PC98C20 => "PC-98/C20",
        SystemSlotType::PC98C24 => "PC-98/C24",
        SystemSlotType::PC98E => "PC-98/E",
        SystemSlotType::PC98LocalBus => "PC-98/Local Bus",
        SystemSlotType::PC98Card => "PC-98/Card",
        SystemSlotType::PciExpress(generation, width) => match (generation, width) {
            (PciExpressGeneration::PCIExpressGen1, PciExpressSlotWidth::UndefinedSlotWidth) => {
                "PCI Express"
            }
            (PciExpressGeneration::PCIExpressGen1, PciExpressSlotWidth::X1) => "PCI Express x1",
            (PciExpressGeneration::PCIExpressGen1, PciExpressSlotWidth::X2) => "PCI Express x2",
            (PciExpressGeneration::PCIExpressGen1, PciExpressSlotWidth::X4) => "PCI Express x4",
            (PciExpressGeneration::PCIExpressGen1, PciExpressSlotWidth::X8) => "PCI Express x8",
            (PciExpressGeneration::PCIExpressGen1, PciExpressSlotWidth::X16) => "PCI Express x16",
            (PciExpressGeneration::PCIExpressGen1, PciExpressSlotWidth::Sff8639) => "",
            (
                PciExpressGeneration::PCIExpressGen1,
                PciExpressSlotWidth::PciExpressMini52WithKeepouts,
            ) => "",
            (
                PciExpressGeneration::PCIExpressGen1,
                PciExpressSlotWidth::PciExpressMini52WithoutKeepouts,
            ) => "",
            (PciExpressGeneration::PCIExpressGen1, PciExpressSlotWidth::PciExpressMini76) => "",
            (PciExpressGeneration::PCIExpressGen2, PciExpressSlotWidth::UndefinedSlotWidth) => {
                "PCI Express 2"
            }
            (PciExpressGeneration::PCIExpressGen2, PciExpressSlotWidth::X1) => "PCI Express 2 x1",
            (PciExpressGeneration::PCIExpressGen2, PciExpressSlotWidth::X2) => "PCI Express 2 x2",
            (PciExpressGeneration::PCIExpressGen2, PciExpressSlotWidth::X4) => "PCI Express 2 x4",
            (PciExpressGeneration::PCIExpressGen2, PciExpressSlotWidth::X8) => "PCI Express 2 x8",
            (PciExpressGeneration::PCIExpressGen2, PciExpressSlotWidth::X16) => "PCI Express 2 x16",
            (PciExpressGeneration::PCIExpressGen2, PciExpressSlotWidth::Sff8639) => {
                "PCI Express 2 SFF-8639 (U.2)"
            }
            (
                PciExpressGeneration::PCIExpressGen2,
                PciExpressSlotWidth::PciExpressMini52WithKeepouts,
            ) => "",
            (
                PciExpressGeneration::PCIExpressGen2,
                PciExpressSlotWidth::PciExpressMini52WithoutKeepouts,
            ) => "",
            (PciExpressGeneration::PCIExpressGen2, PciExpressSlotWidth::PciExpressMini76) => "",
            (PciExpressGeneration::PCIExpressGen3, PciExpressSlotWidth::UndefinedSlotWidth) => {
                "PCI Express 3"
            }
            (PciExpressGeneration::PCIExpressGen3, PciExpressSlotWidth::X1) => "PCI Express 3 x1",
            (PciExpressGeneration::PCIExpressGen3, PciExpressSlotWidth::X2) => "PCI Express 3 x2",
            (PciExpressGeneration::PCIExpressGen3, PciExpressSlotWidth::X4) => "PCI Express 3 x4",
            (PciExpressGeneration::PCIExpressGen3, PciExpressSlotWidth::X8) => "PCI Express 3 x8",
            (PciExpressGeneration::PCIExpressGen3, PciExpressSlotWidth::X16) => "PCI Express 3 x16",
            (PciExpressGeneration::PCIExpressGen3, PciExpressSlotWidth::Sff8639) => {
                "PCI Express 3 SFF-8639 (U.2)"
            }
            (
                PciExpressGeneration::PCIExpressGen3,
                PciExpressSlotWidth::PciExpressMini52WithKeepouts,
            ) => "",
            (
                PciExpressGeneration::PCIExpressGen3,
                PciExpressSlotWidth::PciExpressMini52WithoutKeepouts,
            ) => "",
            (PciExpressGeneration::PCIExpressGen3, PciExpressSlotWidth::PciExpressMini76) => "",
            (PciExpressGeneration::PCIExpressGen4, PciExpressSlotWidth::UndefinedSlotWidth) => {
                "PCI Express 4"
            }
            (PciExpressGeneration::PCIExpressGen4, PciExpressSlotWidth::X1) => "PCI Express 4 x1",
            (PciExpressGeneration::PCIExpressGen4, PciExpressSlotWidth::X2) => "PCI Express 4 x2",
            (PciExpressGeneration::PCIExpressGen4, PciExpressSlotWidth::X4) => "PCI Express 4 x4",
            (PciExpressGeneration::PCIExpressGen4, PciExpressSlotWidth::X8) => "PCI Express 4 x8",
            (PciExpressGeneration::PCIExpressGen4, PciExpressSlotWidth::X16) => "PCI Express 4 x16",
            (PciExpressGeneration::PCIExpressGen4, PciExpressSlotWidth::Sff8639) => {
                "PCI Express 4 SFF-8639 (U.2)"
            }
            (
                PciExpressGeneration::PCIExpressGen4,
                PciExpressSlotWidth::PciExpressMini52WithKeepouts,
            ) => "",
            (
                PciExpressGeneration::PCIExpressGen4,
                PciExpressSlotWidth::PciExpressMini52WithoutKeepouts,
            ) => "",
            (PciExpressGeneration::PCIExpressGen4, PciExpressSlotWidth::PciExpressMini76) => "",
            (PciExpressGeneration::PCIExpressGen5, PciExpressSlotWidth::UndefinedSlotWidth) => {
                "PCI Express 5"
            }
            (PciExpressGeneration::PCIExpressGen5, PciExpressSlotWidth::X1) => "PCI Express 5 x1",
            (PciExpressGeneration::PCIExpressGen5, PciExpressSlotWidth::X2) => "PCI Express 5 x2",
            (PciExpressGeneration::PCIExpressGen5, PciExpressSlotWidth::X4) => "PCI Express 5 x4",
            (PciExpressGeneration::PCIExpressGen5, PciExpressSlotWidth::X8) => "PCI Express 5 x8",
            (PciExpressGeneration::PCIExpressGen5, PciExpressSlotWidth::X16) => "PCI Express 5 x16",
            (PciExpressGeneration::PCIExpressGen5, PciExpressSlotWidth::Sff8639) => {
                "PCI Express 5 SFF-8639 (U.2)"
            }
            (
                PciExpressGeneration::PCIExpressGen5,
                PciExpressSlotWidth::PciExpressMini52WithKeepouts,
            ) => "",
            (
                PciExpressGeneration::PCIExpressGen5,
                PciExpressSlotWidth::PciExpressMini52WithoutKeepouts,
            ) => "",
            (PciExpressGeneration::PCIExpressGen5, PciExpressSlotWidth::PciExpressMini76) => "",
            (PciExpressGeneration::PCIExpressGen6, PciExpressSlotWidth::UndefinedSlotWidth) => {
                "PCI Express 6+"
            }
            (PciExpressGeneration::PCIExpressGen6, PciExpressSlotWidth::X1) => "",
            (PciExpressGeneration::PCIExpressGen6, PciExpressSlotWidth::X2) => "",
            (PciExpressGeneration::PCIExpressGen6, PciExpressSlotWidth::X4) => "",
            (PciExpressGeneration::PCIExpressGen6, PciExpressSlotWidth::X8) => "",
            (PciExpressGeneration::PCIExpressGen6, PciExpressSlotWidth::X16) => "",
            (PciExpressGeneration::PCIExpressGen6, PciExpressSlotWidth::Sff8639) => "",
            (
                PciExpressGeneration::PCIExpressGen6,
                PciExpressSlotWidth::PciExpressMini52WithKeepouts,
            ) => "",
            (
                PciExpressGeneration::PCIExpressGen6,
                PciExpressSlotWidth::PciExpressMini52WithoutKeepouts,
            ) => "",
            (PciExpressGeneration::PCIExpressGen6, PciExpressSlotWidth::PciExpressMini76) => "",
            (PciExpressGeneration::Undefined, PciExpressSlotWidth::UndefinedSlotWidth) => "",
            (PciExpressGeneration::Undefined, PciExpressSlotWidth::X1) => "",
            (PciExpressGeneration::Undefined, PciExpressSlotWidth::X2) => "",
            (PciExpressGeneration::Undefined, PciExpressSlotWidth::X4) => "",
            (PciExpressGeneration::Undefined, PciExpressSlotWidth::X8) => "",
            (PciExpressGeneration::Undefined, PciExpressSlotWidth::X16) => "",
            (PciExpressGeneration::Undefined, PciExpressSlotWidth::Sff8639) => "",
            (
                PciExpressGeneration::Undefined,
                PciExpressSlotWidth::PciExpressMini52WithKeepouts,
            ) => "PCI Express Mini 52-pin with bottom-side keep-outs",
            (
                PciExpressGeneration::Undefined,
                PciExpressSlotWidth::PciExpressMini52WithoutKeepouts,
            ) => "PCI Express Mini 52-pin without bottom-side keep-outs",
            (PciExpressGeneration::Undefined, PciExpressSlotWidth::PciExpressMini76) => {
                "PCI Express Mini 76-pin"
            }
        },
        SystemSlotType::EnterpriseAndDataCenter1UE1 => "EDSFF E1",
        SystemSlotType::EnterpriseAndDataCenter3InE3 => "EDSFF E3",
        SystemSlotType::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, system_slot_type.raw),
        false => print.to_string(),
    }
}
pub fn dmi_slot_current_usage(current_usage: &SlotCurrentUsageData) -> String {
    let print = match current_usage.value {
        SlotCurrentUsage::Other => OTHER,
        SlotCurrentUsage::Unknown => UNKNOWN,
        SlotCurrentUsage::Available => "Available",
        SlotCurrentUsage::InUse => "In Use",
        SlotCurrentUsage::Unavailable => "Unavailable",
        SlotCurrentUsage::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, current_usage.raw),
        false => print.to_string(),
    }
}
pub fn dmi_slot_length(slot_length: &SlotLengthData) -> String {
    let print = match slot_length.value {
        SlotLength::Other => OTHER,
        SlotLength::Unknown => UNKNOWN,
        SlotLength::ShortLength => "Short",
        SlotLength::LongLength => "Long",
        SlotLength::DriveFormFactor25 => "2.5\" drive form factor",
        SlotLength::DriveFormFactor35 => "3.5\" drive form factor",
        SlotLength::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, slot_length.raw),
        false => print.to_string(),
    }
}
pub fn dmi_slot_characteristics(
    attr: &str,
    characteristics1: &Option<SystemSlotCharacteristics1>,
    characteristics2: &Option<SystemSlotCharacteristics2>,
) {
    match (&characteristics1, &characteristics2) {
        (Some(c1), None) => {
            if c1.unknown() {
                println!("\t{}: Unknown", attr);
                return;
            } else if c1.raw & 0xFE == 0 {
                println!("\t{}: None", attr);
                return;
            }
        }
        (Some(c1), Some(c2)) => {
            if c1.unknown() {
                println!("\t{}: Unknown", attr);
                return;
            } else if c1.raw & 0xFE == 0 && c2.raw & 0x07 == 0 {
                println!("\t{}: None", attr);
                return;
            }
        }
        _ => return,
    }

    println!("\t{}:", attr);

    match &characteristics1 {
        Some(c1) => {
            if c1.provides5_volts() {
                println!("\t\t5.0 V is provided");
            }
            if c1.provides33_volts() {
                println!("\t\t3.3 V is provided");
            }
            if c1.shared() {
                println!("\t\tOpening is shared");
            }
            if c1.supports_pc_card16() {
                println!("\t\tPC Card-16 is supported");
            }
            if c1.supports_card_bus() {
                println!("\t\tCardbus is supported");
            }
            if c1.supports_zoom_video() {
                println!("\t\tZoom Video is supported");
            }
            if c1.supports_modem_ring_resume() {
                println!("\t\tModem ring resume is supported");
            }
        }
        None => (),
    }
    match &characteristics2 {
        Some(c2) => {
            if c2.supports_power_management_event() {
                println!("\t\tPME signal is supported");
            }
            if c2.supports_hot_plug_devices() {
                println!("\t\tHot-plug devices are supported");
            }
            if c2.supports_smbus_signal() {
                println!("\t\tSMBus signal is supported");
            }
            if c2.supports_bifurcation() {
                println!("\t\tPCIe slot bifurcation is supported");
            }
            if c2.supports_suprise_removal() {
                println!("\t\tAsync/surprise removal is supported");
            }
            if c2.flexbus_slot_cxl10_capable() {
                println!("\t\tFlexbus slot, CXL 1.0 capable");
            }
            if c2.flexbus_slot_cxl20_capable() {
                println!("\t\tFlexbus slot, CXL 2.0 capable");
            }
        }
        None => (),
    }
}
pub fn dmi_slot_segment_bus_func(
    segment_group_number: u16,
    bus_number: u8,
    device_function_number: u8,
) {
    if !(segment_group_number == u16::MAX
        && bus_number == u8::MAX
        && device_function_number == u8::MAX)
    {
        println!(
            "\tBus Address: {:04x}:{:02x}:{:02x}.{:x}",
            segment_group_number,
            bus_number,
            device_function_number >> 3,
            device_function_number & 0x7
        )
    }
}
pub fn dmi_on_board_devices_type(device_type: &OnBoardDeviceType) -> String {
    let print = match &device_type.type_of_device() {
        TypeOfDevice::Other => OTHER,
        TypeOfDevice::Unknown => UNKNOWN,
        TypeOfDevice::Video => "Video",
        TypeOfDevice::ScsiController => "SCSI Controller",
        TypeOfDevice::Ethernet => "Ethernet",
        TypeOfDevice::TokenRing => "Token Ring",
        TypeOfDevice::Sound => "Sound",
        TypeOfDevice::PataController => "PATA Controller",
        TypeOfDevice::SataController => "SATA Controller",
        TypeOfDevice::SasController => "SAS Controller",
        TypeOfDevice::None => "",
    };
    match print == "" {
        true => format!("{} ({})", OUT_OF_SPEC, &device_type.raw & 0x7F),
        false => print.to_string(),
    }
}
pub fn dmi_event_log_method(access_method: &AccessMethodData) -> String {
    let print = match access_method.value {
        AccessMethod::IndexedIO18Bit => "Indexed I/O, one 8-bit index port, one 8-bit data port",
        AccessMethod::IndexedIO28Bit => "Indexed I/O, two 8-bit index ports, one 8-bit data port",
        AccessMethod::IndexedIO116Bit => "Indexed I/O, one 16-bit index port, one 8-bit data port",
        AccessMethod::MemoryMapped32Bit => "Memory-mapped physical 32-bit address",
        AccessMethod::GeneralPurposeNonVolatile => "General-purpose non-volatile data functions",
        AccessMethod::None => "",
    };
    match print == "" {
        true => {
            if access_method.raw >= 0x80 {
                format!("OEM-specific ({})", access_method.raw)
            } else {
                format!("{} ({})", OUT_OF_SPEC, access_method.raw)
            }
        }
        false => print.to_string(),
    }
}
pub fn dmi_event_log_address(access_method: &AccessMethodData, address: u32) {
    let print_indexed = |addr: u32| {
        let bytes = address.to_le_bytes();
        let index = u16::from_le_bytes(bytes[0..2].try_into().expect("u16 is two bytes"));
        let data = u16::from_le_bytes(bytes[2..4].try_into().expect("u16 is two bytes"));
        println!("\tAccess Address: Index {:#06X}, Data {:#06X}", index, data);
    };
    match access_method.value {
        AccessMethod::IndexedIO18Bit => print_indexed(address),
        AccessMethod::IndexedIO28Bit => print_indexed(address),
        AccessMethod::IndexedIO116Bit => print_indexed(address),
        AccessMethod::MemoryMapped32Bit => println!("\tAccess Address: {:#10X}", address),
        AccessMethod::GeneralPurposeNonVolatile => {
            println!("\tAccess Address: {:#06X}", address & u16::MAX as u32)
        }
        AccessMethod::None => println!("\tAccess Address: Unknown"),
    };
}
pub fn dmi_event_log_header_type(header_format: &HeaderFormatData) -> String {
    let print = match header_format.value {
        HeaderFormat::NoHeader => "No Header",
        HeaderFormat::Type1LogHeader => "Type 1",
        HeaderFormat::None => "",
    };
    match print == "" {
        true => {
            if header_format.raw >= 0x80 {
                format!("OEM-specific ({})", header_format.raw)
            } else {
                format!("{} ({})", OUT_OF_SPEC, header_format.raw)
            }
        }
        false => print.to_string(),
    }
}
pub fn dmi_event_log_descriptor_type(log_type: &LogTypeData) -> String {
    let print = match log_type.value {
        LogType::SingleBitEccMemoryError => "Single-bit ECC memory error",
        LogType::MultiBitEccMemoryError => "Multi-bit ECC memory error",
        LogType::ParityMemoryError => "Parity memory error",
        LogType::BusTimeOut => "Bus timeout",
        LogType::IOChannelCheck => "I/O channel block",
        LogType::SoftwareNmi => "Software NMI",
        LogType::PostMemoryResize => "POST memory resize",
        LogType::PostError => "POST error",
        LogType::PciParityError => "PCI parity error",
        LogType::PciSystemError => "PCI system error",
        LogType::CpuFailure => "CPU failure",
        LogType::EisaFailSafeTimerTimeout => "EISA failsafe timer timeout",
        LogType::CorrectableMemoryLogDisabled => "Correctable memory log disabled",
        LogType::LoggingDisabledForSpecificEventType => "Logging disabled",
        LogType::Reserved0F => "Reserved (0x0F)",
        LogType::SystemLimitExceeded => "System limit exceeded",
        LogType::AsyncHardwareTimerExpired => "Asynchronous hardware timer expired",
        LogType::SystemConfigurationInformation => "System configuration information",
        LogType::HardDiskInformation => "Hard disk information",
        LogType::SystemReconfigured => "System reconfigured",
        LogType::UncorrectableCpuComplexError => "Uncorrectable CPU-complex error",
        LogType::LogAreaReset => "Log area reset/cleared",
        LogType::SystemBoot => "System boot",
        LogType::None => "",
    };
    match print == "" {
        true => {
            if log_type.raw >= 0x80 && log_type.raw <= 0xFE {
                format!("OEM-specific ({})", log_type.raw)
            } else if log_type.raw == 0xFF {
                format!("End of log")
            } else {
                format!("{} ({})", OUT_OF_SPEC, log_type.raw)
            }
        }
        false => print.to_string(),
    }
}
pub fn dmi_event_log_descriptor_format(data: &VariableDataFormatTypeData) -> String {
    let print = match data.value {
        VariableDataFormatType::NoStandardFormat => NONE,
        VariableDataFormatType::Handle => "Handle",
        VariableDataFormatType::MultipleEvent => "Multiple-event",
        VariableDataFormatType::MultipleEventHandle => "Multiple-event handle",
        VariableDataFormatType::PostResultsBitmap => "POST results bitmap",
        VariableDataFormatType::SystemManagementType => "System management",
        VariableDataFormatType::MultipleEventSystemManagementType => {
            "Multiple-event system management"
        }
        VariableDataFormatType::None => "",
    };
    match print == "" {
        true => {
            if data.raw >= 0x80 {
                format!("OEM-specific ({})", data.raw)
            } else {
                format!("{} ({})", OUT_OF_SPEC, data.raw)
            }
        }
        false => print.to_string(),
    }
}
pub fn dmi_pointing_device_type(device_type: &PointingDeviceTypeData) -> String {
    let print = match device_type.value {
        PointingDeviceType::Other => OTHER,
        PointingDeviceType::Unknown => UNKNOWN,
        PointingDeviceType::Mouse => "Mouse",
        PointingDeviceType::TrackBall => "Track Ball",
        PointingDeviceType::TrackPoint => "Track Point",
        PointingDeviceType::GlidePoint => "Glide Point",
        PointingDeviceType::TouchPad => "Touch Pad",
        PointingDeviceType::TouchScreen => "Touch Screen",
        PointingDeviceType::OpticalSensor => "Optical Sensor",
        PointingDeviceType::None => "",
    };
    match print == "" {
        true => {
            format!("{} ({})", OUT_OF_SPEC, device_type.raw)
        }
        false => print.to_string(),
    }
}
pub fn dmi_pointing_device_interface(interface: &PointingDeviceInterfaceData) -> String {
    let print = match interface.value {
        PointingDeviceInterface::Other => OTHER,
        PointingDeviceInterface::Unknown => UNKNOWN,
        PointingDeviceInterface::Serial => "Serial",
        PointingDeviceInterface::PS2 => "PS/2",
        PointingDeviceInterface::Infrared => "Infrared",
        PointingDeviceInterface::HpHil => "HIP-HIL",
        PointingDeviceInterface::BusMouse => "Bus Mouse",
        PointingDeviceInterface::Adb => "ADB (Apple Desktop Bus)",
        PointingDeviceInterface::BusMouseDB9 => "Bus Mouse DB-9",
        PointingDeviceInterface::BusMouseMicroDin => "Bus Mouse Micro DIN",
        PointingDeviceInterface::USB => "USB",
        PointingDeviceInterface::None => "",
    };
    match print == "" {
        true => {
            format!("{} ({})", OUT_OF_SPEC, interface.raw)
        }
        false => print.to_string(),
    }
}
pub fn dmi_battery_chemistry(chemistry: &PortableBatteryDeviceChemistryData) -> String {
    let print = match chemistry.value {
        PortableBatteryDeviceChemistry::Other => OTHER,
        PortableBatteryDeviceChemistry::Unknown => UNKNOWN,
        PortableBatteryDeviceChemistry::LeadAcit => "Lead Acid",
        PortableBatteryDeviceChemistry::NickelCadmium => "Nickel Cadmium",
        PortableBatteryDeviceChemistry::NickelMetalHydride => "Nickel Metal Hydride",
        PortableBatteryDeviceChemistry::LithiumIon => "Lithium Ion",
        PortableBatteryDeviceChemistry::ZincAir => "Zinc Air",
        PortableBatteryDeviceChemistry::LithiumPolymer => "Lithium Polymer",
        PortableBatteryDeviceChemistry::None => "",
    };
    match print == "" {
        true => {
            format!("{} ({})", OUT_OF_SPEC, chemistry.raw)
        }
        false => print.to_string(),
    }
}
pub fn dmi_battery_capacity(capacity: &PortableBatteryDesignCapacity, multiplier: u8) {
    print!("\tDesign Capacity: ");
    match capacity {
        PortableBatteryDesignCapacity::MilliWattHours(mwh) => {
            println!("{} mwh", mwh * multiplier as u16)
        }
        PortableBatteryDesignCapacity::Unknown => println!("{}", UNKNOWN),
    }
}
pub fn dmi_battery_voltage(voltage: &PortableBatteryDesignVoltage) {
    print!("\tDesign Voltage: ");
    match voltage {
        PortableBatteryDesignVoltage::MilliVolts(mv) => {
            println!("{} mV", mv)
        }
        PortableBatteryDesignVoltage::Unknown => println!("{}", UNKNOWN),
    }
}
pub fn dmi_battery_maximum_error(error: u8) {
    print!("\tMaximum Error: ");
    match error == 0xFF {
        true => println!("{}", UNKNOWN),
        false => println!("{}%", error),
    }
}
