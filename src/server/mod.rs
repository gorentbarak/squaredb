#![allow(warnings)]

use axum::{
    body::{Bytes, Full},
    extract,
    response::Response,
    routing::{get, post},
    Router,
};

use crate::storage;
use crate::table_management::*;
pub async fn main() {
    let app = Router::new().route("/post_table", post(post_table));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn post_table(extract::Json(table): extract::Json<Table>) -> Response<Full<Bytes>> {
    let _ = storage::table_to_file(format!(r"$HOME/.squaredb/{}", &table.name).as_str(), table);
    return json_response(
        r#"{
        \"message\": \"Table created successfully!\"
    }"#,
    )
    .await;
}

async fn json_response(body: &'static str) -> Response<Full<Bytes>> {
    Response::builder()
        .status(200)
        .header("Content-Type", "application/json; charset=utf-8")
        .body(Full::from(body))
        .unwrap()
}
