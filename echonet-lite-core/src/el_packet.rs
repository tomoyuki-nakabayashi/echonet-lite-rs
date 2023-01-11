#![allow(dead_code)]
use crate::lib::fmt;
use crate::lib::ops::{Deref, DerefMut};
use crate::lib::vec::Vec;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub use crate::object::EchonetObject;
use crate::{de, ser, Error};

/// An ECHONET Lite packet representation.
///
/// ECHONET Lite SPEC shows an ECHONET Lite packet contains
/// - EHD1: ECHONET Lite message header1 (1-byte)
/// - EHD2: ECHONET Lite message header2 (1-byte)
/// - SEOJ: Source ECHONET Lite object specification (3-byte)
/// - DEOJ: Destination ECHONET Lite object specification (3-byte)
/// - ESV: ECHONET Lite service
/// - OPC: Number of processing properties
/// - (EPC, PDC, EDT) * OPC
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElPacket {
    // ECHONTE Lite header must be 0x1081
    ehd1: u8,
    ehd2: u8,
    // unique ID for each packet
    transaction_id: u16,
    // source ECHONET object
    pub seoj: EchonetObject,
    // destination ECHONET object
    pub deoj: EchonetObject,
    // ECHONET service code
    pub esv: ServiceCode,
    // properties contain opc (Operation count), epc (ECHONET property code), and
    // edt (ECHONET data).
    pub props: Properties,
}

impl ElPacket {
    /// Serializes an ECHONET Lite packet into byte array.
    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        ser::serialize(&self)
    }

    /// Deserializes an ECHONET Lite packet from byte array.
    pub fn from_bytes(bytes: &[u8]) -> Result<(usize, ElPacket), Error> {
        de::deserialize(bytes)
    }

    /// Returns whether `self` is a response for the `req`.
    #[allow(clippy::suspicious_operation_groupings)]
    pub fn is_response_for(&self, req: &ElPacket) -> bool {
        self.transaction_id == req.transaction_id && self.seoj == req.deoj
    }

    /// Creates a new response for itself.
    ///
    /// `esv` must be one of response service code.
    /// `props` contains all response properties.
    ///
    /// The created response packet has the same transaction ID as original packet.
    /// The source and the destination are reversed.
    pub fn create_response(&self, esv: ServiceCode, props: Properties) -> ElPacket {
        ElPacketBuilder::new()
            .transaction_id(self.transaction_id)
            .seoj(self.deoj)
            .deoj(self.seoj)
            .esv(esv)
            .props(props)
            .build()
    }
}

impl fmt::Display for ElPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "EHD: {:02X}{:02X}", self.ehd1, self.ehd2)?;
        writeln!(f, "TID: {}", self.transaction_id)?;
        writeln!(f, "SEOJ: {}", self.seoj)?;
        writeln!(f, "DEOJ: {}", self.deoj)?;
        writeln!(f, "{}", self.esv)?;
        write!(f, "{}", self.props)
    }
}

/// Reperesents ECHONET LiteService (ESV).
/// The service code specifies an operation for properties stipulated by the EPC.
#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ServiceCode {
    /// A response for SetI; Property value write "response is not possible".
    SetISNA = 0x50,
    /// A response for SetC; Property value write "response is not possible".
    SetCSNA = 0x51,
    /// A response for Get; Property value read "response is not possible".
    GetSNA = 0x52,
    /// A response for InfReq; Property value notification "response is not possible".
    InfSNA = 0x53,
    /// A response for SetGet; Property value write & read request "response not possible".
    SetGetSNA = 0x5E,
    /// Property value write request (no response required). Broadcast possible.
    SetI = 0x60,
    /// Property value write request (response required). Broadcast possible.
    SetC = 0x61,
    /// Property value read request. Broadcast possible.
    Get = 0x62,
    /// Property value notification request. Broadcast possible.
    InfReq = 0x63,
    /// Property value read & write request. Broadcast possible.
    SetGet = 0x6E,
    /// An individual response for SetC; Property value write response.
    SetRes = 0x71,
    /// An individual response for Get; Property value read response.
    GetRes = 0x72,
    /// Property value notification. Both individual notification and broadcast notification.
    Inf = 0x73,
    /// Individual property value notification (response required).
    InfC = 0x74,
    /// An individual response for InfC; Property value notification response.
    InfCRes = 0x7A,
    /// An individual response for SetGet; Property value write & read response.
    SetGetRes = 0x7E,
}

