//! cry about it

use std::net;

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
        let path = req.uri().path().to_string();

        let option = if let Some(value) = req.headers().get_one("X-Real-IP") {
            value.parse::<net::IpAddr>().ok()
        } else if let Some(addr) = req.remote() {
            Some(addr.ip())
        } else {
            None
        };

        if let Some(addr) = option {
            db.run(move |conn| {
                diesel::insert_into(schema::ip_log::table)
                    .values((
                        schema::ip_log::addr.eq(ipnetwork::IpNetwork::from(addr)),
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
