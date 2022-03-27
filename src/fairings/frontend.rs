use rocket::fs;

use super::prelude::*;

#[rocket::get("/")]
async fn index(db: Db, config: PageConfig<"home">) -> Template {
    let projects = db
        .run(|conn| schema::projects::table.load::<model::Project>(conn))
        .await
        .expect("failed to load projects");

    Template::render("index", json!({ "projects": projects, "config": config.0 }))
}

pub fn fairing() -> impl Fairing {
    AdHoc::on_ignite("Frontend", |rocket| async {
        rocket
            .attach(Template::fairing())
            .mount("/", rocket::routes![index])
            .mount("/", fs::FileServer::new("static", fs::Options::None))
    })
}
