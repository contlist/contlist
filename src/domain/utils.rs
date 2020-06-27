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
