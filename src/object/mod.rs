use core::fmt;
use serde::{Deserialize, Serialize};

mod code {
    pub const CONTROLLER: [u8; 2] = [0x05, 0xFE];
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
struct ClassCode([u8; 2]);

impl fmt::Display for ClassCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02X} {:02X}", self.0[0], self.0[1])
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
