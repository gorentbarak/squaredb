#![allow(unused)]
use crate::table_management::Column;

use serde::ser::{Serialize, Serializer, SerializeStruct};
impl<T> Serialize for Column<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {
        let mut sv = serializer.serialize_struct("Column", 2)?;
        sv.end()
    }
}