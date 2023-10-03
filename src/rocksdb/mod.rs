use rocksdb::{DB, Options};
use super::table_management::*;

/*
 * I created this module in order to use the rocksdb file format so I can enable both the use of
 * this as an embedded database, and the use of this in a server mode.
 */
