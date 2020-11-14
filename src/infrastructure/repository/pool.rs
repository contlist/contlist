use diesel::connection::Connection;
use diesel::r2d2::{self, ConnectionManager};
use shaku::{Component, Module, ModuleBuildContext};
use std::env;
use std::ops::Deref;

// A wrapper for r2d2::Pool to be able to implement shaku::Component for it
#[derive(Debug)]
pub struct R2D2Pool<C: Connection + 'static>(r2d2::Pool<ConnectionManager<C>>);

impl<C: Connection> R2D2Pool<C> {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<C>::new(database_url);
        r2d2::Pool::new(manager)
            .map(Self)
            .expect("failed to create database pool")
    }
}

impl<C: Connection> Deref for R2D2Pool<C> {
    type Target = r2d2::Pool<ConnectionManager<C>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Manual implementation to specify a Self type as an interface instead of some trait
impl<M, C> Component<M> for R2D2Pool<C>
where
    M: Module,
    C: diesel::Connection + 'static,
{
    type Interface = Self;
    type Parameters = ();

    fn build(
        _context: &mut ModuleBuildContext<M>,
        _params: Self::Parameters,
    ) -> Box<Self::Interface> {
        box Self::new()
    }
}
