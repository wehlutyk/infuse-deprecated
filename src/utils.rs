use diesel::pg::PgConnection;
use iron::prelude::*;
use iron::typemap::Key;
use persistent;
use r2d2;
use r2d2_diesel::ConnectionManager;
use router::Router;


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
    req.extensions.get::<Router>().unwrap().find(name).unwrap()
}
