#![allow(warnings)] // For now.
use super::table_management::*;
use bincode;
use rocksdb::{Options, DB};
use std::io;
/*
 * I created this module in order to use the rocksdb file format so I can enable both the use of
 * this as an embedded database, and the use of this in a server mode as a storage layer.
 */

pub fn table_to_file(path: &str, table: Table) -> Result<(), Box<dyn std::error::Error>> {
    let db = DB::open_default(path)?;
    let table_bytes = bincode::serialize(&table)?;
    db.put(b"table", &table_bytes)?;
    Ok(())
}

pub fn file_to_table(path: &str) -> Result<Table, Box<dyn std::error::Error>> {
    let db = DB::open_default(path)?;
    let table_bytes = db.get(b"table")?
        .ok_or("Table not found")?;
    let table: Table = bincode::deserialize(&table_bytes)?;
    Ok(table)
}

