use crate::{deserialize, ElPacket, Properties};
use core::fmt::{self, Formatter};
pub use property_maps::*;
use serde::{de::Visitor, ser::SerializeTuple, Deserialize, Serialize};

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
            ClassCode(code::HOME_AIR_CONDITIONER) => {
                ClassPacket::AirConditioner(AirConditionerPacket(props))
            }
            ClassCode(code::POWER_DISTRIBUTION_BOARD_METERING) => {
                ClassPacket::Metering(MeteringPacket(props))
            }
            ClassCode(code::FUEL_CELL) => ClassPacket::FuelCell(FuelCellPacket(props)),
            ClassCode(code::INSTANTANEOUS_WATER_HEATER) => {
                ClassPacket::InstantaneousWaterHeater(InstantaneousWaterHeaterPacket(props))
            }
            ClassCode(code::GENERAL_LIGHTING) => {
                ClassPacket::GeneralLighting(GeneralLightingPacket(props))
            }
            ClassCode(code::MONO_FUNCTION_LIGHTING) => {
                ClassPacket::MonoFunctionLighting(MonoFunctionLightingPacket(props))
            }
            ClassCode(code::LIGHTING_SYSTEM) => {
                ClassPacket::LightingSystem(LightingSystemPacket(props))
            }
            ClassCode(code::PROFILE) => ClassPacket::Profile(ProfilePacket(props)),
            _ => ClassPacket::Unimplemented(UnimplementedPacket(eoj.class, props)),
        }
    }

    /// fetches the properties for this ClassPacket, when appropriate.
    pub fn properties(&self) -> &Properties {
        match self {
            Self::Unimplemented(p) => p.properties(),
            Self::SolarPower(p) => p.properties(),
            Self::StorageBattery(p) => p.properties(),
            Self::Evps(p) => p.properties(),
            Self::Hp(p) => p.properties(),
            Self::SmartMeter(p) => p.properties(),
            Self::AirConditioner(p) => p.properties(),
            Self::Metering(p) => p.properties(),
            Self::FuelCell(p) => p.properties(),
            Self::InstantaneousWaterHeater(p) => p.properties(),
            Self::GeneralLighting(p) => p.properties(),
            Self::MonoFunctionLighting(p) => p.properties(),
            Self::LightingSystem(p) => p.properties(),
            Self::Profile(p) => p.properties(),
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
            ClassCode(code::POWER_DISTRIBUTION_BOARD_METERING) => {
                ClassPacket::Metering(value.into())
            }
            ClassCode(code::FUEL_CELL) => ClassPacket::FuelCell(value.into()),
            ClassCode(code::INSTANTANEOUS_WATER_HEATER) => {
                ClassPacket::InstantaneousWaterHeater(value.into())
            }
            ClassCode(code::GENERAL_LIGHTING) => ClassPacket::GeneralLighting(value.into()),
            ClassCode(code::MONO_FUNCTION_LIGHTING) => {
                ClassPacket::MonoFunctionLighting(value.into())
            }
            ClassCode(code::LIGHTING_SYSTEM) => ClassPacket::LightingSystem(value.into()),
            ClassCode(code::PROFILE) => ClassPacket::Profile(value.into()),
            _ => ClassPacket::Unimplemented(value.into()),
        }
    }
}

impl fmt::Display for ClassPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClassPacket::SolarPower(v) => write!(f, "{v}")?,
            ClassPacket::StorageBattery(v) => write!(f, "{v}")?,
            ClassPacket::Evps(v) => write!(f, "{v}")?,
            ClassPacket::Hp(v) => write!(f, "{v}")?,
            ClassPacket::SmartMeter(v) => write!(f, "{v}")?,
            ClassPacket::AirConditioner(v) => write!(f, "{v}")?,
            ClassPacket::Metering(v) => write!(f, "{v}")?,
            ClassPacket::FuelCell(v) => write!(f, "{v}")?,
            ClassPacket::InstantaneousWaterHeater(v) => write!(f, "{v}")?,
            ClassPacket::GeneralLighting(v) => write!(f, "{v}")?,
            ClassPacket::MonoFunctionLighting(v) => write!(f, "{v}")?,
            ClassPacket::LightingSystem(v) => write!(f, "{v}")?,
            ClassPacket::Profile(v) => write!(f, "{v}")?,
            ClassPacket::Unimplemented(v) => write!(f, "{v}")?,
        }
        Ok(())
    }
}

