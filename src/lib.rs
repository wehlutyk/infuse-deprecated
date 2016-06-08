#![feature(custom_derive, custom_attribute, plugin)]

#![plugin(diesel_codegen, dotenv_macros, clippy)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate iron;
extern crate logger;
extern crate params;
extern crate persistent;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate router;

pub mod models;
pub mod schema;
pub mod utils;
pub mod views;

use diesel::pg::PgConnection;
use dotenv::dotenv;
use iron::error;
use iron::prelude::*;
use r2d2::{Config, Pool};
use r2d2_diesel::ConnectionManager;
use std::env;


pub struct Infuse {
    server: Iron<Chain>,
}

impl Default for Infuse {
    fn default() -> Infuse {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let config = Config::default();
        let pool = Pool::new(config, manager).expect("Failed to create connection pool");

        let router = router!(get "/jobs" => views::jobs_handler,
                             get "/jobs/:id" => views::job_handler,
                             get "/documents" => views::documents_handler,
                             post "/documents" => views::new_document_handler,
                             get "/documents/:id" => views::document_handler);
        let logger = logger::Logger::new(None);

        let mut chain = Chain::new(router);
        chain.link(logger);
        chain.link(persistent::Read::<utils::Database>::both(pool));

        Infuse { server: Iron::new(chain) }
    }
}

impl Infuse {
    pub fn serve(self) -> error::HttpResult<iron::Listening> {
        self.server.http("localhost:3000")
    }
}
