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
    link_types (id) {
        id -> Int4,
        icon -> Varchar,
        bg_color -> Varchar,
        fg_color -> Varchar,
    }
}

table! {
    project_links (id) {
        id -> Int8,
        url -> Varchar,
        name -> Varchar,
        link_type_id -> Int4,
        project_id -> Varchar,
    }
}

table! {
    project_tags (id) {
        id -> Int8,
        tag_type_id -> Int4,
        project_id -> Varchar,
    }
}

table! {
    projects (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Varchar,
        image -> Nullable<Varchar>,
    }
}

table! {
    tag_types (id) {
        id -> Int4,
        name -> Varchar,
        bg_color -> Varchar,
        fg_color -> Varchar,
    }
}

joinable!(highlights -> projects (id));
joinable!(project_links -> link_types (link_type_id));
joinable!(project_links -> projects (project_id));
joinable!(project_tags -> projects (project_id));
joinable!(project_tags -> tag_types (tag_type_id));

allow_tables_to_appear_in_same_query!(
    configs,
    highlights,
    ip_log,
    link_types,
    project_links,
    project_tags,
    projects,
    tag_types,
);
