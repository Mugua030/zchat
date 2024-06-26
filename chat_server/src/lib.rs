mod config;
mod handlers;

use handlers::*;
use std::{ops::Deref, sync::Arc};

use axum::{
    routing::{get, patch, post},
    Router,
};

pub use config::AppConfig;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}

#[allow(unused)]
#[derive(Debug)]
pub(crate) struct AppStateInner {
    pub(crate) config: AppConfig,
}

pub fn get_router(config: AppConfig) -> Router {
    let state = AppState::new(config);

    // user route
    let user_routes: Router<AppState> = Router::new()
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler));

    // chat route
    let chat_routes = Router::new()
        .route("/chat", get(list_chat_handler).post(create_chat_handler))
        .route(
            "/chat/:id", 
            patch(update_chat_handler)
                            .delete(delete_chat_handler)
                            .post(send_msg_handler)
        )
        .route("/chat/:id/msg", get(list_msg_handler));

    let api_routes = Router::new()
        .route("/", get(index_handler))
        .merge(user_routes)
        .merge(chat_routes);

    // - POST /api/signin
    // - POST /api/signup
    // - GET  /api/chat
    Router::new().nest("/api", api_routes).with_state(state)

}

impl AppState {
    fn new(config: AppConfig) -> Self {
        Self {
            inner: Arc::new(AppStateInner { config }),
        }
    }
}

// state.config => state.inner.config
impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
