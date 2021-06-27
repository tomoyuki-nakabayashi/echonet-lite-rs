use core::fmt;
use serde::{Deserialize, Serialize};

// TODO: add methods
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct EchonetObject{
    class: [u8; 2],
    instance: u8,
}

impl From<[u8; 3]> for EchonetObject {
    fn from(eobj: [u8; 3]) -> Self {
        Self {
            class: [eobj[0], eobj[1]],
            instance: eobj[2],
        }
    }
}

impl fmt::Display for EchonetObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:02X} {:02X} {:02X}]", self.class[0], self.class[1], self.instance)
    }
}
