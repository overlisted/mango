use super::schema::*;
use serde::*;

#[derive(Identifiable, Queryable, Insertable, AsChangeset, Serialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
}

#[derive(Associations, Identifiable, Queryable, Serialize)]
#[belongs_to(TagType)]
#[belongs_to(Project)]
pub struct ProjectTag {
    pub id: i64,
    pub tag_type_id: i32,
    pub project_id: String,
}

#[derive(Associations, Identifiable, Queryable, Serialize)]
#[belongs_to(LinkType)]
#[belongs_to(Project)]
pub struct ProjectLink {
    pub id: i64,
    pub url: String,
    pub name: String,
    pub link_type_id: i32,
    pub project_id: String,
}

#[derive(Identifiable, Queryable, Serialize)]
pub struct TagType {
    pub id: i32,
    pub name: String,
    pub bg_color: String,
    pub fg_color: String,
}

#[derive(Identifiable, Queryable, Serialize)]
pub struct LinkType {
    pub id: i32,
    pub icon: String,
    pub bg_color: String,
    pub fg_color: String,
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
