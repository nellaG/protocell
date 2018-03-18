#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::response::Redirect;


#[get("/")]
fn root() -> Redirect {
    Redirect::to("/index")
}


#[get("/index")]
fn index() -> &'static str {
    "Protocell"
}


fn main() {
    rocket::ignite().mount("/", routes![index, root]).launch();
}
