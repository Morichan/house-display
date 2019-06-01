#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate house_display;
pub use house_display::timetable::api_agent::ApiAgent;

#[get("/")]
fn index() -> String {
    let mut api = ApiAgent::new();
    api.search_train_time();

    let times: Vec<String> = api.train_times.iter().map(
        |t| format!("{} -> {}", t.from, t.to)).collect::<Vec<_>>();

    times.join("\n")
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
