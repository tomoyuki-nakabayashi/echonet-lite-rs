use crate::error::{Error, ErrorKind, Result};
use alloc::{boxed::Box, vec::Vec};
use bare_io as io;
use read::ReadBytesExt;
use serde;
use serde::de::Error as DeError;
use serde::de::IntoDeserializer;
use serde::de::Visitor;

mod read;

pub fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<(usize, T)>
where
    T: serde::de::Deserialize<'a>,
{
    let length = bytes.len();
    let mut deserializer = Deserializer::new(SliceReader::new(bytes));
    let result = serde::Deserialize::deserialize(&mut deserializer);

    match result {
        Ok(packet) => {
            let consumed = length - deserializer.reader.slice.len();
            Ok((consumed, packet))
        }
        Err(err) => Err(err),
    }
}

pub trait EchonetLiteRead<'storage>: io::Read {
    fn get_byte_buffer(&mut self, length: usize) -> Result<Vec<u8>>;

    fn forward_read_bytes<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'storage>;
}

pub struct SliceReader<'storage> {
    slice: &'storage [u8],
}

impl<'storage> SliceReader<'storage> {
    pub fn new(bytes: &'storage [u8]) -> SliceReader<'storage> {
        SliceReader { slice: bytes }
    }

    #[inline(always)]
    fn unexpected_eof() -> Box<ErrorKind> {
        return Box::new(ErrorKind::Io(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "",
        )));
    }
}

impl<'storage> io::Read for SliceReader<'storage> {
    #[inline(always)]
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        (&mut self.slice).read(out)
    }
    #[inline(always)]
    fn read_exact(&mut self, out: &mut [u8]) -> io::Result<()> {
        (&mut self.slice).read_exact(out)
    }
}

impl<'storage> EchonetLiteRead<'storage> for SliceReader<'storage> {
    #[inline(always)]
    fn get_byte_buffer(&mut self, length: usize) -> Result<Vec<u8>> {
        if length > self.slice.len() {
            return Err(SliceReader::unexpected_eof());
        }

        let r = &self.slice[..length];
        self.slice = &self.slice[length..];
        Ok(r.to_vec())
    }

    #[inline(always)]
    fn forward_read_bytes<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'storage>,
    {
        if length > self.slice.len() {
            return Err(SliceReader::unexpected_eof());
        }

        let r = visitor.visit_borrowed_bytes(&self.slice[..length]);
        self.slice = &self.slice[length..];
        r
    }
}

pub(crate) struct Deserializer<R> {
    reader: R,
}

impl<'de, R: EchonetLiteRead<'de>> Deserializer<R> {
    pub fn new(r: R) -> Deserializer<R> {
        Deserializer { reader: r }
    }

    fn read_vec(&mut self) -> Result<Vec<u8>> {
        let len: u8 = serde::Deserialize::deserialize(&mut *self)?;
        self.reader.get_byte_buffer(len as usize)
    }
}

macro_rules! impl_nums {
    ($ty:ty, $dser_method:ident, $visitor_method:ident, $reader_method:ident) => {
        #[inline]
        fn $dser_method<V>(self, visitor: V) -> Result<V::Value>
        where
            V: serde::de::Visitor<'de>,
        {
            let value = self.reader.$reader_method()?;
            visitor.$visitor_method(value)
        }
    };
}

impl<'de, 'a, R> serde::Deserializer<'de> for &'a mut Deserializer<R>
where
    R: EchonetLiteRead<'de>,
{
    type Error = crate::Error;

    #[inline]
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Box::new(ErrorKind::DeserializeAnyNotSupported))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value: u8 = serde::Deserialize::deserialize(self)?;
        match value {
            1 => visitor.visit_bool(true),
            0 => visitor.visit_bool(false),
            value => Err(ErrorKind::InvalidBoolEncoding(value).into()),
        }
    }

    #[inline]
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u8(self.reader.read_u8()?)
    }

    #[inline]
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i8(self.reader.read_i8()?)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let len: u8 = serde::Deserialize::deserialize(&mut *self)?;
        self.reader.forward_read_bytes(len as usize, visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_byte_buf(self.read_vec()?)
    }

    fn deserialize_enum<V>(
        self,
        _enum: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        impl<'de, 'a, R: 'a> serde::de::EnumAccess<'de> for &'a mut Deserializer<R>
        where
            R: EchonetLiteRead<'de>,
        {
            type Error = Error;
            type Variant = Self;

            fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
            where
                V: serde::de::DeserializeSeed<'de>,
            {
                let idx: u32 = serde::de::Deserialize::deserialize(&mut *self)?;
                let val: Result<_> = seed.deserialize(idx.into_deserializer());
                Ok((val?, self))
            }
        }

        visitor.visit_enum(self)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        struct Access<'a, R: io::Read + 'a> {
            deserializer: &'a mut Deserializer<R>,
            len: usize,
        }

        impl<'de, 'a, 'b: 'a, R: EchonetLiteRead<'de> + 'b> serde::de::SeqAccess<'de> for Access<'a, R> {
            type Error = Error;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
            where
                T: serde::de::DeserializeSeed<'de>,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let value =
                        serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.len)
            }
        }

        visitor.visit_seq(Access {
            deserializer: self,
            len: len,
        })
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let len: u8 = serde::Deserialize::deserialize(&mut *self)?;

        self.deserialize_tuple(len as usize, visitor)
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let message = "serde-echonet-lite does not support Deserializer::deserialize_identifier";
        Err(Error::custom(message))
    }

    fn deserialize_newtype_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let message = "serde-echonet-lite does not support Deserializer::deserialize_ignored_any";
        Err(Error::custom(message))
    }

    fn is_human_readable(&self) -> bool {
        false
    }

    impl_nums!(u16, deserialize_u16, visit_u16, read_u16);
    impl_nums!(u32, deserialize_u32, visit_u32, read_u32);
    impl_nums!(i16, deserialize_i16, visit_i16, read_i16);
    impl_nums!(i32, deserialize_i32, visit_i32, read_i32);

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
}

impl<'de, 'a, R> serde::de::VariantAccess<'de> for &'a mut Deserializer<R>
where
    R: EchonetLiteRead<'de>,
{
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        serde::de::DeserializeSeed::deserialize(seed, self)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_tuple(self, len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_tuple(self, fields.len(), visitor)
    }
}
