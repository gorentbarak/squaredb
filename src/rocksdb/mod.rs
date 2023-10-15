#![allow(warnings)] // For now.

use rocksdb::{DB, Options};
use super::table_management::*;

/*
 * I created this module in order to use the rocksdb file format so I can enable both the use of
 * this as an embedded database, and the use of this in a server mode.
 */

fn table_to_file(path: &str, table: Table) {
    let db = DB::open_default(path).unwrap();
    db.put(b"name", table.name.as_bytes()).unwrap();
    for i in table.rows {
        for j in i.columns {
            db.put(j.unwrap().name.as_bytes(), j.unwrap()).unwrap();
        }
    }
}
