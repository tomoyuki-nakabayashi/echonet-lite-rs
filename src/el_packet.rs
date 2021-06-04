#![allow(dead_code)]
use alloc::boxed::Box;
use num_derive::FromPrimitive;

// TODO: serialize / deserialize
#[derive(Debug)]
struct ElPacket {
    ehd1: u8,
    ehd2: u8,
    transaction_id: u16,
    seoj: EchonetObject,
    deoj: EchonetObject,
    esv: ServiceCode,
    opc: u8,
    props: Box<[u8]>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive)]
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

#[derive(Debug, Default)]
struct EchonetObject([u8; 3]);

#[derive(Debug)]
struct ElPacketBuilder {
    transaction_id: u16, // builder 作るときに渡しても良いかも
    seoj: EchonetObject,
    deoj: EchonetObject,
    esv: Option<ServiceCode>,
    opc: u8,
    props: Box<[u8]>,
}

impl ElPacketBuilder {
    pub fn new() -> Self {
        Self {
            transaction_id: 0,
            seoj: Default::default(),
            deoj: Default::default(),
            esv: None,
            opc: 0,
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

    pub fn props(mut self, props: Box<[u8]>) -> Self {
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
            opc: self.opc,
            props: self.props,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn el() {
        let packet = ElPacketBuilder::new()
            .transaction_id(1)
            .esv(ServiceCode::Get)
            .build();
        println!("{:?}", packet);
        assert!(true);
    }
}
