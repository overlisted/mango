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

    Template::render("admin/index", json!({ "projects": projects }))
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

pub fn fairing() -> impl Fairing {
    AdHoc::on_ignite("Admin Frontend", |rocket| async {
        rocket
            .register("/admin", rocket::catchers![teapot])
            .mount("/admin", rocket::routes![login, index])
            .mount("/admin/api", rocket::routes![upsert_project])
    })
}
