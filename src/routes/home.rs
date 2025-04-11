use rocket::Route;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "My program",
        description: "Welcome to my personal project showcase website",
    })
}

pub fn routes() -> Vec<Route> {
    routes![index]
}