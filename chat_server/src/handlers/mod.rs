mod auth;
mod chat;
mod msg;

use axum::response::IntoResponse;

pub(crate) use auth::*;
pub(crate) use chat::*;
pub(crate) use msg::*;

pub(crate) async fn index_handler() -> impl IntoResponse {
    "index"
}
