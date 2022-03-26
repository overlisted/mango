mod prelude;

mod admin;
mod db;
mod frontend;

pub use admin::fairing as admin;
pub use db::fairing as db;
pub use frontend::fairing as frontend;
