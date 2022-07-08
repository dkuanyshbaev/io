// ---------------------------------------
// IOracle hardware controll
// ---------------------------------------
use futures::channel::mpsc;
use rocket::tokio::time::{sleep, Duration};

pub enum ControllCommand {
    Rest,
    Read,
    Display(String),
}

pub async fn go(mut receiver: mpsc::UnboundedReceiver<ControllCommand>) {
    println!("go!");

    loop {
        sleep(Duration::from_secs(2)).await;
        match receiver.try_next() {
            // message is fetched
            Ok(Some(t)) => match t {
                ControllCommand::Rest => println!("Resting -------------------------"),
                ControllCommand::Read => println!("Reading -------------------------"),
                ControllCommand::Display(h) => {
                    println!("Displaing ------------------------- {}", h)
                }
            },
            // channel is closed and no messages left in the queue
            Ok(None) => {
                println!("None");
            }
            // there are no messages available, but channel is not yet closed
            Err(e) => {
                println!("e: {}", e);
            }
        }
    }
}

pub async fn read(controll_channel: mpsc::UnboundedSender<ControllCommand>) -> (String, String) {
    let _ = controll_channel.unbounded_send(ControllCommand::Read);
    // ---------------------------------------
    // TODO: ???
    // ---------------------------------------

    // ---------------------------------------
    // TODO: alot
    // ---------------------------------------

    ("111000".to_string(), "000111".to_string())
}
