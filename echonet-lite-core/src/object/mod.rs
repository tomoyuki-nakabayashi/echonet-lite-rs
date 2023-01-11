use crate::{ElPacket, Properties};
use core::fmt;
use serde::{Deserialize, Serialize};
pub use property_maps::*;

mod property_maps;

/// Packet specified to an ECHONET class.
pub enum ClassPacket {
    /// Any unimplemented class fallback
    Unimplemented(UnimplementedPacket),
    /// House hold solar power class packet
    SolarPower(SolarPowerPacket),
    /// Storage battery class packet
    StorageBattery(StorageBatteryPacket),
    /// Electric vehicle charger/discharger class packet
    Evps(EvpsPacket),
    /// Heat pump
    Hp(HpPacket),
    /// Smart Meter class packet
    SmartMeter(SmartMeterPacket),
    /// Home Air Conditioner class packet
    AirConditioner(AirConditionerPacket),
    /// Power Distribution Board Metering class packet
    Metering(MeteringPacket),
    /// Fuel Cell class packet
    FuelCell(FuelCellPacket),
    /// Instantaneous Water Heater class packet
    InstantaneousWaterHeater(InstantaneousWaterHeaterPacket),
    /// General Lighting class packet
    GeneralLighting(GeneralLightingPacket),
    /// Mono Function Lighting class packet
    MonoFunctionLighting(MonoFunctionLightingPacket),
    /// Lighting System class packet
    LightingSystem(LightingSystemPacket),
    /// Node profile class packet
    Profile(ProfilePacket),
}

impl ClassPacket {
    pub fn new(eoj: EchonetObject, props: Properties) -> ClassPacket {
        match eoj.class {
            ClassCode(code::HOUSEHOLD_SOLAR_POWER) => {
                ClassPacket::SolarPower(SolarPowerPacket(props))
            }
            ClassCode(code::STORAGE_BATTERY) => {
                ClassPacket::StorageBattery(StorageBatteryPacket(props))
            }
            ClassCode(code::EVPS) => ClassPacket::Evps(EvpsPacket(props)),
            ClassCode(code::HP) => ClassPacket::Hp(HpPacket(props)),
            ClassCode(code::SMART_METER) => ClassPacket::SmartMeter(SmartMeterPacket(props)),
            ClassCode(code::HOME_AIR_CONDITIONER) => ClassPacket::AirConditioner(AirConditionerPacket(props)),
            ClassCode(code::POWER_DISTRIBUTION_BOARD_METERING) => ClassPacket::Metering(MeteringPacket(props)),
            ClassCode(code::FUEL_CELL) => ClassPacket::FuelCell(FuelCellPacket(props)),
            ClassCode(code::INSTANTANEOUS_WATER_HEATER) => ClassPacket::InstantaneousWaterHeater(InstantaneousWaterHeaterPacket(props)),
            ClassCode(code::GENERAL_LIGHTING) => ClassPacket::GeneralLighting(GeneralLightingPacket(props)),
            ClassCode(code::MONO_FUNCTION_LIGHTING) => ClassPacket::MonoFunctionLighting(MonoFunctionLightingPacket(props)),
            ClassCode(code::LIGHTING_SYSTEM) => ClassPacket::LightingSystem(LightingSystemPacket(props)),
            ClassCode(code::PROFILE) => ClassPacket::Profile(ProfilePacket(props)),
            _ => ClassPacket::Unimplemented(UnimplementedPacket(eoj.class, props)),
        }
    }
}

impl From<ElPacket> for ClassPacket {
    fn from(value: ElPacket) -> Self {
        match value.seoj.class {
            ClassCode(code::HOUSEHOLD_SOLAR_POWER) => ClassPacket::SolarPower(value.into()),
            ClassCode(code::STORAGE_BATTERY) => ClassPacket::StorageBattery(value.into()),
            ClassCode(code::EVPS) => ClassPacket::Evps(value.into()),
            ClassCode(code::HP) => ClassPacket::Hp(value.into()),
            ClassCode(code::SMART_METER) => ClassPacket::SmartMeter(value.into()),
            ClassCode(code::HOME_AIR_CONDITIONER) => ClassPacket::AirConditioner(value.into()),
            ClassCode(code::POWER_DISTRIBUTION_BOARD_METERING) => ClassPacket::Metering(value.into()),
            ClassCode(code::FUEL_CELL) => ClassPacket::FuelCell(value.into()),
            ClassCode(code::INSTANTANEOUS_WATER_HEATER) => ClassPacket::InstantaneousWaterHeater(value.into()),
            ClassCode(code::GENERAL_LIGHTING) => ClassPacket::GeneralLighting(value.into()),
            ClassCode(code::MONO_FUNCTION_LIGHTING) => ClassPacket::MonoFunctionLighting(value.into()),
            ClassCode(code::LIGHTING_SYSTEM) => ClassPacket::LightingSystem(value.into()),
            ClassCode(code::PROFILE) => ClassPacket::Profile(value.into()),
            _ => ClassPacket::Unimplemented(value.into()),
        }
    }
}

