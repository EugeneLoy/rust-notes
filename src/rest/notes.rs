use axum::Json;
use axum::extract::{State, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use diesel::{QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

use crate::rest::CoerceErrExt;
use crate::model::*;
use crate::repository::Pool;
use crate::schema::notes;


pub async fn create_note(State(pool): State<Pool>, Json(create_note): Json<CreateUpdateNote>) -> Result<Json<i32>, Response> {
    diesel::insert_into(notes::table)
        .values(&create_note)
        .returning(Note::as_select())
        .get_result::<Note>(&mut pool.get().await.coerce_err()?).await
        .map(|note| Json(note.id))
        .coerce_err()
}

pub async fn get_note(State(pool): State<Pool>, Path(id): Path<i32>) -> Result<Json<Note>, Response> {
    notes::table
        .find(id)
        .get_result(&mut pool.get().await.coerce_err()?).await
        .map(Json)
        .map_err(|e| match e {
            diesel::NotFound => { StatusCode::NOT_FOUND.into_response() }
            e => { (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response() }
        })
}

pub async fn update_note(State(pool): State<Pool>, Path(id): Path<i32>, Json(update_note): Json<CreateUpdateNote>) -> Result<StatusCode, Response> {
    diesel::update(notes::table.find(id))
        .set(&update_note)
        .execute(&mut pool.get().await.coerce_err()?).await
        .map(|updated| match updated {
            1 => StatusCode::OK,
            _ => StatusCode::BAD_REQUEST
        })
        .coerce_err()
}

pub async fn delete_note(State(pool): State<Pool>, Path(id): Path<i32>) -> Result<StatusCode, Response> {
    diesel::delete(notes::table.find(id))
        .execute(&mut pool.get().await.coerce_err()?).await
        .map(|deleted| match deleted {
            1 => StatusCode::OK,
            _ => StatusCode::BAD_REQUEST
        })
        .coerce_err()
}

