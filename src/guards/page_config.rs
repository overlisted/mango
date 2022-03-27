use super::prelude::*;

pub struct PageConfig<const NAME: &'static str>(pub serde_json::Value);

#[async_trait]
impl<'r, const NAME: &'static str> FromRequest<'r> for PageConfig<NAME> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let db = super::Db::get_one(request.rocket()).await.unwrap();

        let result = db
            .run(|conn| {
                schema::configs::table
                    .filter(schema::configs::name.eq(NAME))
                    .select(schema::configs::data)
                    .first::<serde_json::Value>(conn)
            })
            .await;

        if let Ok(json) = result {
            Outcome::Success(PageConfig(json))
        } else {
            Outcome::Failure((Status::InternalServerError, ()))
        }
    }
}
