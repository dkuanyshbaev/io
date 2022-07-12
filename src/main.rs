// ---------------------------------------
// IOracle server
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
fn home(command_sender: &State<mpsc::UnboundedSender<wires::Command>>) -> Template {
    wires::rest(command_sender.inner().to_owned());
    Template::render("home", rocket_dyn_templates::context! {})
}

#[post("/question", data = "<form_data>")]
async fn question(
    form_data: Form<FormData>,
    command_sender: &State<mpsc::UnboundedSender<wires::Command>>,
) -> Redirect {
    let (hexagram, r_hexagram) = wires::read(command_sender.inner().to_owned()).await;
    let new_answer = iching::Answer::new(form_data.question.to_owned(), hexagram, r_hexagram);
    let new_answer_id = new_answer.save();
    Redirect::to(format!("/answer/{}", new_answer_id))
}

#[get("/answer/<id>")]
fn answer(id: u64, command_sender: &State<mpsc::UnboundedSender<wires::Command>>) -> Template {
    let answer = iching::Answer::get_by_id(id);
    wires::display(command_sender.inner().to_owned(), answer.hexagram);
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
    let (command_sender, command_receiver): (
        mpsc::UnboundedSender<wires::Command>,
        mpsc::UnboundedReceiver<wires::Command>,
    ) = mpsc::unbounded();

    rocket::tokio::spawn(async move {
        wires::hardware_controll(command_receiver).await;
    });

    rocket::build()
        .mount("/", routes![home, question, answer])
        .register("/", catchers![not_found, internal_error])
        .attach(Template::fairing())
        .manage(command_sender)
}
