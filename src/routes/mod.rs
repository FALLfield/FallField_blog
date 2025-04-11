mod home;
mod projects;
mod admin;
mod blogs;

pub fn routes() -> Vec<rocket::Route> {
    vec![
        home::routes(),
        projects::routes(),
        admin::routes(),
        blogs::routes(),
    ].concat()
}

