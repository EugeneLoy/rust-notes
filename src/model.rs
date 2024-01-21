use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::schema::notebooks;

#[derive(Debug, Queryable, Selectable, Serialize, JsonSchema)]
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
