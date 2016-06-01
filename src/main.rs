extern crate infuse;

use infuse::Infuse;

fn main() {
    Infuse::new().serve().unwrap();
}
