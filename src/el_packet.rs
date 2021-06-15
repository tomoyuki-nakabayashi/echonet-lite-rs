#![allow(dead_code)]
use alloc::vec::Vec;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize, Serializer, ser::SerializeTuple};
use serde_repr::{Serialize_repr, Deserialize_repr};

// TODO: deserialize
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ElPacket {
    ehd1: u8,
    ehd2: u8,
    transaction_id: u16,
    seoj: EchonetObject,
    deoj: EchonetObject,
    esv: ServiceCode,
    props: Properties,
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

#[derive(Debug, PartialEq, Default, Deserialize)]
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

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
struct Property {
    epc: u8,
    edt: Edt,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::de::{Deserializer as ElDeserializer, SliceReader};

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

    #[test]
    fn deserialize() {
        let input: Vec<u8> = vec![0x10, 0x81, 0, 1, 0xef, 0xff, 0x01, 0x03, 0x08, 0x01, 0x62, 1, 0x80, 0x01, 0x02];
        let mut deserializer = ElDeserializer::new(SliceReader::new(&input));
        let decoded: ElPacket = serde::Deserialize::deserialize(&mut deserializer).unwrap();

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

        assert_eq!(expect, decoded);
    }

    #[test]
    fn deserialize_tid() {
        let input = [0u8, 1u8];
        let mut deserializer = ElDeserializer::new(SliceReader::new(&input));
        let decoded: u16 = serde::Deserialize::deserialize(&mut deserializer).unwrap();

        assert_eq!(1, decoded);
    }

    #[test]
    fn deserialize_esv() {
        let input = [0x62u8];
        let mut deserializer = ElDeserializer::new(SliceReader::new(&input));
        let decoded: ServiceCode = serde::Deserialize::deserialize(&mut deserializer).unwrap();

        assert_eq!(ServiceCode::Get, decoded);
    }

    #[test]
    fn deserialize_eoj() {
        let input = [0xefu8, 0xffu8, 0x01u8];
        let mut deserializer = ElDeserializer::new(SliceReader::new(&input));
        let decoded: EchonetObject = serde::Deserialize::deserialize(&mut deserializer).unwrap();

        assert_eq!(EchonetObject([0xefu8, 0xffu8, 0x01u8]), decoded);
    }

    #[test]
    fn deserialize_edt() {
        let input: [u8; 2] = [0x01, 0x01];
        let mut deserializer = ElDeserializer::new(SliceReader::new(&input));
        let decoded: Edt = serde::Deserialize::deserialize(&mut deserializer).unwrap();

        let expect = Edt(vec![0x01u8]);
        assert_eq!(expect, decoded);
    }

    #[test]
    fn deserialize_empty_edt() {
        let input: [u8; 1] = [0u8];
        let mut deserializer = ElDeserializer::new(SliceReader::new(&input));
        let decoded: Edt = serde::Deserialize::deserialize(&mut deserializer).unwrap();

        let expect = Edt(vec![]);
        assert_eq!(expect, decoded);
    }

    #[test]
    fn deserialize_props() {
        let input: Vec<u8> = vec![1, 0x80, 0x01, 0x02];
        let mut deserializer = ElDeserializer::new(SliceReader::new(&input));
        let decoded: Properties = serde::Deserialize::deserialize(&mut deserializer).unwrap();

        let expect = Properties(vec![Property{ epc: 0x80, edt: Edt(vec![0x02])}]);
        assert_eq!(expect, decoded);
    }
}
