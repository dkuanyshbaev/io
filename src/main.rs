// ---------------------------------------
// IOracle server
// ---------------------------------------
use futures::channel::mpsc;
use rocket::{form::Form, response::Redirect, State};
use rocket_db_pools::{sqlx, Connection, Database};
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

pub mod iching;
pub mod wires;

#[derive(FromForm)]
struct FormData {
    question: String,
}

#[derive(Database)]
#[database("ioracle")]
pub struct Db(sqlx::SqlitePool);

#[get("/")]
fn home(command_sender: &State<mpsc::UnboundedSender<wires::Command>>) -> Template {
    wires::rest(command_sender.inner().to_owned());
    Template::render("home", rocket_dyn_templates::context! {})
}

#[post("/question", data = "<form_data>")]
async fn question(
    form_data: Form<FormData>,
    command_sender: &State<mpsc::UnboundedSender<wires::Command>>,
    mut db: Connection<Db>,
) -> Redirect {
    let (hexagram, r_hexagram) = wires::read(command_sender.inner().to_owned()).await;
    let new_answer = iching::Answer::new(form_data.question.to_owned(), hexagram, r_hexagram);
    let new_answer_id = new_answer.save(db);
    Redirect::to(format!("/answer/{}", new_answer_id))
}

#[get("/answer/<id>")]
async fn answer(
    id: u32,
    command_sender: &State<mpsc::UnboundedSender<wires::Command>>,
    mut db: Connection<Db>,
) -> Template {
    let answer = iching::Answer::get_by_id(db, id);
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
        .attach(Db::init())
        .manage(command_sender)
}
