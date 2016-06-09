use crypto::sha3::Sha3;
use crypto::digest::Digest;
use diesel;
use diesel::prelude::*;
use diesel::result::Error::NotFound as DieselNotFound;
use iron::headers::Location;
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status;
use params::{Params, Value};
use std::io::Read;
use std::num::ParseIntError;

use files::save_file;
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

        if let Some(&Value::File(ref pfile)) = map.find(&["file"]) {
            // Read the file.
            let mut file = pfile.open().expect("Couldn't open file");
            let size = file.metadata().expect("Couldn't get file metadata").len();
            let mut bytes = Vec::with_capacity(size as usize);
            file.read_to_end(&mut bytes).unwrap();

            // Compute its sha.
            let mut hasher = Sha3::sha3_256();
            hasher.input(&bytes);
            let hash = hasher.result_str();

            // See if we don't have the file already.
            match jobs::table
                .filter(jobs::columns::sha.eq(&hash))
                .first::<Job>(&*connection) {
                Ok(job) => return Ok(Response::with((status::Conflict,
                                                     Header(Location(format!("/jobs/{}", job.id)))))),
                Err(DieselNotFound) => (),
                Err(err) => panic!(err),
            }

            // Store the file
            save_file(&hash, &bytes);

            // Create the job.
            let new_job = NewJob {
                sha: &hash,
            };
            let job = diesel::insert(&new_job).into(jobs::table)
                .get_result::<Job>(&*connection)
                .expect("Error saving new job");

            // TODO: actually launch the job

            Ok(Response::with((status::Accepted,
                               Header(Location(format!("/jobs/{}", job.id))))))
        } else {
            Ok(Response::with(status::BadRequest))
        }
    } else {
        Ok(Response::with(status::BadRequest))
    }
}
