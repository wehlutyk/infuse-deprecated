#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

extern crate iron;
#[macro_use] extern crate router;
#[macro_use] extern crate diesel;
extern crate logger;
extern crate dotenv;

pub mod schema;
pub mod models;
pub mod views;

use iron::prelude::*;
use iron::error::HttpResult;
use iron::Listening;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use logger::Logger;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub struct Infuse {
    pub server: Iron<Chain>,
}

impl Infuse {
    pub fn new() -> Infuse {
        let router = router!(get "/documents" => views::documents_handler,
                             get "/documents/:id" => views::document_handler);
        let (logger_before, logger_after) = Logger::new(None);

        let mut chain = Chain::new(router);
        chain.link_before(logger_before);
        chain.link_after(logger_after);

        Infuse { server: Iron::new(chain) }
    }

    pub fn serve(self) -> HttpResult<Listening> {
        self.server.http("localhost:3000")
    }
}
