use super::prelude::*;

pub struct AdminAccess;

#[async_trait]
impl<'r> FromRequest<'r> for AdminAccess {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookie = request
            .cookies()
            .get_private("nothing")
            .map(|x| String::from(x.value()));

        // no key = everyone has access. useful for development because rocket resets the encryption
        // thing after every restart
        if cookie == std::env::var("ADMIN_KEY").ok() {
            Outcome::Success(AdminAccess)
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
