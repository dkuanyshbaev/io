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
    //----------------------------------------------
    // TODO: get answer by id
    //----------------------------------------------

    Template::render(
        "answer",
        rocket_dyn_templates::context! {
            answer: id,
        },
    )
}

#[post("/question", data = "<form_data>")]
async fn question(form_data: Form<FormData>) -> Redirect {
    //----------------------------------------------
    // TODO: process the question
    //----------------------------------------------
    println!("question: {}", form_data.question);
    sleep(Duration::from_secs(20)).await;

    // // get reading
    // let (hexagram, related) = wires::reading();
    // // create new answer
    // let new_answer = Answer::new(&data.email, &data.question, &hexagram, &related);
    // let answer_id = new_answer.id.clone();
    // // save answer
    // Answer::insert(&connection, &new_answer)?;
    // // send an email with the answer
    // Answer::send(
    //     &data.email,
    //     &data.question,
    //     &config.username,
    //     &config.password,
    //     &iching::full_answer(&connection, new_answer)?,
    // );
    //----------------------------------------------

    Redirect::to(format!("/answer/{}", 42))
}

#[launch]
fn rocket() -> _ {
    // TODO: run periferal threads

    rocket::build()
        .mount("/", routes![home, question, answer])
        .attach(Template::fairing())
}
