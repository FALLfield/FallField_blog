use rocket_dyn_templates::{Template, context};

#[get("/admin")]
fn index() -> Template {
    Template::render("index", context! {
        title: "Administrative Panel",
        description: "Administration",
    })
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index]
}