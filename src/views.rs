use diesel;
use diesel::prelude::*;
use diesel::result::Error::NotFound as DieselNotFound;
use iron::headers::Location;
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status;
use params::Params;
use std::num::ParseIntError;

use models::{Document, Job, NewJob};
use serializers::SerializableResponse;
use utils::{get_pool_connection, get_router_param};


pub fn jobs_handler(req: &mut Request) -> IronResult<Response> {
    use schema::jobs::dsl::jobs;

    let connection = get_pool_connection(req);
    let results = jobs.limit(5)
        .load::<Job>(&*connection)
        .expect("Error loading jobs");

    Ok(Response::with((status::Ok, SerializableResponse(results))))
}

pub fn job_handler(req: &mut Request) -> IronResult<Response> {
    use schema::jobs::dsl::jobs;

    let id = match get_router_param(req, "id").parse::<i32>() {
        Ok(id) => id,
        Err(ParseIntError { .. }) => return Ok(Response::with(status::NotFound)),
    };

    let connection = get_pool_connection(req);
    match jobs.find(id).first::<Job>(&*connection) {
        Ok(job) => Ok(Response::with((status::Ok, SerializableResponse(job)))),
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

    Ok(Response::with((status::Ok, SerializableResponse(results))))
}

pub fn document_handler(req: &mut Request) -> IronResult<Response> {
    use schema::documents::dsl::documents;

    let id = match get_router_param(req, "id").parse::<i32>() {
        Ok(id) => id,
        Err(ParseIntError { .. }) => return Ok(Response::with(status::NotFound)),
    };

    let connection = get_pool_connection(req);
    match documents.find(id).first::<Document>(&*connection) {
        Ok(document) => Ok(Response::with((status::Ok, SerializableResponse(document)))),
        Err(DieselNotFound) => Ok(Response::with(status::NotFound)),
        Err(err) => panic!(err),
    }
}

pub fn new_document_handler(req: &mut Request) -> IronResult<Response> {
    use schema::jobs;

    let connection = get_pool_connection(req);
    if let Ok(map) = req.get_ref::<Params>() {

        let new_job = NewJob {
            sha: "bla",
        };

        let job = diesel::insert(&new_job).into(jobs::table)
            .get_result::<Job>(&*connection)
            .expect("Error saving new job");

        Ok(Response::with((status::Accepted,
                           Header(Location(format!("/jobs/{}", job.id))),
                           SerializableResponse(job))))
    } else {
        Ok(Response::with(status::BadRequest))
    }
}