impl fmt::Display for ClassPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClassPacket::SolarPower(v) => write!(f, "{}", v)?,
            ClassPacket::StorageBattery(v) => write!(f, "{}", v)?,
            ClassPacket::Evps(v) => write!(f, "{}", v)?,
            ClassPacket::Hp(v) => write!(f, "{}", v)?,
            ClassPacket::SmartMeter(v) => write!(f, "{}", v)?,
            ClassPacket::AirConditioner(v) => write!(f, "{}", v)?,
            ClassPacket::Metering(v) => write!(f, "{}", v)?,
            ClassPacket::FuelCell(v) => write!(f, "{}", v)?,
            ClassPacket::InstantaneousWaterHeater(v) => write!(f, "{}", v)?,
            ClassPacket::GeneralLighting(v) => write!(f, "{}", v)?,
            ClassPacket::MonoFunctionLighting(v) => write!(f, "{}", v)?,
            ClassPacket::LightingSystem(v) => write!(f, "{}", v)?,
            ClassPacket::Profile(v) => write!(f, "{}", v)?,
            ClassPacket::Unimplemented(v) => write!(f, "{}", v)?,
        }
        Ok(())
    }
}

mod code {
    pub const HOME_AIR_CONDITIONER: [u8; 2] = [0x01, 0x30];
    pub const INSTANTANEOUS_WATER_HEATER: [u8; 2] = [0x02, 0x72];
    pub const HOUSEHOLD_SOLAR_POWER: [u8; 2] = [0x02, 0x79];
    pub const FUEL_CELL: [u8; 2] = [0x02, 0x7C];
    pub const STORAGE_BATTERY: [u8; 2] = [0x02, 0x7D];
    pub const EVPS: [u8; 2] = [0x02, 0x7E];
    pub const HP: [u8; 2] = [0x02, 0x6B];
    pub const POWER_DISTRIBUTION_BOARD_METERING: [u8; 2] = [0x02, 0x87];
    pub const SMART_METER: [u8; 2] = [0x02, 0x88];
    pub const GENERAL_LIGHTING: [u8; 2] = [0x02, 0x90];
    pub const MONO_FUNCTION_LIGHTING: [u8; 2] = [0x02, 0x91];
    pub const LIGHTING_SYSTEM: [u8; 2] = [0x02, 0xA3];
    pub const CONTROLLER: [u8; 2] = [0x05, 0xFE];
    pub const PROFILE: [u8; 2] = [0x0E, 0xF0];
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
struct ClassCode([u8; 2]);

impl fmt::Display for ClassCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02X} {:02X}", self.0[0], self.0[1])
    }
}

pub struct UnimplementedPacket(ClassCode, Properties);
impl fmt::Display for UnimplementedPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Unimplemented Class: {}", self.0)?;
        for prop in self.1.iter() {
            if let Some(name) = SUPER_CLASS.get(&prop.epc) {
                writeln!(f, "[{}]\t {}", name, prop)?;
                continue;
            }
            writeln!(f, "[unknown]\t {}", prop)?;
        }
        Ok(())
    }
}

impl From<ElPacket> for UnimplementedPacket {
    fn from(value: ElPacket) -> Self {
        UnimplementedPacket(value.seoj.class, value.props)
    }
}

macro_rules! convert_packet {
    ( $code:expr, $ty:ty, $class:expr, $class_desc:expr) => {
        impl $ty {
            #[allow(dead_code)]
            const CODE: [u8; 2] = $code;
        }

        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                writeln!(
                    f,
                    "{}: 0x{:02X}{:02X}",
                    $class_desc,
                    Self::CODE[0],
                    Self::CODE[1]
                )?;
                for prop in self.0.iter() {
                    if let Some(name) = SUPER_CLASS.get(&prop.epc) {
                        writeln!(f, "[{}]\t {}", name, prop)?;
                        continue;
                    }
                    if let Some(name) = $class.get(&prop.epc) {
                        writeln!(f, "[{}]\t {}", name, prop)?;
                        continue;
                    }
                    writeln!(f, "[unknown]\t {}", prop)?;
                }
                Ok(())
            }
        }

        impl From<ElPacket> for $ty {
            fn from(value: ElPacket) -> Self {
                if value.seoj.class != ClassCode(Self::CODE) {
                    panic!("invalid source object class.")
                }
                Self(value.props)
            }
        }
    };
}

