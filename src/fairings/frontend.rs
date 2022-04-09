use rocket::fs;

use super::prelude::*;

#[rocket::get("/")]
async fn index(db: Db, configs: PageConfigs) -> Template {
    let highlights = db
        .run(|conn| {
            schema::projects::table
                .inner_join(schema::highlights::table)
                .select(schema::projects::all_columns)
                .load::<model::Project>(conn)
        })
        .await
        .expect("failed to load projects");

    Template::render(
        "index",
        json!({ "highlights": highlights, "config": configs.get("home").await.unwrap() }),
    )
}

pub fn fairing() -> impl Fairing {
    AdHoc::on_ignite("Frontend", |rocket| async {
        rocket
            .attach(Template::fairing())
            .mount("/", rocket::routes![index])
            .mount("/", fs::FileServer::new("static", fs::Options::None))
    })
}
