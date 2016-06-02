use iron::prelude::*;
use iron::status;
use router::Router;

use get_pool_connection;
use models::*;
use diesel::prelude::*;
use diesel::result::Error::NotFound as DieselNotFound;
use std::num::ParseIntError;

pub fn jobs_handler(req: &mut Request) -> IronResult<Response> {
    use schema::jobs::dsl::jobs;

    let connection = get_pool_connection(req);
    // FIXME: Why is &* necessary? I thought Deref coercion was supposed to solve
    // that.
    let results = jobs.limit(5)
        .load::<Job>(&*connection)
        .expect("Error loading jobs");

    let mut response = format!("Displaying {} jobs", results.len());
    for job in results {
        response.push_str(&format!("{}", job.sha));
        response.push_str(&format!("----------\n"));
    }

    Ok(Response::with((status::Ok, response)))
}

pub fn job_handler(req: &mut Request) -> IronResult<Response> {
    use schema::jobs::dsl::jobs;

    let id = match req.extensions.get::<Router>().unwrap().find("id").unwrap().parse::<i32>() {
        Ok(id) => id,
        Err(ParseIntError { .. }) => return Ok(Response::with(status::NotFound)),
    };

    let connection = get_pool_connection(req);
    // FIXME: Why is &* necessary? I thought Deref coercion was supposed to solve
    // that.
    match jobs.find(id).first::<Job>(&*connection) {
        Ok(job) => {
            let mut response = format!("{}", job.sha);
            response.push_str(&format!("----------\n"));
            Ok(Response::with((status::Ok, response)))
        }
        Err(DieselNotFound) => Ok(Response::with(status::NotFound)),
        Err(err) => panic!(err),
    }
}

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
        response.push_str(&format!("{}", document.sha));
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
            let mut response = format!("{}", document.sha);
            response.push_str(&format!("----------\n"));
            response.push_str(&format!("{}", document.tei));
            Ok(Response::with((status::Ok, response)))
        }
        Err(DieselNotFound) => Ok(Response::with(status::NotFound)),
        Err(err) => panic!(err),
    }
}
