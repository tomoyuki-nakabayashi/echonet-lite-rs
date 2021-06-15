#![allow(dead_code)]
use alloc::vec::Vec;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

use crate::{Error, de, ser};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ElPacket {
    ehd1: u8,
    ehd2: u8,
    transaction_id: u16,
    seoj: EchonetObject,
    deoj: EchonetObject,
    esv: ServiceCode,
    props: Properties,
}

impl ElPacket {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        ser::serialize(&self)
    }

    fn from_bytes(bytes: &[u8]) -> Result<(usize, ElPacket), Error> {
        de::deserialize(bytes)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ServiceCode {
    SetISNA = 0x50,
    SetCSNA = 0x51,
    GetSNA = 0x52,
    INFSNA = 0x53,
    SetGetSNA = 0x5E,
    SetI = 0x60,
    SetC = 0x61,
    Get = 0x62,
    INFREQ = 0x63,
    SetGet = 0x6E,
    SetRes = 0x71,
    GetRes = 0x72,
    INF = 0x73,
    INFC = 0x74,
    INFCRes = 0x7A,
    SetGetRes = 0x7E,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EchonetObject([u8; 3]);
impl From<[u8; 3]> for EchonetObject {
    fn from(eobj: [u8; 3]) -> Self {
        Self(eobj)
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Properties(Vec<Property>);

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Property {
    epc: u8,
    edt: Edt,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Edt(Vec<u8>);

#[derive(Debug)]
pub struct ElPacketBuilder {
    transaction_id: u16, // builder 作るときに渡しても良いかも
    seoj: EchonetObject,
    deoj: EchonetObject,
    esv: Option<ServiceCode>,
    props: Properties,
}

impl ElPacketBuilder {
    pub fn new() -> Self {
        Self {
            transaction_id: 0,
            seoj: Default::default(),
            deoj: Default::default(),
            esv: None,
            props: Default::default(),
        }
    }

    pub fn transaction_id(mut self, tid: u16) -> Self {
        self.transaction_id = tid;
        self
    }

    pub fn seoj<T>(mut self, seoj: T) -> Self
    where
        T: Into<EchonetObject>,
    {
        self.seoj = seoj.into();
        self
    }

    pub fn deoj<T>(mut self, deoj: T) -> Self
    where
        T: Into<EchonetObject>,
    {
        self.deoj = deoj.into();
        self
    }

    pub fn esv(mut self, esv: ServiceCode) -> Self {
        self.esv = Some(esv);
        self
    }

    pub fn props(mut self, props: Properties) -> Self {
        self.props = props;
        self
    }

    pub fn build(self) -> ElPacket {
        ElPacket {
            ehd1: 0x10,
            ehd2: 0x81,
            transaction_id: self.transaction_id,
            seoj: self.seoj,
            deoj: self.deoj,
            esv: self.esv.unwrap(), // TODO: define error
            props: self.props,
        }
    }
}

macro_rules! prop {
    ( $epc:expr, [ $( $edt:expr ),* ] ) => {
        {
            let mut bytes: Vec<u8> = Vec::new();
            $(
                bytes.push($edt);
            )*
            Property{ epc: $epc, edt: Edt(bytes) }
        }
    };
}

// TODO: props!(0x80, [0x31])
// props!([0x80, [1, 2, 3]], [0x81, [1, 2, 3]])
macro_rules! props {
    ( $( [ $epc:expr, [ $( $edt:expr ),* ] ] ),* ) => {
        {
            let mut props: Vec<Property> = Vec::new();
            $(
                props.push( prop!($epc, [ $( $edt ),* ] ) );
            ),*
            Properties(props)
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::de;

    #[test]
    fn serialize() {
        let props = props!( [ 0x80, [0x02] ], [0x81, [0x01]] );
        let result = ElPacketBuilder::new()
            .transaction_id(1)
            .esv(ServiceCode::Get)
            .seoj([0xefu8, 0xffu8, 0x01u8])
            .deoj([0x03u8, 0x08u8, 0x01u8])
            .props(props)
            .build()
            .serialize()
            .unwrap();
        assert_eq!(
            vec![0x10, 0x81, 0, 1, 0xef, 0xff, 0x01, 0x03, 0x08, 0x01, 0x62, 1, 0x80, 0x01, 0x02],
            result
        );
    }

    #[test]
    fn deserialize() {
        let input: Vec<u8> = vec![0x10, 0x81, 0, 1, 0xef, 0xff, 0x01, 0x03, 0x08, 0x01, 0x62, 1, 0x80, 0x01, 0x02];
        let (consumed, decoded): (usize, ElPacket) = ElPacket::from_bytes(&input).unwrap();

        let prop = Property {
            epc: 0x80,
            edt: Edt(vec![0x02]),
        };
        let expect  = 
            ElPacketBuilder::new()
                .transaction_id(1)
                .esv(ServiceCode::Get)
                .seoj(EchonetObject([0xef, 0xff, 0x01]))
                .deoj(EchonetObject([0x03, 0x08, 0x01]))
                .props(Properties(vec![prop]))
                .build();

        assert_eq!(15, consumed);
        assert_eq!(expect, decoded);
    }

    #[test]
    fn deserialize_tid() {
        let input = [0u8, 1u8];
        let (_, decoded): (usize, u16) = de::deserialize(&input).unwrap();

        assert_eq!(1, decoded);
    }

    #[test]
    fn deserialize_esv() {
        let input = [0x62u8];
        let (_, decoded): (usize, ServiceCode) = de::deserialize(&input).unwrap();

        assert_eq!(ServiceCode::Get, decoded);
    }

    #[test]
    fn deserialize_eoj() {
        let input = [0xefu8, 0xffu8, 0x01u8];
        let (_, decoded): (usize, EchonetObject) = de::deserialize(&input).unwrap();

        assert_eq!(EchonetObject([0xefu8, 0xffu8, 0x01u8]), decoded);
    }

    #[test]
    fn deserialize_edt() {
        let input: [u8; 2] = [0x01, 0x01];
        let (_, decoded): (usize, Edt) = de::deserialize(&input).unwrap();

        let expect = Edt(vec![0x01u8]);
        assert_eq!(expect, decoded);
    }

    #[test]
    fn deserialize_empty_edt() {
        let input: [u8; 1] = [0u8];
        let (_, decoded): (usize, Edt) = de::deserialize(&input).unwrap();

        let expect = Edt(vec![]);
        assert_eq!(expect, decoded);
    }

    #[test]
    fn deserialize_props() {
        let input: Vec<u8> = vec![1, 0x80, 0x01, 0x02];
        let (_, decoded): (usize, Properties) = de::deserialize(&input).unwrap();
        
        let expect = Properties(vec![Property{ epc: 0x80, edt: Edt(vec![0x02])}]);
        assert_eq!(expect, decoded);
    }
}
