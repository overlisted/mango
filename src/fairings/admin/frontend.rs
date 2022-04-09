use crate::fairings::prelude::*;

#[rocket::get("/")]
pub async fn index(_access: AdminAccess, db: Db) -> Template {
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

#[rocket::get("/ip-log?<page>&<search>")]
pub async fn ip_log(
    _access: AdminAccess,
    db: Db,
    page: Option<u32>,
    search: Option<std::net::IpAddr>,
) -> Template {
    let logs = db
        .run(move |conn| {
            let base = schema::ip_log::table
                .order(schema::ip_log::timestamp.desc())
                .offset(page.unwrap_or(0) as i64 * 50)
                .limit(50);

            if let Some(x) = search {
                base.filter(schema::ip_log::addr.eq(ipnetwork::IpNetwork::from(x)))
                    .load::<model::IpLogEntry>(conn)
            } else {
                base.load::<model::IpLogEntry>(conn)
            }
        })
        .await
        .expect("failed to load ip log");

    Template::render(
        "admin/ip-log",
        json!({ "entries": logs, "nextPage": page.unwrap_or(0) + 1, "search": search }),
    )
}
