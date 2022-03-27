use super::prelude::*;

#[rocket::catch(401)]
fn teapot() -> (Status, &'static str) {
    (Status { code: 418 }, "🫖")
}

#[rocket::post("/login", data = "<key>")]
fn login(jar: &CookieJar<'_>, key: String) {
    jar.add_private(Cookie::new("nothing", key));
}

#[rocket::get("/")]
async fn index(_access: AdminAccess, db: Db) -> Template {
    let projects = db
        .run(|conn| schema::projects::table.load::<model::Project>(conn))
        .await
        .expect("failed to load projects");
    let configs: Vec<_> = db
        .run(|conn| schema::configs::table.load::<model::PageConfig>(conn))
        .await
        .expect("failed to load configs")
        .into_iter()
        .map(|it| json!({ "name": it.name, "data": serde_json::to_string_pretty(&it.data).unwrap() }))
        .collect();

    Template::render(
        "admin/index",
        json!({ "projects": projects, "configs": configs }),
    )
}

#[rocket::post("/projects", data = "<data>")]
async fn upsert_project(_access: AdminAccess, db: Db, data: Form<Strict<model::Project>>) {
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
async fn upsert_data(_access: AdminAccess, db: Db, name: String, data: Json<serde_json::Value>) {
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

pub fn fairing() -> impl Fairing {
    AdHoc::on_ignite("Admin Frontend", |rocket| async {
        rocket
            .register("/admin", rocket::catchers![teapot])
            .mount("/admin", rocket::routes![login, index])
            .mount("/admin/api", rocket::routes![upsert_project, upsert_data])
    })
}
