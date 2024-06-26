use axum::response::IntoResponse;

pub(crate) async fn send_msg_handler() -> impl IntoResponse {
    "send msg"
}

pub(crate) async fn list_msg_handler() -> impl IntoResponse {
    "list msg"
}