impl fmt::Display for ServiceCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ESV: {:02X} ", *self as u8)?;
        let esv = match self {
            ServiceCode::SetISNA => "SetISNA",
            ServiceCode::SetCSNA => "SetCSNA",
            ServiceCode::GetSNA => "GetSNA",
            ServiceCode::InfSNA => "InfSNA",
            ServiceCode::SetGetSNA => "SetGetSNA",
            ServiceCode::SetI => "SetI",
            ServiceCode::SetC => "SetC",
            ServiceCode::Get => "Get",
            ServiceCode::InfReq => "InfReq",
            ServiceCode::SetGet => "SetGet",
            ServiceCode::SetRes => "SetRes",
            ServiceCode::GetRes => "GetRes",
            ServiceCode::Inf => "Inf",
            ServiceCode::InfC => "InfC",
            ServiceCode::InfCRes => "InfCRes",
            ServiceCode::SetGetRes => "SetGetRes",
        };
        write!(f, "({})", esv)
    }
}

/// An ECHONET property array consists of `OPC, EPC1, PDC1, EDT1 ... EPCn, PDCn, EDTn`.
#[derive(PartialEq, Default, Serialize, Deserialize)]
pub struct Properties(Vec<Property>);
impl Properties {
    pub fn num(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<&Property> {
        if index < self.num() {
            Some(&self.0[index])
        } else {
            None
        }
    }

    pub fn iter(&self) -> core::slice::Iter<Property> {
        self.0.iter()
    }
}

impl core::iter::IntoIterator for Properties {
    type Item = Property;
    type IntoIter = crate::lib::vec::IntoIter<Property>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for Properties {
    type Target = Vec<Property>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Properties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// TODO: from Iter<Property>?
impl From<Vec<Property>> for Properties {
    fn from(props: Vec<Property>) -> Self {
        Self(props)
    }
}

impl fmt::Debug for Properties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "OPC: {}", self.0.len())?;
        for prop in self.0.iter() {
            write!(f, "{:?}", prop)?;
        }
        Ok(())
    }
}

impl fmt::Display for Properties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for prop in self.0.iter() {
            writeln!(f, "{}", prop)?;
        }
        Ok(())
    }
}

impl Clone for Properties {
    fn clone(&self) -> Self {
        Self(self.0.to_vec())
    }
}

/// A ECHONET property putting EPC, OPC, and EDT together.
#[derive(Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Property {
    pub epc: u8,
    pub edt: Edt,
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02X}: ", self.epc)?;
        write!(f, "{}", self.edt)?;
        Ok(())
    }
}

impl fmt::Debug for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "EPC: {:02X}", self.epc)?;
        writeln!(f, "{:?}", self.edt)?;
        Ok(())
    }
}

/// ECHONET property value data.
///
/// EDT consists of data for the relevant ECHONET property (EPC) and
/// control by an ESV (ServiceCode).
#[derive(PartialEq, Default, Serialize, Deserialize)]
pub struct Edt(Vec<u8>);

impl Edt {
    pub fn new(value: Vec<u8>) -> Edt {
        Edt(value)
    }
}

impl Deref for Edt {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for Edt {
    fn clone(&self) -> Self {
        Self(self.0.to_vec())
    }
}

impl fmt::Display for Edt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0.iter() {
            write!(f, "{:02X} ", byte)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Edt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "PDC: {}", self.0.len())?;
        write!(f, "EDT: ")?;
        for byte in self.0.iter() {
            write!(f, "{:02X} ", byte)?;
        }
        Ok(())
    }
}

