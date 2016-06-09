// Silence some clippy warnings:
// * warn(clone_on_copy) because of #[belongs_to(document)]
// * warn(identity_op) because of #[derive(Serialize)]
#![allow(clone_on_copy, identity_op)]

use diesel::prelude::*;
use serde::{Serializer, Deserializer};
use std::marker::Sized;

use schema::{documents, jobs};


trait SerializeWith {
    fn serialize_with<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error>;
}

trait DeserializeWith {
    fn deserialize_with<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error>
        where Self: Sized;
}

impl SerializeWith for i32 {
    fn serialize_with<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        serializer.serialize_str(&format!("{}", self))
    }
}

#[derive(Queryable, Serialize)]
#[has_many(jobs)]
pub struct Document {
    #[serde(serialize_with="SerializeWith::serialize_with")]
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
    #[serde(serialize_with="SerializeWith::serialize_with")]
    pub id: i32,
    pub sha: String,
    pub document_id: Option<i32>,
}

#[insertable_into(jobs)]
pub struct NewJob<'a> {
    pub sha: &'a str,
}
