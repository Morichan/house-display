#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate house_display;
pub use house_display::timetable::api_agent::ApiAgent;

#[get("/")]
fn index() -> String {
    let api = ApiAgent::new();
    return api.search_train_time()
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
