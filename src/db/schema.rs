table! {
    configs (name) {
        name -> Varchar,
        data -> Jsonb,
    }
}

table! {
    projects (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(configs, projects,);
