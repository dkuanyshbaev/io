// ---------------------------------------
// IOracle hardware controll
// ---------------------------------------
use futures::channel::mpsc;
use rocket::tokio::time::{sleep, Duration};

pub async fn go(mut receiver: mpsc::UnboundedReceiver<crate::ControllCommand>) {
    println!("go!");

    loop {
        sleep(Duration::from_secs(2)).await;
        match receiver.try_next() {
            // message is fetched
            Ok(Some(t)) => match t {
                crate::ControllCommand::Rest => println!("Resting -------------------------"),
                crate::ControllCommand::Read => println!("Reading -------------------------"),
                crate::ControllCommand::Display(h) => {
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

pub async fn rest(sender: mpsc::UnboundedSender<crate::ControllCommand>) {
    println!("resting");

    // rocket::tokio::spawn(async move {
    //     loop {
    //         sleep(Duration::from_secs(2)).await;
    //         sender.unbounded_send(crate::Command::Rest);
    //     }
    // });

    // loop {
    //     sleep(Duration::from_secs(2)).await;
    //     sender.unbounded_send("test".to_string());
    // }

    // ---------------------------------------
    // TODO: leds
    // ---------------------------------------
}

pub fn read() -> (String, String) {
    println!("reading");

    // ---------------------------------------
    // TODO: alot
    // ---------------------------------------

    ("111000".to_string(), "000111".to_string())
}

pub async fn display(hexagram: String, sender: mpsc::UnboundedSender<crate::ControllCommand>) {
    println!("displaing");

    rocket::tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(2)).await;
            sender.unbounded_send(crate::ControllCommand::Display(hexagram.clone()));
        }
    });

    // ---------------------------------------
    // TODO: leds
    // TODO: sound
    // TODO: pump
    // TODO: fan
    // ---------------------------------------
}