/// Builds a ECHONET Lite packet.
///
/// # Examples
///
/// ```
/// use echonet_lite as el;
/// use el::prelude::*;
///
/// let packet = el::ElPacketBuilder::new()
///     .transaction_id(1)
///     .seoj([0x05u8, 0xFFu8, 0x01u8])
///     .deoj([0x0Eu8, 0xF0u8, 0x01u8])
///     .esv(el::ServiceCode::Get)
///     .props(el::props!([0x80, []]))
///     .build();
/// ```
#[derive(Debug)]
pub struct ElPacketBuilder {
    transaction_id: u16,
    seoj: EchonetObject,
    deoj: EchonetObject,
    esv: Option<ServiceCode>,
    props: Properties,
}

impl Default for ElPacketBuilder {
    fn default() -> Self {
        Self::new()
    }
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

/// Create a Property object from a pair of EPC (u8) and EDT ([u8; _]).
///
/// # Examples
///
/// ```
/// use echonet_lite::{prelude::*, prop};
/// let prop = prop!(0x80, [0x30]);
/// ```
#[macro_export]
macro_rules! prop {
    ( $epc:expr, [ $( $edt:expr ),* ] ) => {
        {
            let mut bytes: Vec<u8> = Vec::new();
            $(
                bytes.push($edt);
            )*
            Property{ epc: $epc, edt: Edt::new(bytes) }
        }
    };
}

/// Create a Properties object from an array of EPC (u8) and EDT ([u8; _]) pairs.
///
/// # Examples
///
/// ```
/// use echonet_lite::{prelude::*, props};
/// let props = props!([0x80, [0x30]], [0x81, [0x08]]);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! props {
    ( $( [ $epc:expr, [ $( $edt:expr ),* ] ] ),* ) => {
        {
            let mut props: Vec<Property> = Vec::new();
            $(
                props.push( $crate::prop!($epc, [ $( $edt ),* ] ) );
            )*
            Properties::from(props)
        }
    };
}

/// Create a Properties object for Get request obtaining one or more property values.
///
/// # Examples
///
/// ```
/// use echonet_lite::{prelude::*, bulk_read};
/// let props = bulk_read!(0x80, 0x81, 0x82);
/// ```
#[macro_export]
macro_rules! bulk_read {
    ( $( $epc:expr ),* ) => {
        {
            let mut props: Vec<Property> = Vec::new();
            $(
                props.push( $crate::prop!($epc, [] ) );
            )*
            Properties::from(props)
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::de;

    #[test]
    fn serialize() {
        let props = props!([0x80, [0x02]]);
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
        let input: Vec<u8> = vec![
            0x10, 0x81, 0, 1, 0xef, 0xff, 0x01, 0x03, 0x08, 0x01, 0x62, 1, 0x80, 0x01, 0x02,
        ];
        let (consumed, decoded): (usize, ElPacket) = ElPacket::from_bytes(&input).unwrap();

        let prop = Property {
            epc: 0x80,
            edt: Edt(vec![0x02]),
        };
        let expect = ElPacketBuilder::new()
            .transaction_id(1)
            .esv(ServiceCode::Get)
            .seoj([0xef, 0xff, 0x01])
            .deoj([0x03, 0x08, 0x01])
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

        assert_eq!(EchonetObject::from([0xefu8, 0xffu8, 0x01u8]), decoded);
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

        let expect = Properties(vec![Property {
            epc: 0x80,
            edt: Edt(vec![0x02]),
        }]);
        assert_eq!(expect, decoded);
    }

    #[test]
    fn iter_properties() {
        let props = props!([0x80, [0x02]]);
        let expect = prop!(0x80, [0x02]);
        assert_eq!(1usize, props.num());
        for prop in props.iter() {
            assert_eq!(expect, *prop);
        }
    }
}
