use iron::modifier::Modifier;
use iron::prelude::*;
use serde_json::ser::{to_vec};

use models::{Document, Job};


impl Modifier<Response> for Document {
    fn modify(self, res: &mut Response) {
        to_vec(&self).unwrap().modify(res);
    }
}

impl Modifier<Response> for Job {
    fn modify(self, res: &mut Response) {
        to_vec(&self).unwrap().modify(res);
    }
}
