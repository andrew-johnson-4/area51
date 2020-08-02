#![feature(decl_macro)]
#[macro_use] extern crate rocket;
use rdxl::xhtml;

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
   xhtml!(Hello, {{age}} year old named {{name}})
}

fn main() {
   rocket::ignite().mount("/", routes![hello]).launch();
}
