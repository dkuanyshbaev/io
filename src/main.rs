use rocket::{
    form::Form,
    response::Redirect,
    tokio::time::{sleep, Duration},
};
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

pub mod wires;

#[derive(FromForm)]
struct FormData {
    question: String,
}

#[get("/")]
fn home() -> Template {
    Template::render("home", rocket_dyn_templates::context! {})
}

#[get("/answer/<id>")]
fn answer(id: u64) -> Template {
    // TODO: get answer by id
    println!("answer id:{}", id);

    Template::render(
        "answer",
        rocket_dyn_templates::context! {
            answer: "42",
        },
    )
}

#[post("/question", data = "<form_data>")]
async fn question(form_data: Form<FormData>) -> Redirect {
    // TODO: process the question
    println!("question: {}", form_data.question);
    sleep(Duration::from_secs(20)).await;

    Redirect::to(format!("/answer/{}", 42))
}

#[launch]
fn rocket() -> _ {
    // TODO: run periferal threads

    rocket::build()
        .mount("/", routes![home, question, answer])
        .attach(Template::fairing())
}
