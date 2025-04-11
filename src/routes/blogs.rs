use rocket_dyn_templates::{Template, context};
use rocket::Route;

#[get("/blogs")]
fn list() -> Template {
    let blogs = vec![
        ("blog1", "This is my first blog"),
        ("blog2", "This is my second blog"),
        ("blog3", "This is my third blog"),
    ];

    Template::render("blogs/list", context! {
        title: "Blogs list",
        blogs: blogs,
    })
}

#[get("/blogs/<id>")]
fn details(id: String) -> Template {
    // 模拟项目详情数据
    Template::render("blogs/details", context! {
        title: format!("Blog: {}", id),
        blog_name: id,
        description: "This is the blog description",
        tech_stack: "Rust, Rocket, SQLite",
    })
}


pub fn routes() -> Vec<Route> {
    routes![list, details]
}