#![feature(plugin)]
#![plugin(clippy)]

extern crate infuse;

use infuse::Infuse;

fn main() {
    Infuse::default().serve().unwrap();
}
