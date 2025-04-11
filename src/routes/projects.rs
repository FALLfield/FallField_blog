use rocket_dyn_templates::{Template, context};
use rocket::Route;

#[get("/projects")]
fn list() -> Template {
    let projects = vec![
        ("program1", "This is my first program"),
        ("program2", "This is my second program"),
        ("program3", "This is my third program"),
    ];

    Template::render("projects/list", context! {
        title: "Projects list",
        projects: projects,
    })
}

#[get("/projects/<id>")]
fn details(id: String) -> Template {
    // 模拟项目详情数据
    Template::render("projects/details", context! {
        title: format!("Project: {}", id),
        project_name: id,
        description: "This is the project description",
        tech_stack: "Rust, Rocket, SQLite",
        github_link: "https://github.com/example/project",
    })
}


pub fn routes() -> Vec<Route> {
    routes![list, details]
}