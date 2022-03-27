//! cry about it

use super::prelude::*;

struct IpLogger;

#[async_trait]
impl Fairing for IpLogger {
    fn info(&self) -> Info {
        Info {
            kind: Kind::Request,
            name: "IP Logger",
        }
    }

    async fn on_request(&self, req: &mut rocket::Request<'_>, _data: &mut rocket::Data<'_>) {
        let db = Db::get_one(req.rocket()).await.unwrap();

        if let Some(addr) = req.remote() {
            let path = req.uri().path().to_string();
            db.run(move |conn| {
                diesel::insert_into(schema::ip_log::table)
                    .values((
                        schema::ip_log::addr.eq(ipnetwork::IpNetwork::from(addr.ip())),
                        schema::ip_log::path.eq(path),
                    ))
                    .execute(conn)
            })
            .await
            .ok();
        }
    }
}

pub fn fairing() -> impl Fairing {
    IpLogger
}
