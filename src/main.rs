// ---------------------------------------
// IOracle main server
// ---------------------------------------
use futures::channel::mpsc;
use rocket::{form::Form, response::Redirect, State};
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
async fn home(controll_channel: &State<mpsc::UnboundedSender<wires::ControllCommand>>) -> Template {
    let _ = controll_channel.unbounded_send(wires::ControllCommand::Rest);
    Template::render("home", rocket_dyn_templates::context! {})
}

#[post("/question", data = "<form_data>")]
async fn question(
    form_data: Form<FormData>,
    controll_channel: &State<mpsc::UnboundedSender<wires::ControllCommand>>,
) -> Redirect {
    let (hexagram, r_hexagram) = wires::read(controll_channel.inner().to_owned()).await;
    let new_answer = iching::Answer::new(form_data.question.to_owned(), hexagram, r_hexagram);
    let new_answer_id = new_answer.save();
    Redirect::to(format!("/answer/{}", new_answer_id))
}

#[get("/answer/<id>")]
async fn answer(
    id: u64,
    controll_channel: &State<mpsc::UnboundedSender<wires::ControllCommand>>,
) -> Template {
    let answer = iching::Answer::get_by_id(id);
    let _ = controll_channel.unbounded_send(wires::ControllCommand::Display(answer.hexagram));
    Template::render(
        "answer",
        rocket_dyn_templates::context! { answer: answer.answer },
    )
}

#[catch(404)]
pub fn not_found() -> Redirect {
    Redirect::to("/")
}

#[catch(500)]
pub fn internal_error() -> Redirect {
    Redirect::to("/")
}

#[launch]
fn rocket() -> _ {
    // ---------------------------------------
    let (sender, receiver): (
        mpsc::UnboundedSender<wires::ControllCommand>,
        mpsc::UnboundedReceiver<wires::ControllCommand>,
    ) = mpsc::unbounded();

    rocket::tokio::spawn(async move {
        wires::go(receiver).await;
    });
    // ---------------------------------------

    rocket::build()
        .mount("/", routes![home, question, answer])
        .register("/", catchers![not_found, internal_error])
        .attach(Template::fairing())
        .manage(sender)
}
