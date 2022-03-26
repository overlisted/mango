use super::schema::*;
use rocket::FromForm;
use serde::Serialize;

#[derive(Queryable, Insertable, AsChangeset, Serialize, FromForm)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
}
