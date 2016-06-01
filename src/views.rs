use iron::prelude::*;
use iron::status;
use router::Router;

use establish_connection;
use models::*;
use diesel::prelude::*;
use diesel::result::Error::NotFound as DieselNotFound;
use std::num::ParseIntError;

pub fn documents_handler(_: &mut Request) -> IronResult<Response> {
    use schema::documents::dsl::documents;

    let connection = establish_connection();
    let results = documents.limit(5)
        .load::<Document>(&connection)
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

    let router = req.extensions.get::<Router>().unwrap();
    let id = match router.find("id").unwrap().parse::<i32>() {
        Ok(id) => id,
        Err(ParseIntError { .. }) => return Ok(Response::with(status::NotFound)),
    };

    let connection = establish_connection();
    match documents.find(id).first::<Document>(&connection) {
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
