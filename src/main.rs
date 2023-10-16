pub mod server;
pub mod storage;
pub mod table_management;
#[tokio::main]
async fn main() {
    const MESSAGE: [&str; 3] = ["SquareDB",
    "To get a table from the database, go to https://localhost:3000/get_table/<table_name>",
    "To send a table to the databse, go to https://localhost:3000/post_table and send JSON-encoded content with the table's data and a request header Content-Type: application/json"];
    for i in MESSAGE.iter() {
        println!("{}", i);
    }
    server::main().await;
}