pub mod code {
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
    pub const CONTROLLER: [u8; 2] = [0x05, 0xFF];
    pub const PROFILE: [u8; 2] = [0x0E, 0xF0];
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
struct ClassCode([u8; 2]);

impl From<[u8; 2]> for ClassCode {
    fn from(value: [u8; 2]) -> Self {
        Self(value)
    }
}

impl fmt::Display for ClassCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02X} {:02X}", self.0[0], self.0[1])
    }
}

pub struct UnimplementedPacket(ClassCode, Properties);

impl UnimplementedPacket {
    pub fn properties(&self) -> &Properties {
        &self.1
    }
}

impl fmt::Display for UnimplementedPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Unimplemented Class: {}", self.0)?;
        for prop in self.1.iter() {
            if let Some(name) = SUPER_CLASS.get(&prop.epc) {
                writeln!(f, "{prop}\t\t[{name}]")?;
                continue;
            }
            writeln!(f, "{prop}\t\t[unknown]")?;
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

            /// Gets the properties used to construct this ClassPacket
            pub fn properties(&self) -> &Properties {
                &self.0
            }
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
                        writeln!(f, "{prop}\t\t[{name}]")?;
                        continue;
                    }
                    if let Some(name) = $class.get(&prop.epc) {
                        writeln!(f, "{prop}\t\t[{name}]")?;
                        continue;
                    }
                    writeln!(f, "{prop}\t\t[unknown]")?;
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
convert_packet!(
    code::HOME_AIR_CONDITIONER,
    AirConditionerPacket,
    HOME_AIR_CONDITIONER_CLASS,
    "Home Air Conditioner"
);

pub struct MeteringPacket(Properties);
convert_packet!(
    code::POWER_DISTRIBUTION_BOARD_METERING,
    MeteringPacket,
    POWER_DISTRIBUTION_BOARD_METERING_CLASS,
    "Power Distribution Board Metering"
);

pub struct FuelCellPacket(Properties);
convert_packet!(
    code::FUEL_CELL,
    FuelCellPacket,
    FUEL_CELL_CLASS,
    "Fuel Cell"
);

pub struct InstantaneousWaterHeaterPacket(Properties);
convert_packet!(
    code::INSTANTANEOUS_WATER_HEATER,
    InstantaneousWaterHeaterPacket,
    INSTANTANEOUS_WATER_HEATER_CLASS,
    "Instantaneous Water Heater"
);

pub struct GeneralLightingPacket(Properties);
convert_packet!(
    code::GENERAL_LIGHTING,
    GeneralLightingPacket,
    GENERAL_LIGHTING_CLASS,
    "General Lighting"
);

pub struct MonoFunctionLightingPacket(Properties);
convert_packet!(
    code::MONO_FUNCTION_LIGHTING,
    MonoFunctionLightingPacket,
    MONO_FUNCTION_LIGHTING_CLASS,
    "Mono Function Lighting"
);

pub struct LightingSystemPacket(Properties);
convert_packet!(
    code::LIGHTING_SYSTEM,
    LightingSystemPacket,
    LIGHTING_SYSTEM_CLASS,
    "Lighting System"
);

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

