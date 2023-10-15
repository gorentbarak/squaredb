use crate::table_management::ColumnType;

pub mod storage;
pub mod table_management;
fn main() {
    let table =
        table_management::create_table("TEST").insert_row(table_management::Row::new(vec![
            ColumnType::Int(table_management::Column::new("name")),
        ]));
    dbg!(&table);
    storage::table_to_file("/home/goren/rocksdb/file.db", table);
    dbg!(storage::file_to_table("/home/goren/rocksdb/file.db/"));
}
