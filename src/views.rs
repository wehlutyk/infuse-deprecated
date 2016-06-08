use diesel::prelude::*;
use diesel::result::Error::NotFound as DieselNotFound;
use iron::headers::Location;
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status;
use params::Params;
use std::num::ParseIntError;

use models::{Document, Job};
use utils::{get_pool_connection, get_router_param};


pub fn jobs_handler(req: &mut Request) -> IronResult<Response> {
    use schema::jobs::dsl::jobs;

    let connection = get_pool_connection(req);
    let results = jobs.limit(5)
        .load::<Job>(&*connection)
        .expect("Error loading jobs");

    let mut response = format!("Displaying {} jobs", results.len());
    for job in results {
        response.push_str(&job.sha);
        response.push_str("\n----------\n");
    }

    Ok(Response::with((status::Ok, response)))
}

pub fn job_handler(req: &mut Request) -> IronResult<Response> {
    use schema::jobs::dsl::jobs;

    let id = match get_router_param(req, "id").parse::<i32>() {
        Ok(id) => id,
        Err(ParseIntError { .. }) => return Ok(Response::with(status::NotFound)),
    };

    let connection = get_pool_connection(req);
    match jobs.find(id).first::<Job>(&*connection) {
        Ok(job) => Ok(Response::with((status::Ok, job.sha + "\n----------"))),
        Err(DieselNotFound) => Ok(Response::with(status::NotFound)),
        Err(err) => panic!(err),
    }
}

pub fn documents_handler(req: &mut Request) -> IronResult<Response> {
    use schema::documents::dsl::documents;

    let connection = get_pool_connection(req);
    let results = documents.limit(5)
        .load::<Document>(&*connection)
        .expect("Error loading documents");

    let mut response = format!("Displaying {} documents", results.len());
    for document in results {
        response.push_str(&document.tei);
        response.push_str("\n----------\n");
    }

    Ok(Response::with((status::Ok, response)))
}

pub fn document_handler(req: &mut Request) -> IronResult<Response> {
    use schema::documents::dsl::documents;

    let id = match get_router_param(req, "id").parse::<i32>() {
        Ok(id) => id,
        Err(ParseIntError { .. }) => return Ok(Response::with(status::NotFound)),
    };

    let connection = get_pool_connection(req);
    match documents.find(id).first::<Document>(&*connection) {
        Ok(document) => Ok(Response::with((status::Ok, document.tei + "\n----------"))),
        Err(DieselNotFound) => Ok(Response::with(status::NotFound)),
        Err(err) => panic!(err),
    }
}

pub fn new_document_handler(req: &mut Request) -> IronResult<Response> {
    if let Ok(map) = req.get_ref::<Params>() {
        Ok(Response::with((status::Accepted,
                           Header(Location("/jobs/FIXME".to_string())),
                           map.keys().cloned().collect::<Vec<_>>().join("\n"))))
    } else {
        Ok(Response::with(status::BadRequest))
    }
}
