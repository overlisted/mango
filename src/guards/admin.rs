use super::prelude::*;

pub struct AdminAccess;

#[async_trait]
impl<'r> FromRequest<'r> for AdminAccess {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookie = request
            .cookies()
            .get("nothing")
            .map(|x| String::from(x.value()));

        if let Ok(key) = std::env::var("ADMIN_KEY") {
            if cookie == Some(key) {
                Outcome::Success(AdminAccess)
            } else {
                Outcome::Failure((Status::Unauthorized, ()))
            }
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
