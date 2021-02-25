use crate::infrastructure::web::current_user::CurrentUser;
use crate::infrastructure::web::error::Result as WebResult;
use crate::log::ResultInspectRef;

pub trait OptionExt<E> {
    fn err_and<T>(self, ok: T) -> Result<T, E>;
    fn err_and_then<T>(self, ok: impl FnOnce() -> T) -> Result<T, E>;
}

impl<E> OptionExt<E> for Option<E> {
    fn err_and<T>(self, ok: T) -> Result<T, E> {
        match self {
            Some(e) => Err(e),
            None => Ok(ok),
        }
    }

    fn err_and_then<T>(self, ok: impl FnOnce() -> T) -> Result<T, E> {
        match self {
            Some(e) => Err(e),
            None => Ok(ok()),
        }
    }
}

pub fn inspect_current_user(current_user: &WebResult<CurrentUser>) {
    current_user
        .inspect_err_ref(|e| log::warn!("the user is not logged in: {}", e))
        .inspect_ref(|c| log::debug!("current user: {:#?}", c));
}
