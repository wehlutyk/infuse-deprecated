#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros, clippy, serde_macros)]

extern crate crypto;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate hyper;
extern crate iron;
#[macro_use]
extern crate log;
extern crate logger;
extern crate multipart;
extern crate params;
extern crate persistent;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate router;
extern crate serde;
extern crate serde_json;

pub mod files;
pub mod models;
pub mod processing;
pub mod schema;
pub mod serializers;
pub mod utils;
pub mod views;

use diesel::pg::PgConnection;
use dotenv::dotenv;
use iron::error;
use iron::prelude::*;
use processing::Processor;
use r2d2::Config;
use r2d2_diesel::ConnectionManager;
use std::env;
use std::sync::mpsc;
use utils::{Pool, Database};


pub struct Infuse {
    server: Iron<Chain>,
    processor: Processor,
}

impl Default for Infuse {
    fn default() -> Infuse {
        env_logger::init().unwrap();
        dotenv().ok();

        // Create pool.
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let config = Config::default();
        let pool = Pool::new(config, manager).expect("Failed to create connection pool");

        // Create router.
        let router = router!(get "/jobs" => views::jobs_handler,
                             get "/jobs/:id" => views::job_handler,
                             get "/documents" => views::documents_handler,
                             post "/documents" => views::new_document_handler,
                             get "/documents/:id" => views::document_handler);

        // Create logger.
        let logger = logger::Logger::new(None);

        // Create processor.
        let (sender, receiver) = mpsc::channel();
        let processor = Processor::new((sender.clone(), receiver), pool.clone());

        // Link it all together.
        let mut chain = Chain::new(router);
        chain.link(logger);
        chain.link_before(persistent::Read::<Database>::one(pool));
        chain.link_before(persistent::Write::<Processor>::one(sender.clone()));

        Infuse { server: Iron::new(chain), processor: processor }
    }
}

impl Infuse {
    pub fn serve(self) -> error::HttpResult<iron::Listening> {
        let port = env::var("INFUSE_PORT").expect("INFUSE_PORT must be set");
        let address = "localhost:".to_string() + &port;
        self.processor.start();
        self.server.http(&*address)
    }
}
