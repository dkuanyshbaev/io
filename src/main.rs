// ---------------------------------------
// IOracle main server
// ---------------------------------------
use futures::channel::mpsc;
use rocket::{
    form::Form,
    response::Redirect,
    tokio::time::{sleep, Duration},
    State,
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

pub enum ControllCommand {
    Rest,
    Read,
    Display(String),
}

#[get("/")]
async fn home(controll_channel: &State<mpsc::UnboundedSender<ControllCommand>>) -> Template {
    controll_channel.unbounded_send(ControllCommand::Rest);
    Template::render("home", rocket_dyn_templates::context! {})
}

#[post("/question", data = "<form_data>")]
async fn question(
    form_data: Form<FormData>,
    controll_channel: &State<mpsc::UnboundedSender<ControllCommand>>,
) -> Redirect {
    // ---------------------------------------
    // TODO: ???
    // ---------------------------------------
    let (hexagram, r_hexagram) = wires::read();
    controll_channel.unbounded_send(crate::ControllCommand::Read);
    // ---------------------------------------

    let new_answer = iching::Answer::new(form_data.question.to_owned(), hexagram, r_hexagram);
    let new_answer_id = new_answer.save();
    Redirect::to(format!("/answer/{}", new_answer_id))
}

#[get("/answer/<id>")]
async fn answer(
    id: u64,
    controll_channel: &State<mpsc::UnboundedSender<ControllCommand>>,
) -> Template {
    // ---------------------------------------
    // TODO: check result here
    // ---------------------------------------
    let answer = iching::Answer::get_by_id(id);
    // ---------------------------------------

    controll_channel.unbounded_send(crate::ControllCommand::Display(answer.hexagram));

    // wires::display(answer.hexagram, controll_channel.inner().to_owned()).await;
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
        mpsc::UnboundedSender<ControllCommand>,
        mpsc::UnboundedReceiver<ControllCommand>,
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
