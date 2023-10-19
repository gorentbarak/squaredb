use axum::{
    body::{Bytes, Full},
    extract,
    response::Response,
    routing::{get, post},
    Router,
};
use dirs::home_dir;

use crate::storage;
use crate::table_management::*;
pub async fn main() {
    let app = Router::new()
        .route("/post_table", post(post_table))
        .route("/drop_table/:name", post(drop_table))
        .route("/get_table/:name", get(get_table));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn post_table(extract::Json(table): extract::Json<Table>) -> Response<Full<Bytes>> {
    let _ = storage::table_to_file(format!(r"{}/.squaredb/{}", home_dir().unwrap().display(), &table.name).as_str(), table);
    Response::builder()
        .status(200)
        .header("Content-Type", "application/json; charset=utf-8")
        .body(Full::from("{}"))
        .unwrap()
}

async fn get_table(extract::Path(name): extract::Path<String>) -> Response<Full<Bytes>> {
    let table = storage::file_to_table(format!(r"{}/.squaredb/{}", home_dir().unwrap().display(), &name).as_str());
    let response = serde_json::to_string(&table.unwrap()).unwrap();
    Response::builder()
        .status(200)
        .header("Content-Type", "application/json; charset=utf-8")
        .body(Full::from(Bytes::from(response)))
        .unwrap()
}

async fn drop_table(extract::Path(name): extract::Path<String>) -> Response<Full<Bytes>> {
    let _ = tokio::fs::remove_dir_all(format!(r"{}/.squaredb/{}", home_dir().unwrap().display(), &name)).await;
    Response::builder()
        .status(200)
        .header("Content-Type", "application/json; charset=utf-8")
        .body(Full::from("{\"message\": \"Table dropped\"}"))
        .unwrap()
}