pub struct SmartMeterPacket(Properties);
convert_packet!(
    code::SMART_METER,
    SmartMeterPacket,
    SMART_METER_CLASS,
    "Smart Meter"
);

pub struct SolarPowerPacket(Properties);
convert_packet!(
    code::HOUSEHOLD_SOLAR_POWER,
    SolarPowerPacket,
    HOUSEHOLD_SOLAR_POWER_CLASS,
    "House Hold Solar Power"
);
pub struct StorageBatteryPacket(Properties);
convert_packet!(
    code::STORAGE_BATTERY,
    StorageBatteryPacket,
    STORAGE_BATTERY_CLASS,
    "Storage Battery"
);

pub struct EvpsPacket(Properties);
convert_packet!(code::EVPS, EvpsPacket, EVPS_CLASS, "EVPS");

pub struct HpPacket(Properties);
convert_packet!(code::HP, HpPacket, HP_CLASS, "HP");

pub struct AirConditionerPacket(Properties);
convert_packet!(code::HOME_AIR_CONDITIONER, AirConditionerPacket, HOME_AIR_CONDITIONER_CLASS, "Home Air Conditioner");

pub struct MeteringPacket(Properties);
convert_packet!(code::POWER_DISTRIBUTION_BOARD_METERING, MeteringPacket, POWER_DISTRIBUTION_BOARD_METERING_CLASS, "Power Distribution Board Metering");

pub struct FuelCellPacket(Properties);
convert_packet!(code::FUEL_CELL, FuelCellPacket, FUEL_CELL_CLASS, "Fuel Cell");

pub struct InstantaneousWaterHeaterPacket(Properties);
convert_packet!(code::INSTANTANEOUS_WATER_HEATER, InstantaneousWaterHeaterPacket, INSTANTANEOUS_WATER_HEATER_CLASS, "Instantaneous Water Heater");

pub struct GeneralLightingPacket(Properties);
convert_packet!(code::GENERAL_LIGHTING, GeneralLightingPacket, GENERAL_LIGHTING_CLASS, "General Lighting");

pub struct MonoFunctionLightingPacket(Properties);
convert_packet!(code::MONO_FUNCTION_LIGHTING, MonoFunctionLightingPacket, MONO_FUNCTION_LIGHTING_CLASS, "Mono Function Lighting");

pub struct LightingSystemPacket(Properties);
convert_packet!(code::LIGHTING_SYSTEM, LightingSystemPacket, LIGHTING_SYSTEM_CLASS, "Lighting System");

pub struct ProfilePacket(Properties);
convert_packet!(code::PROFILE, ProfilePacket, PROFILE_CLASS, "Node Profile");

pub struct Controller;
impl Controller {
    #[allow(dead_code)]
    const CODE: [u8; 2] = code::CONTROLLER;
}

impl fmt::Display for Controller {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Controller: 0x{:02X}{:02X}",
            Self::CODE[0],
            Self::CODE[1]
        )
    }
}

enum Class {
    Controller(Controller),
}

impl From<ClassCode> for Class {
    fn from(code: ClassCode) -> Self {
        match code.0 {
            code::CONTROLLER => Class::Controller(Controller),
            _ => {
                todo!()
            }
        }
    }
}

impl From<EchonetObject> for Class {
    fn from(obj: EchonetObject) -> Self {
        Self::from(obj.class)
    }
}

/// An ECHONET object.
///
/// ECHONET objects are described using the formats [X1.X2] and [X3].
/// - X1: Class group code
/// - X2: Class code
/// - X3: Instance code
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct EchonetObject {
    class: ClassCode,
    instance: u8,
}

impl From<[u8; 3]> for EchonetObject {
    fn from(eobj: [u8; 3]) -> Self {
        Self {
            class: ClassCode([eobj[0], eobj[1]]),
            instance: eobj[2],
        }
    }
}

impl fmt::Display for EchonetObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} {:02X}]", self.class, self.instance)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn conversion() {
        let obj: EchonetObject = [0x05, 0xFE, 0x01].into();
        let _class: Class = obj.class.into();
        let _class: Class = obj.into();
    }
}
