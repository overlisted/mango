use crate::fairings::prelude::*;

#[rocket::post("/login", data = "<key>")]
pub fn login(jar: &CookieJar<'_>, key: String) {
    jar.add(Cookie::new("nothing", key));
}

#[derive(FromForm)]
pub struct HighlightProject {
    id: String,
    highlight: String,
}

#[rocket::post("/highlights", data = "<data>")]
pub async fn highlight_project(_access: AdminAccess, db: Db, data: Form<Strict<HighlightProject>>) {
    let highlight = if &data.highlight == "true" {
        true
    } else {
        false
    };

    db.run(move |conn| {
        let id = schema::highlights::id.eq(&data.id);

        if highlight {
            diesel::insert_into(schema::highlights::table)
                .values(id)
                .execute(conn)
        } else {
            diesel::delete(schema::highlights::table.filter(id))
                .execute(conn)
        }
    })
    .await
    .unwrap();
}

#[rocket::post("/projects", data = "<data>")]
pub async fn upsert_project(_access: AdminAccess, db: Db, data: Form<Strict<model::Project>>) {
    db.run(move |conn| {
        diesel::insert_into(schema::projects::table)
            .values(&**data)
            .on_conflict(schema::projects::id)
            .do_update()
            .set(&**data)
            .execute(conn)
    })
    .await
    .unwrap();
}

#[rocket::post("/config/<name>", data = "<data>")]
pub async fn upsert_config(_access: AdminAccess, db: Db, name: String, data: Json<serde_json::Value>) {
    db.run(move |conn| {
        diesel::insert_into(schema::configs::table)
            .values(model::PageConfig {
                name: name.clone(),
                data: data.0.clone(),
            })
            .on_conflict(schema::configs::name)
            .do_update()
            .set(model::PageConfig { name, data: data.0 })
            .execute(conn)
    })
    .await
    .unwrap();
}
