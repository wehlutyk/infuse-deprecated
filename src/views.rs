use iron::prelude::*;
use iron::status;
use router::Router;

use get_pool_connection;
use models::*;
use diesel::prelude::*;
use diesel::result::Error::NotFound as DieselNotFound;
use std::num::ParseIntError;

pub fn documents_handler(req: &mut Request) -> IronResult<Response> {
    use schema::documents::dsl::documents;

    let connection = get_pool_connection(req);
    // FIXME: Why is &* necessary? I thought Deref coercion was supposed to solve
    // that.
    let results = documents.limit(5)
        .load::<Document>(&*connection)
        .expect("Error loading documents");

    let mut response = format!("Displaying {} documents", results.len());
    for document in results {
        response.push_str(&format!("{}", document.hash));
        response.push_str(&format!("----------\n"));
        response.push_str(&format!("{}", document.tei));
    }

    Ok(Response::with((status::Ok, response)))
}

pub fn document_handler(req: &mut Request) -> IronResult<Response> {
    use schema::documents::dsl::documents;

    let id = match req.extensions.get::<Router>().unwrap().find("id").unwrap().parse::<i32>() {
        Ok(id) => id,
        Err(ParseIntError { .. }) => return Ok(Response::with(status::NotFound)),
    };

    let connection = get_pool_connection(req);
    // FIXME: Why is &* necessary? I thought Deref coercion was supposed to solve
    // that.
    match documents.find(id).first::<Document>(&*connection) {
        Ok(document) => {
            let mut response = format!("{}", document.hash);
            response.push_str(&format!("----------\n"));
            response.push_str(&format!("{}", document.tei));
            Ok(Response::with((status::Ok, response)))
        }
        Err(DieselNotFound) => Ok(Response::with(status::NotFound)),
        Err(err) => panic!(err),
    }
}
