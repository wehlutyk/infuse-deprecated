use diesel::pg::PgConnection;
use iron::prelude::*;
use iron::typemap::Key;
use params::{Value, Params};
use persistent;
use r2d2;
use r2d2_diesel::ConnectionManager;
use router::Router;
use std::sync::MutexGuard;

use processing::{Processor, ProcessorSender};


pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
pub struct Database;
impl Key for Database {
    type Value = Pool;
}

pub fn get_pool_connection(req: &Request) -> PooledConnection {
    let pool = req.extensions
        .get::<persistent::Read<Database>>()
        .expect("Database component not initialised");
    pool.get().unwrap()
}

pub fn get_router_param<'a>(req: &'a Request, name: &str) -> &'a str {
    req.extensions.get::<Router>()
        .expect("Router component not initialised")
        .find(name).unwrap()
}

pub fn get_param<'a>(req: &'a mut Request, path: &[&str]) -> Option<&'a Value> {
    req.get_ref::<Params>()
        .expect("Params component not initialised")
        .find(path)
}

pub fn get_processor_sender<'a>(req: &'a Request) -> MutexGuard<'a, ProcessorSender> {
    let sender = req.extensions
        .get::<persistent::Write<Processor>>()
        .expect("Processor component not initialised");
    sender.lock().unwrap()
}
