#![allow(dead_code)] // For now.
#[derive(Debug)]
/* 
 * DISCLAIMER:
 * This is currently only stored in memory.
 * I will later make a file format, and add many other optimizations.
*/
pub enum ColumnType {
    /* Currently public, create better abstractions later.
     * This type is to allow for multiple types of columns (currently the Int type, a 64-bit integer, and the Str type, a string).
     * I'll add more types later.
     */
    Int(Column<i64>),    // Support for the integer type.
    Str(Column<String>), // Support for the string type.
}

#[derive(Debug)]
pub struct Table {
    /* This is the type used to create a table in the database.
     * I'll probably make a better abstraction for this later.
     */
    name:    String,
    columns: Option<Vec<ColumnType>>
}

#[derive(Debug)]
pub struct Column<T> {
    /* This is for a column in our database.
     * No abstractions seem to be currently needed.
     */
    name:    String,
    content: Option<Vec<T>>
}

pub fn create_table(name: &str) -> Table {
    // Create a new table.
    Table {
        name:    name.to_string(),
        columns: None
    }
}

impl<T> Column<T> {
    pub fn new(name: &str) -> Column<T> {
        // Create a new column.
        Column {
            name:    name.to_string(),
            content: None
        }
    }

    pub fn set_content(mut self, content: Vec<T>) -> Column<T> {
        // Set the content of a column.
        self.content = Some(content);
        self
    }

    pub fn insert_content(mut self, val: T) -> Column<T> {
        // Insert content to the column.
        self.content.get_or_insert_with(Vec::new).push(val);
        self
    }
}

impl Table {
    pub fn set_columns(mut self, content: Vec<ColumnType>) -> Table {
        // Set the columns of a table.
        self.columns = Some(content);
        self
    }

    pub fn insert_column(mut self, val: ColumnType) -> Table {
        // Add a column to the table,
        self.columns.get_or_insert_with(Vec::new).push(val);
        self
    }
}