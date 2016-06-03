// Silence clippy's warn(clone_on_copy) because of #[belongs_to(document)]
#![allow(clone_on_copy)]

use schema::{documents, jobs};
use diesel::prelude::*;

#[derive(Queryable)]
#[has_many(jobs)]
pub struct Document {
    pub id: i32,
    pub tei: String,
}

#[insertable_into(documents)]
pub struct NewDocument<'a> {
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
