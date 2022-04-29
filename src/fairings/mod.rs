mod prelude;

mod admin;
mod db;
mod frontend;
mod handlebars;
mod ip_log;

pub use admin::fairing as admin;
pub use db::fairing as db;
pub use frontend::fairing as frontend;
pub use self::handlebars::fairing as handlebars;
pub use ip_log::fairing as ip_log;
