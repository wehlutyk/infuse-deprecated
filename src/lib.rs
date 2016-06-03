#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

extern crate iron;
#[macro_use]
extern crate router;
#[macro_use]
extern crate diesel;
extern crate persistent;
extern crate params;
extern crate logger;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

pub mod schema;
pub mod models;
pub mod views;

use iron::prelude::*;
use iron::error::HttpResult;
use iron::Listening;
use iron::typemap::Key;
use diesel::pg::PgConnection;
use persistent::Read;
use r2d2_diesel::ConnectionManager;
use logger::Logger;
use dotenv::dotenv;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
pub struct Database;
impl Key for Database {
    type Value = Pool;
}

fn get_pool_connection(req: &Request) -> PooledConnection {
    let pool = req.extensions.get::<Read<Database>>().expect("Database component not initialised");
    pool.get().unwrap()
}

pub struct Infuse {
    server: Iron<Chain>,
}

impl Infuse {
    pub fn new() -> Infuse {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let config = r2d2::Config::default();
        let pool = r2d2::Pool::new(config, manager).expect("Failed to create connection pool");

        let router = router!(get "/jobs" => views::jobs_handler,
                             post "/jobs/new" => views::new_job_handler,
                             get "/jobs/:id" => views::job_handler,
                             get "/documents" => views::documents_handler,
                             get "/documents/:id" => views::document_handler);
        let logger = Logger::new(None);

        let mut chain = Chain::new(router);
        chain.link(logger);
        chain.link(Read::<Database>::both(pool));

        Infuse { server: Iron::new(chain) }
    }

    pub fn serve(self) -> HttpResult<Listening> {
        self.server.http("localhost:3000")
    }
}
