// ---------------------------------------
// IOracle server
// ---------------------------------------
use rocket::{
    form::Form,
    response::Redirect,
    tokio::time::{sleep, Duration},
};
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

pub mod iching;
pub mod wires;

#[derive(FromForm)]
struct FormData {
    question: String,
}

#[get("/")]
fn home() -> Template {
    wires::rest();
    Template::render("home", rocket_dyn_templates::context! {})
}

#[post("/question", data = "<form_data>")]
async fn question(form_data: Form<FormData>) -> Redirect {
    let (hexagram, r_hexagram) = wires::read();
    // TODO: real readings
    sleep(Duration::from_secs(10)).await;

    let new_answer = iching::Answer::new(form_data.question.to_owned(), hexagram, r_hexagram);
    let new_answer_id = new_answer.save();
    Redirect::to(format!("/answer/{}", new_answer_id))
}

#[get("/answer/<id>")]
fn answer(id: u64) -> Template {
    // TODO: check result here
    let answer = iching::Answer::get_by_id(id);
    wires::display(answer.hexagram);
    Template::render(
        "answer",
        rocket_dyn_templates::context! { answer: answer.answer },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home, question, answer])
        .attach(Template::fairing())
}
