use std::error::Error;
use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use diesel::{QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

use crate::model::*;
use crate::repository::Pool;
use crate::schema::notebooks;

trait CoerceErrExt<T, U : Error> {
    fn coerce_err(self) -> Result<T, Response>;
}

impl<T, U: Error> CoerceErrExt<T, U> for Result<T, U> {
    fn coerce_err(self) -> Result<T, Response> {
        self.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())
    }
}

pub async fn create_notebook(Extension(pool): Extension<Pool>, Json(create_notebook): Json<CreateUpdateNotebook>) -> Result<Json<i32>, Response> {
    diesel::insert_into(notebooks::table)
        .values(&create_notebook)
        .returning(Notebook::as_select())
        .get_result::<Notebook>(&mut pool.get().await.coerce_err()?).await
        .map(|notebook| Json(notebook.id))
        .coerce_err()
}

pub async fn get_notebook(Extension(pool): Extension<Pool>, Path(id): Path<i32>) -> Result<Json<Notebook>, Response> {
    notebooks::table
        .find(id)
        .get_result(&mut pool.get().await.coerce_err()?).await
        .map(Json)
        .map_err(|e| match e {
            diesel::NotFound => { StatusCode::NOT_FOUND.into_response() }
            e => { (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response() }
        })
}

pub async fn update_notebook(Extension(pool): Extension<Pool>, Path(id): Path<i32>, Json(update_notebook): Json<CreateUpdateNotebook>) -> Result<StatusCode, Response> {
    diesel::update(notebooks::table.find(id))
        .set(update_notebook)
        .execute(&mut pool.get().await.coerce_err()?).await
        .map(|updated| match updated {
            1 => StatusCode::OK,
            _ => StatusCode::BAD_REQUEST
        })
        .coerce_err()
}

pub async fn delete_notebook(Extension(pool): Extension<Pool>, Path(id): Path<i32>) -> Result<StatusCode, Response> {
    diesel::delete(notebooks::table.find(id))
        .execute(&mut pool.get().await.coerce_err()?).await
        .map(|updated| match updated {
            1 => StatusCode::OK,
            _ => StatusCode::BAD_REQUEST
        })
        .coerce_err()
}

pub async fn list_notebooks(Extension(pool): Extension<Pool>) -> Result<Json<Vec<Notebook>>, Response> {
    notebooks::table
        .select(Notebook::as_select())
        .get_results(&mut pool.get().await.coerce_err()?).await
        .map(Json)
        .coerce_err()
}
