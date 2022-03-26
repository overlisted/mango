use diesel::PgConnection;

#[rocket_sync_db_pools::database("api")]
pub struct Db(PgConnection);
