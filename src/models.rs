// Silence some clippy warnings:
// * warn(clone_on_copy) because of #[belongs_to(document)]
// * warn(identity_op) because of #[derive(Serialize)]
#![allow(clone_on_copy, identity_op)]

use diesel::prelude::*;

use schema::{documents, jobs};


#[derive(Queryable, Serialize)]
#[has_many(jobs)]
pub struct Document {
    pub id: i32,
    pub tei: String,
}

#[insertable_into(documents)]
pub struct NewDocument<'a> {
    pub tei: &'a str,
}

#[derive(Queryable, Serialize)]
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
