use super::prelude::*;

pub struct PageConfigs(super::Db);

impl PageConfigs {
    pub async fn get(&self, name: &'static str) -> Option<serde_json::Value> {
        self.0
            .run(move |conn| {
                schema::configs::table
                    .filter(schema::configs::name.eq(name))
                    .select(schema::configs::data)
                    .first::<serde_json::Value>(conn)
            })
            .await
            .ok()
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for PageConfigs {
    type Error = <super::Db as FromRequest<'r>>::Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match super::Db::from_request(request).await {
            Outcome::Success(db) => Outcome::Success(PageConfigs(db)),
            Outcome::Failure(x) => Outcome::Failure(x),
            Outcome::Forward(x) => Outcome::Forward(x),
        }
    }
}
