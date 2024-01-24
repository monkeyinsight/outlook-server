use std::sync::Arc;

use axum::{
    routing::{get,delete},
    Router,
};

use crate::{
    handler::{
        get_item_handler,
        update_item_handler,
        delete_history_handler,
        health_checker_handler
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route(
            "/api/items/:id",
            get(get_item_handler)
                .patch(update_item_handler),
        )
        .route(
            "/api/history/:id",
            delete(delete_history_handler),
        )
        .with_state(app_state)
}
