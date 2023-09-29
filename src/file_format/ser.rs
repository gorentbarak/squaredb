#![allow(unused)]
use crate::table_management::Column;
use serde::ser::{Serialize, SerializeStruct};
impl<T> Serialize for Column<T> where T: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer
    {
        let mut sv = serializer.serialize_struct("Column", 2)?;
        sv.serialize_field("name", &self.name);
        sv.serialize_field("content", &self.content.as_ref().unwrap());
        sv.end()
    }
}