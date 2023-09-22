pub mod table_management;

fn main() {
    println!("{:?}", table_management::create_table("name").set_columns(vec![table_management::ColumnType::Int(table_management::Column::new("id").set_content(vec![1]).insert_content(10))]).insert_column(table_management::ColumnType::Str(table_management::Column::new("name").insert_content("Goren".to_string()))))
}