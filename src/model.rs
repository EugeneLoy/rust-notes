use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::schema::{notebooks, notes};

#[derive(Debug, PartialEq, Identifiable, Queryable, Selectable, Serialize, JsonSchema)]
#[diesel(table_name = notebooks)]
pub struct Notebook {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Insertable, AsChangeset, JsonSchema)]
#[diesel(table_name = notebooks)]
pub struct CreateUpdateNotebook {
    pub name: String
}

#[derive(Debug, PartialEq, Identifiable, Associations, Queryable, Selectable, Serialize, JsonSchema)]
#[diesel(table_name = notes)]
#[diesel(belongs_to(Notebook))]
pub struct Note {
    pub id: i32,
    pub content: String,
    pub notebook_id: i32
}

#[derive(Debug, Deserialize, Insertable, AsChangeset, JsonSchema)]
#[diesel(table_name = notes)]
pub struct CreateUpdateNote {
    pub content: String,
    pub notebook_id: i32
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct NotebookWithNotes {
    #[serde(flatten)]
    pub notebook: Notebook,
    pub notes: Vec<Note>
}
