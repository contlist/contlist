use crate::infrastructure::repository::pool::R2D2Pool;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use shaku::{HasComponent, Module, Provider};
use std::error::Error;
use std::ops::{Deref, DerefMut};

// A wrapper for r2d2::PooledConnection to be able to implement shaku::Provider for it
#[derive(Debug)]
pub struct R2D2Connection<C: diesel::Connection + 'static>(PooledConnection<ConnectionManager<C>>);

impl<C: diesel::Connection + 'static> R2D2Connection<C> {
    pub fn new(conn: PooledConnection<ConnectionManager<C>>) -> Self {
        Self(conn)
    }
}

impl<C: diesel::Connection + 'static> Deref for R2D2Connection<C> {
    type Target = PooledConnection<ConnectionManager<C>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<C: diesel::Connection + 'static> DerefMut for R2D2Connection<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<M, C> Provider<M> for R2D2Connection<C>
where
    M: Module + HasComponent<R2D2Pool<C>>,
    C: diesel::Connection + 'static,
{
    type Interface = R2D2Connection<C>;

    fn provide(module: &M) -> Result<Box<Self::Interface>, Box<dyn Error>> {
        module
            .resolve_ref()
            .get()
            .map_err(|e| -> Box<dyn Error> { Box::new(e) })
            .map(Self::new)
            .map(Box::new)
    }
}
