use iron::prelude::*;
use iron::status;

use establish_connection;
use models::*;
use diesel::prelude::*;

pub fn handler(_: &mut Request) -> IronResult<Response> {
    use schema::documents::dsl::*;

    let connection = establish_connection();
    let results = documents.limit(5)
        .load::<Document>(&connection)
        .expect("Error loading documents");

    let mut response = String::new();
    response.push_str(&format!("Displaying {} documents", results.len()));
    for document in results {
        response.push_str(&format!("{}", document.hash));
        response.push_str(&format!("----------\n"));
        response.push_str(&format!("{}", document.tei));
    }

    Ok(Response::with((status::Ok, response)))
}