/// An ECHONET object.
///
/// ECHONET objects are described using the formats [X1.X2] and [X3].
/// - X1: Class group code
/// - X2: Class code
/// - X3: Instance code
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct EchonetObject {
    // TODO: use `ElClass` instead.
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

impl From<(ElClass, u8)> for EchonetObject {
    fn from(value: (ElClass, u8)) -> Self {
        let (class, instance) = value;
        use code::*;
        match class {
            ElClass::HomeAC => Self {
                class: HOME_AIR_CONDITIONER.into(),
                instance,
            },
            ElClass::Hp => Self {
                class: HP.into(),
                instance,
            },
            ElClass::InstantaneousWaterHeater => Self {
                class: INSTANTANEOUS_WATER_HEATER.into(),
                instance,
            },
            ElClass::Pv => Self {
                class: HOUSEHOLD_SOLAR_POWER.into(),
                instance,
            },
            ElClass::FuelCell => Self {
                class: FUEL_CELL.into(),
                instance,
            },
            ElClass::Battery => Self {
                class: STORAGE_BATTERY.into(),
                instance,
            },
            ElClass::Evps => Self {
                class: EVPS.into(),
                instance,
            },
            ElClass::Metering => Self {
                class: POWER_DISTRIBUTION_BOARD_METERING.into(),
                instance,
            },
            ElClass::SmartMeter => Self {
                class: SMART_METER.into(),
                instance,
            },
            ElClass::MultiInputPCS => Self {
                class: [0x02, 0xA5].into(),
                instance,
            },
            ElClass::GeneralLighting => Self {
                class: GENERAL_LIGHTING.into(),
                instance,
            },
            ElClass::MonoFunctionLighting => Self {
                class: MONO_FUNCTION_LIGHTING.into(),
                instance,
            },
            ElClass::LightingSystem => Self {
                class: LIGHTING_SYSTEM.into(),
                instance,
            },
            ElClass::Controller => Self {
                class: CONTROLLER.into(),
                instance,
            },
            ElClass::Profile => Self {
                class: PROFILE.into(),
                instance,
            },
            ElClass::Unknown(code) => Self {
                class: code.into(),
                instance,
            },
        }
    }
}

impl TryFrom<&[u8]> for EchonetObject {
    type Error = crate::error::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let (_, eobj) = deserialize(value)?;
        Ok(eobj)
    }
}

impl fmt::Display for EchonetObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use code::*;
        let class = match self.class.0 {
            HOME_AIR_CONDITIONER => "Home AC",
            HP => "Heat pump",
            INSTANTANEOUS_WATER_HEATER => "Instantaneous water heater",
            HOUSEHOLD_SOLAR_POWER => "Household solar power",
            FUEL_CELL => "Fuel cell",
            STORAGE_BATTERY => "Storage battery",
            EVPS => "V2H",
            POWER_DISTRIBUTION_BOARD_METERING => "Power distribution Metering",
            SMART_METER => "Smart meter",
            GENERAL_LIGHTING => "General lighting",
            MONO_FUNCTION_LIGHTING => "Mono function lighting",
            LIGHTING_SYSTEM => "Lighting system",
            CONTROLLER => "Controller",
            PROFILE => "Profile",
            _ => "Unknown",
        };
        write!(f, "{} [{} {:02X}]", class, self.class, self.instance)
    }
}

/// echonet-lite class representation.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ElClass {
    HomeAC,
    Hp,
    InstantaneousWaterHeater,
    Pv,
    FuelCell,
    Battery,
    Evps,
    Metering,
    SmartMeter,
    MultiInputPCS,
    GeneralLighting,
    MonoFunctionLighting,
    LightingSystem,
    Controller,
    Profile,
    Unknown([u8; 2]),
}

impl Serialize for ElClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let raw = Into::<[u8; 2]>::into(*self);
        let mut seq = serializer.serialize_tuple(2)?;
        seq.serialize_element(&raw[0])?;
        seq.serialize_element(&raw[1])?;
        seq.end()
    }
}

struct ElClassVisitor;
impl<'de> Visitor<'de> for ElClassVisitor {
    type Value = (u8, u8);

    fn expecting(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        formatter.write_str("never failed")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let group: u8 = seq.next_element()?.unwrap();
        let class: u8 = seq.next_element()?.unwrap();
        Ok((group, class))
    }
}

impl<'de> Deserialize<'de> for ElClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (group, class) = deserializer.deserialize_tuple(2, ElClassVisitor)?;
        Ok(ElClass::from(&[group, class]))
    }
}

