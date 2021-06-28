use core::fmt;
use core::convert::TryFrom;
use serde::{Deserialize, Serialize};
use phf::{phf_map};
use crate::{Properties, ElPacket};

mod code {
    pub const STORAGE_BATTERY: [u8; 2] = [0x02, 0x7D];
    pub const CONTROLLER: [u8; 2] = [0x05, 0xFE];
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
struct ClassCode([u8; 2]);

impl fmt::Display for ClassCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02X} {:02X}", self.0[0], self.0[1])
    }
}

static SUPER_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0x80u8 => "動作状態",
};

pub struct StorageBatteryPacket(Properties);
impl StorageBatteryPacket {
    #[allow(dead_code)]
    const CODE: [u8; 2] = code::STORAGE_BATTERY;
}

impl fmt::Display for StorageBatteryPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "StorageBattery: 0x{:02X}{:02X}", Self::CODE[0], Self::CODE[1])?;
        for prop in self.0.0.iter() {
            if let Some(name) = SUPER_CLASS.get(&prop.epc) {
                writeln!(f, "[{}]\t {}", name, prop)?;
                continue
            }
        }
        Ok(())
    }
}

impl TryFrom<ElPacket> for StorageBatteryPacket {
    // TODO: エラー処理真面目に
    type Error = core::convert::Infallible;
    fn try_from(value: ElPacket) -> Result<Self, Self::Error> {
        if value.seoj.class == ClassCode(code::STORAGE_BATTERY) {
            return Ok(StorageBatteryPacket(value.props))
        }
        todo!()
    }
}

pub struct Controller;
impl Controller {
    #[allow(dead_code)]
    const CODE: [u8; 2] = code::CONTROLLER;
}

impl fmt::Display for Controller {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Controller: 0x{:02X}{:02X}", Self::CODE[0], Self::CODE[1])
    }
}

enum Class {
    Controller(Controller),
}

impl From<ClassCode> for Class {
    fn from(code: ClassCode) -> Self {
        match &code.0 {
            &code::CONTROLLER => Class::Controller(Controller),
            _ => { todo!() }
        }
    }
}

impl From<EchonetObject> for Class {
    fn from(obj: EchonetObject) -> Self {
        Self::from(obj.class)
    }
}

// TODO: add methods
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct EchonetObject{
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
