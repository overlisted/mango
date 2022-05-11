use super::prelude::*;

diesel_migrations::embed_migrations!("db/migrations");

async fn run_migrations(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    let conn = Db::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("diesel migrations");

    rocket
}

pub fn fairing() -> impl Fairing {
    AdHoc::on_ignite("Database", |rocket| async {
        rocket
            .attach(Db::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
    })
}