impl From<&[u8; 2]> for ElClass {
    fn from(value: &[u8; 2]) -> Self {
        use code::*;
        use ElClass::*;
        match *value {
            HOME_AIR_CONDITIONER => HomeAC,
            HP => Hp,
            INSTANTANEOUS_WATER_HEATER => InstantaneousWaterHeater,
            HOUSEHOLD_SOLAR_POWER => Pv,
            FUEL_CELL => FuelCell,
            STORAGE_BATTERY => Battery,
            EVPS => Evps,
            POWER_DISTRIBUTION_BOARD_METERING => Metering,
            SMART_METER => SmartMeter,
            [0x02, 0xA5] => MultiInputPCS,
            GENERAL_LIGHTING => GeneralLighting,
            MONO_FUNCTION_LIGHTING => MonoFunctionLighting,
            LIGHTING_SYSTEM => LightingSystem,
            CONTROLLER => Controller,
            PROFILE => Profile,
            _ => Unknown(*value),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<[u8; 2]> for ElClass {
    fn into(self) -> [u8; 2] {
        use code::*;
        use ElClass::*;
        match self {
            HomeAC => HOME_AIR_CONDITIONER,
            Hp => HP,
            InstantaneousWaterHeater => INSTANTANEOUS_WATER_HEATER,
            Pv => HOUSEHOLD_SOLAR_POWER,
            FuelCell => FUEL_CELL,
            Battery => STORAGE_BATTERY,
            Evps => EVPS,
            Metering => POWER_DISTRIBUTION_BOARD_METERING,
            SmartMeter => SMART_METER,
            MultiInputPCS => [0x02, 0xA5],
            GeneralLighting => GENERAL_LIGHTING,
            MonoFunctionLighting => MONO_FUNCTION_LIGHTING,
            LightingSystem => LIGHTING_SYSTEM,
            Controller => CONTROLLER,
            Profile => PROFILE,
            Unknown(raw) => raw,
        }
    }
}

impl From<EchonetObject> for ElClass {
    fn from(value: EchonetObject) -> Self {
        use code::*;
        use ElClass::*;
        match value.class.0 {
            HOME_AIR_CONDITIONER => HomeAC,
            HP => Hp,
            INSTANTANEOUS_WATER_HEATER => InstantaneousWaterHeater,
            HOUSEHOLD_SOLAR_POWER => Pv,
            FUEL_CELL => FuelCell,
            STORAGE_BATTERY => Battery,
            EVPS => Evps,
            POWER_DISTRIBUTION_BOARD_METERING => Metering,
            SMART_METER => SmartMeter,
            [0x02, 0xA5] => MultiInputPCS,
            GENERAL_LIGHTING => GeneralLighting,
            MONO_FUNCTION_LIGHTING => MonoFunctionLighting,
            LIGHTING_SYSTEM => LightingSystem,
            CONTROLLER => Controller,
            PROFILE => Profile,
            _ => Unknown(value.class.0),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_elclass() {
        let eobj = EchonetObject::from([0x01, 0x30, 0x01]);
        let class = ElClass::from(eobj);
        assert_eq!(class, ElClass::HomeAC);

        let raw = [0x01u8, 0x30u8];
        let class = ElClass::from(&raw);
        assert_eq!(class, ElClass::HomeAC);
    }

    #[test]
    fn to_echonet_object() {
        let class = ElClass::HomeAC;
        let eobj: EchonetObject = (class, 1u8).into();
        assert_eq!(
            eobj,
            EchonetObject {
                class: ClassCode([0x01, 0x30]),
                instance: 1
            }
        );
    }

    #[test]
    fn serialize_el_class() {
        let class = ElClass::HomeAC;
        let bytes = crate::ser::serialize(&class).unwrap();
        assert_eq!(bytes, vec![0x01, 0x30]);
    }

    #[test]
    fn deserialize_el_class() {
        let input = [0x01, 0x30];
        let (bytes_read, class): (usize, ElClass) = deserialize(&input).unwrap();
        assert_eq!(bytes_read, 2);
        assert_eq!(class, ElClass::HomeAC);
    }
}
