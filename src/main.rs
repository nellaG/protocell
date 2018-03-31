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


#[get("/pages/<number>")]
fn pages(number: usize) -> String {
    format!("The page number is : {number}", number=number)
}


fn main() {
    rocket::ignite().mount("/", routes![index, root, pages]).launch();
}
