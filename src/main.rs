extern crate infuse;
extern crate iron;
#[macro_use]
extern crate router;
extern crate logger;

use iron::prelude::*;
use logger::Logger;
use infuse::views;

// 3 endpoints:
// - /process: send a pdf to process, get link to job
// - /jobs/:id: get active/completed jobs
// - /documents/:id: get processed documents

fn main() {
    let router = router!(get "/documents" => views::documents_handler,
                         get "/documents/:id" => views::document_handler);
    let (logger_before, logger_after) = Logger::new(None);

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:3000").unwrap();
}
