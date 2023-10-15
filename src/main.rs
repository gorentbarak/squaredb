pub mod table_management;
pub mod rocksdb;
fn main() {
    dbg!(table_management::create_table("test"));
}
