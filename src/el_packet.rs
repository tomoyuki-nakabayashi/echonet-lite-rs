
// TODO: serialize / deserialize
#[derive(Debug, Default)]
struct ElPacket {
    ehd1: u8,
    ehd2: u8,
    transaction_id: u16,
    seoj: [u8;3],
    deoj: [u8;3],
    esv: u8,
    opc: u8,
    props: Box<[u8]>,
}

// TODO: builder to make an object easily
#[derive(Default)]
struct ElPacketBuilder {
    transaction_id: u16, // builder 作るときに渡しても良いかも
    seoj: [u8;3],
    deoj: [u8;3],
    esv: u8,
    opc: u8,
    props: Box<[u8]>,
}

impl ElPacketBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn transaction_id(mut self, tid: u16) -> Self {
        self.transaction_id = tid;
        self
    }

    pub fn seoj(mut self, seoj: [u8;3]) -> Self {
        self.seoj = seoj;
        self
    }

    pub fn deoj(mut self, deoj: [u8;3]) -> Self {
        self.deoj = deoj;
        self
    }

    pub fn esv(mut self, esv: u8) -> Self {
        self.esv = esv;
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
            esv: self.esv,
            opc: self.opc,
            props: self.props,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{ElPacket, ElPacketBuilder};

    #[test]
    fn el() {
        let packet = ElPacket::default();
        println!("{:?}", packet);

        let packet = ElPacketBuilder::new()
            .transaction_id(1)
            .esv(0x62)
            .build();
        println!("{:?}", packet);
        assert!(true);
    }
}