use crypto::sha3::Sha3;
use crypto::digest::Digest;
use diesel;
use diesel::prelude::*;
use diesel::result::Error::NotFound as DieselNotFound;
use iron::headers::Location;
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status;
use params::Value;
use std::io::Read;
use std::num::ParseIntError;

use files::save_file;
use models::{Document, Job, NewJob};
use processing::Message;
use serializers::SerializableResponse;
use utils::{get_pool_connection, get_router_param, get_processor_sender, get_param};


pub fn jobs_handler(req: &mut Request) -> IronResult<Response> {
    use schema::jobs;

    let connection = get_pool_connection(req);
    let results = jobs::table
        .load::<Job>(&*connection)
        .expect("Error loading jobs");

    Ok(Response::with((status::Ok, SerializableResponse(results))))
}

pub fn job_handler(req: &mut Request) -> IronResult<Response> {
    use schema::jobs;

    let id = match get_router_param(req, "id").parse::<i32>() {
        Ok(id) => id,
        Err(ParseIntError { .. }) => return Ok(Response::with(status::NotFound)),
    };

    let connection = get_pool_connection(req);
    match jobs::table.find(id).first::<Job>(&*connection) {
        Ok(job) => {
            if let Some(document_id) = job.document_id {
                Ok(Response::with((status::SeeOther,
                                   Header(Location(format!("/documents/{}", document_id))))))
            } else {
                Ok(Response::with((status::Ok, SerializableResponse(job))))
            }
        }
        Err(DieselNotFound) => Ok(Response::with(status::NotFound)),
        Err(err) => panic!(err),
    }
}

pub fn documents_handler(req: &mut Request) -> IronResult<Response> {
    use schema::documents;

    let connection = get_pool_connection(req);
    let results = documents::table
        .load::<Document>(&*connection)
        .expect("Error loading documents");

    Ok(Response::with((status::Ok, SerializableResponse(results))))
}

pub fn document_handler(req: &mut Request) -> IronResult<Response> {
    use schema::documents;

    let id = match get_router_param(req, "id").parse::<i32>() {
        Ok(id) => id,
        Err(ParseIntError { .. }) => return Ok(Response::with(status::NotFound)),
    };

    let connection = get_pool_connection(req);
    match documents::table.find(id).first::<Document>(&*connection) {
        Ok(document) => Ok(Response::with((status::Ok, SerializableResponse(document)))),
        Err(DieselNotFound) => Ok(Response::with(status::NotFound)),
        Err(err) => panic!(err),
    }
}

pub fn new_document_handler(req: &mut Request) -> IronResult<Response> {
    use schema::jobs;

    // Extract file contents.
    let mut bytes;
    if let Some(&Value::File(ref pfile)) = get_param(req, &["file"]) {
        // Read the file.
        let mut file = pfile.open().expect("Couldn't open file");
        let size = file.metadata().expect("Couldn't get file metadata").len();
        bytes = Vec::with_capacity(size as usize);
        file.read_to_end(&mut bytes).unwrap();
    } else {
        return Ok(Response::with(status::BadRequest))
    }

    // Compute the file's sha.
    let mut hasher = Sha3::sha3_256();
    hasher.input(&bytes);
    let hash = hasher.result_str();

    // See if we don't have the file already.
    let connection = get_pool_connection(req);
    match jobs::table
        .filter(jobs::columns::sha.eq(&hash))
        .first::<Job>(&*connection) {
        Ok(job) => return Ok(Response::with((status::Conflict,
                                             Header(Location(format!("/jobs/{}", job.id)))))),
        Err(DieselNotFound) => (),
        Err(err) => panic!(err),
    }

    // Store the file.
    save_file(&hash, &bytes);

    // Create the job.
    let new_job = NewJob {
        sha: &hash,
    };
    let job = diesel::insert(&new_job).into(jobs::table)
        .get_result::<Job>(&*connection)
        .expect("Error saving new job");
    let processor_sender = get_processor_sender(req);
    processor_sender.send(Message::new_job(job.id)).unwrap();

    Ok(Response::with((status::Accepted,
                       Header(Location(format!("/jobs/{}", job.id))))))
}
