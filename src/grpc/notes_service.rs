use diesel::{QueryDsl, BelongingToDsl};
use diesel_async::RunQueryDsl;
use tonic::{Request, Response, Status};
use crate::grpc::proto::notes_service_server;
use crate::grpc::proto::{GetNotebook, Notebook, Note};
use crate::repository::Pool;
use crate::schema::{notebooks};
use crate::model;


pub struct NotesService {
    pool: Pool
}

impl NotesService {
    pub fn new(pool: Pool) -> NotesService {
        NotesService { pool }
    }
}

#[tonic::async_trait]
impl notes_service_server::NotesService for NotesService {
    async fn get_notebook(&self, request: Request<GetNotebook>) -> Result<Response<Notebook>, Status> {
        let notebook_id = request.into_inner().notebook_id;
        let mut connection = self.pool.get().await.map_err(|e| Status::unknown(e.to_string()))?;

        let notebook = notebooks::table
            .find(notebook_id)
            .get_result::<model::Notebook>(&mut connection).await
            .map_err(|e| match e {
                diesel::NotFound => { Status::not_found(format!("Notebook with id={} not found.", notebook_id)) }
                e => { Status::unknown(e.to_string()) }
            })?;

        let notes = model::Note::belonging_to(&notebook)
            .get_results::<model::Note>(&mut connection).await
            .map_err(|e| Status::unknown(e.to_string()))?;

        let notebook = Notebook {
            id: notebook.id.into(),
            name: notebook.name.into(),
            notes: notes.into_iter().map(|note| Note {
                id: note.id.into(),
                text: note.content.into(),
                notebook_id: note.notebook_id.into()
            }).collect::<Vec<Note>>()
        };

        Ok(Response::new(notebook))
    }
}
