use diesel::PgConnection;

#[rocket_sync_db_pools::database("diesel")]
pub struct Db(PgConnection);
