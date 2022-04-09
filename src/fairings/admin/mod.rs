use super::prelude::*;

mod api;
mod frontend;

#[rocket::catch(401)]
fn teapot() -> (Status, &'static str) {
    (Status { code: 418 }, "🫖")
}

pub fn fairing() -> impl Fairing {
    AdHoc::on_ignite("Admin Frontend", |rocket| async {
        rocket
            .register("/admin", rocket::catchers![teapot])
            .mount(
                "/admin",
                rocket::routes![api::login, frontend::index, frontend::ip_log],
            )
            .mount(
                "/admin/api",
                rocket::routes![
                    api::highlight_project,
                    api::upsert_project,
                    api::upsert_config
                ],
            )
    })
}
