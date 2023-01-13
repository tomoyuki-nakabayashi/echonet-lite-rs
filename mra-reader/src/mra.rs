use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct ValidRelease {
    from: String,
    to: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct JaEnString {
    pub ja: String,
    pub en: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct AccessRule {
    get: String,
    set: String,
    inf: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct DefRef {
    #[serde(rename = "$ref")]
    def: String,
    coefficient: Option<serde_json::Value>,
    #[serde(rename = "overflowCode")]
    overflow_code: Option<bool>,
    #[serde(rename = "underflowCode")]
    underflow_code: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct EdtVariant {
    edt: String,
    name: String,
    descriptions: JaEnString,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct DefInline {
    #[serde(rename = "type")]
    t: String,
    size: usize,
    #[serde(rename = "enum")]
    enumeration: Vec<EdtVariant>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct DefOneOf {
    #[serde(rename = "oneOf")]
    one_of: Vec<TypeDef>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct DefElement {
    #[serde(rename = "shortName")]
    short_name: String,
    element: TypeDef,
    #[serde(rename = "elementName")]
    name: JaEnString,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct DefObject {
    #[serde(rename = "type")]
    t: String,
    properties: Vec<DefElement>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct DefBitMapPosition {
    index: usize,
    #[serde(rename = "bitMask")]
    bitmask: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct DefBitMaps {
    name: String,
    descriptions: JaEnString,
    position: DefBitMapPosition,
    value: TypeDef,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct DefBitMapType {
    #[serde(rename = "type")]
    t: String,
    size: usize,
    bitmaps: Vec<DefBitMaps>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct DefArray {
    #[serde(rename = "type")]
    t: String,
    #[serde(rename = "itemSize")]
    item_size: usize,
    #[serde(rename = "maxItems")]
    max_items: usize,
    items: TypeDef,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct NumericValue {
    edt: String,
    #[serde(rename = "numericValue")]
    value: f64,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct DefNumeric {
    #[serde(rename = "type")]
    t: String,
    size: usize,
    #[serde(rename = "enum")]
    enumerations: Vec<NumericValue>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum TypeDef {
    Ref(DefRef),
    Inline(DefInline),
    OneOf(DefOneOf),
    Object(DefObject),
    BitMap(DefBitMapType),
    // need Box to avoid recursive infinite size
    Array(Box<DefArray>),
    Numeric(DefNumeric),
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Property {
    pub epc: String,
    #[serde(rename = "validRelease")]
    valid_release: ValidRelease,
    #[serde(rename = "propertyName")]
    pub name: JaEnString,
    #[serde(rename = "shortName")]
    short_name: String,
    #[serde(rename = "accessRule")]
    access_rule: AccessRule,
    descriptions: JaEnString,
    data: TypeDef,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct MraClass {
    eoj: String,
    #[serde(rename = "validRelease")]
    valid_release: ValidRelease,
    #[serde(rename = "className")]
    class_name: JaEnString,
    #[serde(rename = "shortName")]
    pub short_name: String,
    #[serde(rename = "elProperties")]
    pub properties: Vec<Property>,
}
