#![allow(unused)]
use crate::table_management::Column;
use serde::ser::{Serialize};
use serde::ser;
pub struct Serializer  {
    output: String,
}

#[derive(Debug)]
pub struct Error;

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display
    {
        unimplemented!()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occured")
    }
}
impl ser::StdError for Error {}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error; 
    type SerializeSeq = Self;
    type SerializeStruct = Self;
    type SerializeMap = Self;
    type SerializeStructVariant = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;

    fn serialize_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStruct, Self::Error> 
    {
        Ok(self)    
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> 
    {
        Ok(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error>
    {
        Ok(self)
    }

    fn serialize_struct_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> 
    {
        Ok(self)    
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> 
    {
        Ok(self)    
    }

    fn serialize_tuple_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> 
    {
        Ok(self)    
    }

    fn serialize_tuple_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> 
    {
        Ok(self)    
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> 
    {
        self.output += if v {"true"} else {"false"};
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> 
    {
        self.serialize_i64(i64::from(v))    
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> 
    {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> 
    {
        self.serialize_i64(i64::from(v))    
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> 
    {
        self.output += &v.to_string();
        Ok(())    
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> 
    {
        self.serialize_u64(u64::from(v))    
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> 
    {
        self.serialize_u64(u64::from(v))    
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> 
    {
        self.serialize_u64(u64::from(v))    
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> 
    {
        self.output += &v.to_string();

        Ok(())    
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> 
    {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> 
    {
        self.output += &v.to_string();
        Ok(())    
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> 
    {
        self.serialize_str(&v.to_string())    
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> 
    {
        self.output += "\"";
        self.output += v;
        self.output += "\"";

        Ok(())    
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> 
    {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> 
    {
        self.serialize_unit()    
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize 
    {
        value.serialize(self)    
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.output +=  "null";
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> 
    {
        self.serialize_unit()    
    }

    fn serialize_unit_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> 
    {
        self.serialize_str(variant)
    }

    fn serialize_newtype_variant<T: ?Sized>(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize 
    {
        variant.serialize(&mut *self)?;
        self.output += ": ";
        variant.serialize(&mut *self)?;

        Ok(())    
    }

    fn serialize_newtype_struct<T: ?Sized>(
            self,
            name: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize
    {
        value.serialize(self)    
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T: ?Sized>(
            &mut self,
            key: &'static str,
            value: &T,
        ) -> Result<(), Self::Error>
        where
            T: Serialize
    {
        key.serialize(&mut **self)?;
        self.output += ":";
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
        where
            T: Serialize
    {
        key.serialize(&mut **self)    
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize 
    {
        self.output += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize 
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }

        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output += "]";
        Ok(())
    }
}


impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
            &mut self,
            key: &'static str,
            value: &T,
        ) -> Result<(), Self::Error>
        where
            T: Serialize 
    {
        key.serialize(&mut **self)?;
        self.output += ":";
        value.serialize(&mut **self)    
    }

    fn end(self) -> Result<Self::Ok, Self::Error> 
    {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;


    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize 
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }

        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> 
    {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize 
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }

        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> 
    {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize 
    {
        if self.output.ends_with('[') {
            self.output += ",";
        }

        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> 
    {
        Ok(())    
    }
}