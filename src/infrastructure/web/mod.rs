pub mod current_user;
pub mod error;

mod endpoints;
pub use endpoints::user_endpoints as user;
pub use endpoints::contact_endpoints as contact;
