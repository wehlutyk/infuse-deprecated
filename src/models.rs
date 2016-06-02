use schema::{documents, jobs};
use diesel::prelude::*;

#[derive(Queryable)]
#[has_many(jobs)]
pub struct Document {
    pub id: i32,
    pub sha: String,
    pub tei: String,
}

#[insertable_into(documents)]
pub struct NewDocument<'a> {
    pub sha: &'a str,
    pub tei: &'a str,
}

#[derive(Queryable)]
#[belongs_to(document)]
pub struct Job {
    pub id: i32,
    pub sha: String,
    pub document_id: Option<i32>,
}

#[insertable_into(jobs)]
pub struct NewJob<'a> {
    pub sha: &'a str,
}
