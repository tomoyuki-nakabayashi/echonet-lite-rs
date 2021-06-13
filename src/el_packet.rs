#![allow(dead_code)]
use core::fmt::{self, Formatter};

use alloc::vec::Vec;
use num_derive::FromPrimitive;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor, ser::SerializeTuple};
use serde_repr::{Serialize_repr, Deserialize_repr};

// TODO: deserialize
#[derive(Debug, Serialize)]
struct ElPacket {
    ehd1: u8,
    ehd2: u8,
    transaction_id: u16,
    seoj: EchonetObject,
    deoj: EchonetObject,
    esv: ServiceCode,
    opc: u8,
    #[serde(serialize_with = "untag_option")]
    props: Option<Properties>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum ServiceCode {
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
struct EchonetObject([u8; 3]);

#[derive(Debug, Default)]
struct Properties(Vec<Property>);
impl Serialize for Properties {
    // In bincode serialization, a slice serializes into a lengh followed by a byte array.
    // Because we just need a byte array, implement custom serialization here.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(self.0.len())?;
        for e in self.0.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

#[derive(Debug, Default, Serialize)]
struct Property {
    epc: u8,
    pdc: u8,
    #[serde(serialize_with = "untag_option")]
    edt: Option<Edt>,
}

#[derive(Debug, PartialEq, Default)]
struct Edt(Vec<u8>);
impl Serialize for Edt {
    // In bincode serialization, a slice serializes into a lengh followed by a byte array.
    // Because we just need a byte array, implement custom serialization here.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(self.0.len())?;
        for e in self.0.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

struct EdtVisitor;
impl<'de> Visitor<'de> for EdtVisitor {
    type Value = Edt;

    fn expecting(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        formatter.write_str("never failed")
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error
    {
        let v = value.iter().cloned().collect();
        Ok(Edt(v))
    }
}

impl<'de> Deserialize<'de> for Edt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_bytes( EdtVisitor)
    }
}

// untag Option to avoid bincode serializes Some(T) into [1, ...].
fn untag_option<S, T>(f: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    match f {
        Some(value) => value.serialize(serializer),
        None => serializer.serialize_unit(),
    }
}

#[derive(Debug)]
struct ElPacketBuilder {
    transaction_id: u16, // builder 作るときに渡しても良いかも
    seoj: EchonetObject,
    deoj: EchonetObject,
    esv: Option<ServiceCode>,
    opc: u8,
    props: Option<Properties>,
}

impl ElPacketBuilder {
    pub fn new() -> Self {
        Self {
            transaction_id: 0,
            seoj: Default::default(),
            deoj: Default::default(),
            esv: None,
            opc: 0,
            props: None,
        }
    }

    pub fn transaction_id(mut self, tid: u16) -> Self {
        self.transaction_id = tid;
        self
    }

    pub fn seoj(mut self, seoj: EchonetObject) -> Self {
        self.seoj = seoj;
        self
    }

    pub fn deoj(mut self, deoj: EchonetObject) -> Self {
        self.deoj = deoj;
        self
    }

    pub fn esv(mut self, esv: ServiceCode) -> Self {
        self.esv = Some(esv);
        self
    }

    pub fn opc(mut self, opc: u8) -> Self {
        self.opc = opc;
        self
    }

    pub fn props(mut self, props: Properties) -> Self {
        self.props = Some(props);
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
            opc: self.opc,
            props: self.props,
        }
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;

    use super::*;
    use crate::de_echonet_lite::{Deserializer as ElDeserializer, SliceReader};
    use serde::{Deserializer, Serialize, Serializer, ser::SerializeTuple};

    // #[test]
    // fn serialize() {
    //     let prop = Property {
    //         epc: 0x80,
    //         pdc: 0x01,
    //         edt: Some(Edt(vec![0x02u8])),
    //     };
    //     let packet = ElPacketBuilder::new()
    //         .transaction_id(1)
    //         .esv(ServiceCode::Get)
    //         .seoj(EchonetObject([0xef, 0xff, 0x01]))
    //         .deoj(EchonetObject([0x03, 0x08, 0x01]))
    //         .opc(1)
    //         .props(Properties(vec![prop]))
    //         .build();
    //     let config = bincode::DefaultOptions::new()
    //         .with_big_endian()
    //         .with_fixint_encoding();
    //     let encoded: Vec<u8> = config.serialize(&packet).unwrap();
    //     assert_eq!(
    //         vec![0x10, 0x81, 0, 1, 0xef, 0xff, 0x01, 0x03, 0x08, 0x01, 0x62, 1, 0x80, 0x01, 0x02],
    //         encoded
    //     );
    // }

    // #[test]
    // fn deserialize() {
    //     let input: Vec<u8> = vec![0x10, 0x81, 0, 1, 0xef, 0xff, 0x01, 0x03, 0x08, 0x01, 0x62, 1, 0x80, 0x01, 0x02];
    //     let config = bincode::DefaultOptions::new()
    //         .with_big_endian()
    //         .with_fixint_encoding();
    //     let decoded: Option<ElPacket> = bincode::deserialize(&input).unwrap();

    //     let prop = Property {
    //         epc: 0x80,
    //         pdc: 0x01,
    //         edt: Some(Edt(Box::new([0x02u8]))),
    //     };
    //     let expect: Option<ElPacket> = Some(
    //         ElPacketBuilder::new()
    //             .transaction_id(1)
    //             .esv(ServiceCode::Get)
    //             .seoj(EchonetObject([0xef, 0xff, 0x01]))
    //             .deoj(EchonetObject([0x03, 0x08, 0x01]))
    //             .opc(1)
    //             .props(Properties(Box::new([prop])))
    //             .build()
    //     );

    //     assert_eq!(expect, decoded);
    // }

    // #[test]
    // fn deserialize_tid() {
    //     let input = [0u8, 1u8];
    //     let config = bincode::DefaultOptions::new()
    //         .with_big_endian()
    //         .with_fixint_encoding();
    //     let decoded: u16 = config.deserialize(&input).unwrap();

    //     assert_eq!(1, decoded);
    // }

    #[test]
    fn deserialize_esv() {
        let input = [0x62u8];
        let mut deserializer = ElDeserializer::new(SliceReader::new(&input));
        let decoded:u8 = serde::Deserialize::deserialize(&mut deserializer).unwrap();

        assert_eq!(0x62, decoded);
        // let decoded: ServiceCode = deserializer.deserialize_u8().unwrap();

        // assert_eq!(ServiceCode::Get, decoded);
    }

    // #[test]
    // fn deserialize_eoj() {
    //     let input = [0xefu8, 0xffu8, 0x01u8];
    //     let decoded: EchonetObject = bincode::deserialize(&input).unwrap();

    //     assert_eq!(EchonetObject([0xefu8, 0xffu8, 0x01u8]), decoded);
    // }

    #[test]
    fn deserialize_edt() {
        let input: [u8; 2] = [0x01, 0x01];
        let mut deserializer = ElDeserializer::new(SliceReader::new(&input));
        let decoded:Edt = serde::Deserialize::deserialize(&mut deserializer).unwrap();

        let expect = Edt(vec![0x01u8]);
        assert_eq!(expect, decoded);
    }
}
