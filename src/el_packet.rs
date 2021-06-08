#![allow(dead_code)]
use alloc::boxed::Box;
use num_derive::FromPrimitive;
use serde::Serialize;
use serde_repr::Serialize_repr;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    props: Option<Box<[Property]>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive, Serialize_repr)]
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

#[derive(Debug, Default, Serialize)]
struct EchonetObject([u8; 3]);

#[derive(Debug, Default, Serialize)]
struct Property {
    epc: u8,
    pdc: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    edt: Option<Box<[u8]>>,
}

#[derive(Debug)]
struct ElPacketBuilder {
    transaction_id: u16, // builder 作るときに渡しても良いかも
    seoj: EchonetObject,
    deoj: EchonetObject,
    esv: Option<ServiceCode>,
    opc: u8,
    props: Option<Box<[Property]>>,
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

    pub fn props(mut self, props: Box<[Property]>) -> Self {
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
    use bincode::Options;
    use super::*;

    #[test]
    fn serialize() {
        let packet = ElPacketBuilder::new()
            .transaction_id(1)
            .esv(ServiceCode::Get)
            .seoj(EchonetObject([0xef, 0xff, 0x01]))
            .deoj(EchonetObject([0x03, 0x08, 0x01]))
            .build();
        let config = bincode::DefaultOptions::new().with_big_endian().with_fixint_encoding();
        let encoded: Vec<u8> = config.serialize(&packet).unwrap();
        assert_eq!(vec![0x10, 0x81, 0, 1, 0xef, 0xff, 0x01, 0x03, 0x08, 0x01, 0x62, 0], encoded);
    }
}
