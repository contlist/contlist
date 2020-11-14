use shaku::Interface;
use std::error::Error;

pub trait Pool: Interface {
    type PooledConnection;

    type Error: Error;

    fn get(&self) -> Result<Self::PooledConnection, Self::Error>;
}
