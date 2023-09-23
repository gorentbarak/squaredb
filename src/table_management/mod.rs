#![allow(dead_code)] // For now.
use uuid;
/* 
 * DISCLAIMER:
 * This is currently only stored in memory.
 * I will later make a file format, and add many other optimizations.
*/
#[derive(Debug)]

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
    rows: Vec<Row>
}

#[derive(Debug)]
pub struct Column<T> {
    /* This is for a column in our database.
     * It's parent is a Row.
     */
    name:    String,
    content: Option<T>
}
#[derive(Debug)]
pub struct Row {
    /* This is a row in our database.
     * It's child is a Column.
    */
    id: String, // Use uuid version 4.
    columns: Vec<ColumnType>,
}

pub fn create_table(name: &str) -> Table {
    // Create a new table.
    Table {
        name: name.to_string(),
        rows: Vec::new()
    }
}

impl Row {
    pub fn new(columns: Vec<ColumnType>) -> Row {
        Row {
            id: uuid::Uuid::new_v4().to_string(),
            columns
        }
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

    pub fn set_content(mut self, content: T) -> Column<T> {
        // Set the content of a column.
        self.content = Some(content);
        self
    }
}

impl Table {
    pub fn set_rows(mut self, content: Vec<Row>) -> Table {
        // Set the columns of a table.
        self.rows = content;
        self
    }

    pub fn insert_row(mut self, val: Row) -> Table {
        // Add a column to the table,
        self.rows.get_or_insert_with(Vec::new).push(val);
        self
    }

}