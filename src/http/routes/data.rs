use actix_web::{get, post, web, HttpResponse, Responder};
use crate::{models::item::{DataFile, Item, NewItem}, services::fs_store};
use crate::state::AppState;

#[get("/data")]
pub async fn get_data(state: web::Data<AppState>) -> impl Responder {
    let _r = state.file_lock.read().await;
    match fs_store::read_data(&state.data_path).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            tracing::error!(error = %e, "failed to read data file");
            HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "failed_to_read_data" }))
        }
    }
}

#[post("/data")]
pub async fn add_item(state: web::Data<AppState>, payload: web::Json<NewItem>) -> impl Responder {
    let _w = state.file_lock.write().await;

    let mut data = match fs_store::read_data(&state.data_path).await {
        Ok(d) => d,
        Err(e) => {
            tracing::error!(error = %e, "read before write failed");
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "read_failed" }));
        }
    };

    if let Some(existing) = data.items.iter_mut().find(|it| it.id == payload.id) {
        existing.name = payload.name.clone();
    } else {
        data.items.push(Item { id: payload.id.clone(), name: payload.name.clone() });
    }

    if let Err(e) = fs_store::write_data(&state.data_path, &data).await {
        tracing::error!(error = %e, "write failed");
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({ "error": "write_failed" }));
    }

    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}
