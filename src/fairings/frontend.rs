use rocket::fs;

use super::prelude::*;

fn groups_into_json<E, T>(groups: Vec<Vec<(E, T)>>) -> Vec<Vec<serde_json::Value>>
where
    E: serde::Serialize,
    T: serde::Serialize,
{
    groups
        .into_iter()
        .map(|vec| {
            vec.into_iter()
                .map(|(e, t)| json!({ "extra": e, "type": t }))
                .collect()
        })
        .collect()
}

#[rocket::get("/")]
async fn index(db: Db, configs: PageConfigs) -> Template {
    let highlights: diesel::QueryResult<Vec<_>> = db
        .run(|conn| try {
            let projects = schema::projects::table
                .inner_join(schema::highlights::table)
                .select(schema::projects::all_columns)
                .order_by((schema::projects::started.desc(), schema::projects::name.desc()))
                .load::<model::Project>(conn)?;

            let links = groups_into_json(
                model::ProjectLink::belonging_to(&projects)
                    .inner_join(schema::link_types::table)
                    .load::<(model::ProjectLink, model::LinkType)>(conn)?
                    .grouped_by(&projects),
            );

            projects
                .into_iter()
                .zip(links)
                .map(|(p, l)| json!({ "project": p, "links": l }))
                .collect()
        })
        .await;

    Template::render(
        "index",
        json!({ "highlights": highlights.unwrap(), "config": configs.get("home").await }),
    )
}

#[rocket::get("/projects")]
async fn projects(db: Db, configs: PageConfigs) -> Template {
    let projects: diesel::QueryResult<Vec<_>> = db
        .run(|conn| try {
            let projects = schema::projects::table
                .order_by((schema::projects::started.desc(), schema::projects::name.desc()))
                .load::<model::Project>(conn)?;

            let links = groups_into_json(
                model::ProjectLink::belonging_to(&projects)
                    .inner_join(schema::link_types::table)
                    .load::<(model::ProjectLink, model::LinkType)>(conn)?
                    .grouped_by(&projects),
            );

            let tags = groups_into_json(
                model::ProjectTag::belonging_to(&projects)
                    .inner_join(schema::tag_types::table)
                    .load::<(model::ProjectTag, model::TagType)>(conn)?
                    .grouped_by(&projects),
            );

            projects
                .into_iter()
                .zip(links)
                .zip(tags)
                .map(|((p, l), t)| json!({ "project": p, "links": l, "tags": t }))
                .collect()
        })
        .await;

    Template::render(
        "projects",
        json!({
           "projects": projects.unwrap(),
           "config": configs.get("projects").await
        }),
    )
}

pub fn fairing() -> impl Fairing {
    AdHoc::on_ignite("Frontend", |rocket| async {
        rocket
            .mount("/", rocket::routes![index, projects])
            .mount("/", fs::FileServer::new("static", fs::Options::None))
    })
}
