#![allow(warnings)] // For now.
use super::table_management::*;
use bincode;
use rocksdb::{Options, DB};
use std::io;
use tokio::fs;
/*
 * I created this module in order to use the rocksdb file format so I can enable both the use of
 * this as an embedded database, and the use of this in a server mode as a storage layer.
 */

#[cfg(test)]
mod tests {
    use crate::{
        storage::{file_to_table, table_to_file},
        table_management::{create_table, Column, ColumnType, Row},
    };

    #[test]
    fn test_storage_layer() {
        let table1 = create_table("test")
            .insert_row(Row::new(vec![ColumnType::Int(Column::new("test").set_content(10))]));
        table_to_file("/tmp/test", table1.clone()).unwrap();
        let table2 = file_to_table("/tmp/test").unwrap();
        assert_eq!(&table1, &table2);
    }
}

pub fn table_to_file(path: &str, table: Table) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(path);
    let db = DB::open_default(path)?;
    let table_bytes = bincode::serialize(&table)?;
    db.put(b"table", &table_bytes)?;
    Ok(())
}

pub fn file_to_table(path: &str) -> Result<Table, Box<dyn std::error::Error>> {
    let db = DB::open_default(path)?;
    let table_bytes = db.get(b"table")?.ok_or("Table not found")?;
    let table: Table = bincode::deserialize(&table_bytes)?;
    Ok(table)
}
