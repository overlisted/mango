use super::schema::*;
use rocket::FromForm;
use serde::Serialize;

#[derive(Queryable, Insertable, AsChangeset, Serialize, FromForm)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Queryable, Insertable, AsChangeset, Serialize)]
#[table_name = "configs"]
pub struct PageConfig {
    pub name: String,
    pub data: serde_json::Value,
}

#[derive(Queryable, Serialize)]
pub struct IpLogEntry {
    pub id: i64,
    pub addr: ipnetwork::IpNetwork,
    pub path: String,
    pub timestamp: chrono::NaiveDateTime,
}
