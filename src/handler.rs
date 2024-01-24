use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    model::HistoryModel,
    schema::UpdateHistorySchema,
    AppState,
};

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Outlook helper";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn get_item_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as::<_, HistoryModel>(
            r#"SELECT * FROM history WHERE "topic" = $1 ORDER BY "date" DESC"#,
        )
        .bind(id)
        .fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all note items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let history = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": history.len(),
        "history": history
    });
    Ok(Json(json_response))
}

pub async fn update_item_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateHistorySchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query(
            r#"INSERT INTO history ("topic", "user", "action") VALUES ($1, $2, $3) RETURNING *"#
        )
        .bind(id)
        .bind(body.user.to_owned())
        .bind(body.action.to_owned())
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(_) => {
            let note_response = json!({"status": "success"});

            return Ok((StatusCode::CREATED, Json(note_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}

pub async fn delete_history_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query(
            r#"DELETE FROM history WHERE "id" = $1"#
        )
        .bind(id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}
