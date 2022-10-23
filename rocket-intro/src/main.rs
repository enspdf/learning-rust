#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "Rocket Overview"
    })
}

#[get("/about")]
fn about() -> &'static str {
    "About"
}

#[get("/profile")]
fn profile() -> &'static str {
    "Profile"
}

#[post("/profile")]
fn create_profile() -> &'static str {
    "Profile"
}

#[put("/profile")]
fn update_profile() -> &'static str {
    "Profile"
}

#[delete("/profile")]
fn delete_profile() -> &'static str {
    "Profile"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Template::fairing())
    .mount("/", routes![index, about])
    .mount("/profile", routes![profile, create_profile, update_profile, delete_profile])
}