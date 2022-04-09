table! {
    configs (name) {
        name -> Varchar,
        data -> Jsonb,
    }
}

table! {
    highlights (id) {
        id -> Varchar,
    }
}

table! {
    ip_log (id) {
        id -> Int8,
        addr -> Inet,
        path -> Varchar,
        timestamp -> Timestamp,
    }
}

table! {
    projects (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Varchar,
    }
}

joinable!(highlights -> projects (id));

allow_tables_to_appear_in_same_query!(
    configs,
    highlights,
    ip_log,
    projects,
);
