
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use inline_python::python;

fn main() {

    rocket::ignite().mount("/", routes![index]).launch();

}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

