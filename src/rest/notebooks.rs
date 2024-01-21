use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use diesel::{BelongingToDsl, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{AsyncConnection, RunQueryDsl};
use diesel_async::scoped_futures::ScopedFutureExt;

use crate::rest::CoerceErrExt;
use crate::model::*;
use crate::repository::Pool;
use crate::schema::{notebooks, notes};


pub async fn create_notebook(Extension(pool): Extension<Pool>, Json(create_notebook): Json<CreateUpdateNotebook>) -> Result<Json<i32>, Response> {
    diesel::insert_into(notebooks::table)
        .values(&create_notebook)
        .returning(Notebook::as_select())
        .get_result::<Notebook>(&mut pool.get().await.coerce_err()?).await
        .map(|notebook| Json(notebook.id))
        .coerce_err()
}

pub async fn get_notebook(Extension(pool): Extension<Pool>, Path(id): Path<i32>) -> Result<Json<NotebookWithNotes>, Response> {
    let notebook: Notebook = notebooks::table
        .find(id)
        .get_result(&mut pool.get().await.coerce_err()?).await
        .map_err(|e| match e {
            diesel::NotFound => { StatusCode::NOT_FOUND.into_response() }
            e => { (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response() }
        })?;

    // TODO this can be done without round trip to db (using join on query above):
    Note::belonging_to(&notebook)
        .get_results(&mut pool.get().await.coerce_err()?).await
        .map(|notes| Json(NotebookWithNotes { notebook, notes }))
        .coerce_err()
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
    (pool.get().await.coerce_err()?).transaction::<StatusCode, diesel::result::Error, _>(|connection| async move {
        diesel::delete(notes::table)
            .filter(notes::notebook_id.eq(id))
            .execute(connection).await?;

        diesel::delete(notebooks::table.find(id))
            .execute(connection).await
            .map(|deleted| match deleted {
                1 => StatusCode::OK,
                _ => StatusCode::BAD_REQUEST
            })
    }.scope_boxed()).await.coerce_err()
}

pub async fn list_notebooks(Extension(pool): Extension<Pool>) -> Result<Json<Vec<Notebook>>, Response> {
    notebooks::table
        .select(Notebook::as_select())
        .get_results(&mut pool.get().await.coerce_err()?).await
        .map(Json)
        .coerce_err()
}

