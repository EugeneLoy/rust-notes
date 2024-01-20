use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::notebooks;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = notebooks)]
pub struct Notebook {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = notebooks)]
pub struct CreateUpdateNotebook {
    pub name: String
}
