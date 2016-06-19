// Silence some clippy warnings:
// * warn(identity_op) because of #[derive(Serialize)]
#![allow(identity_op)]

use iron::headers::ContentType;
use iron::modifier::Modifier;
use iron::prelude::*;
use serde::Serialize;
use serde_json::to_vec;


pub struct SerializableResponse<T: Serialize>(pub T);

#[derive(Serialize)]
pub struct SerializableData<'a, T: 'a + Serialize> {
    data: &'a T,
}

impl<T> Modifier<Response> for SerializableResponse<T>
    where T: Serialize
{
    fn modify(self, response: &mut Response) {
        response.headers.set(ContentType::json());
        let data = SerializableData { data: &self.0 };
        to_vec(&data)
            .expect("Could not serialize response data")
            .modify(response);
    }
}
