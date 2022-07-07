// use log::{debug, error, info, trace, warn};
use log::{error, info, warn};
use std::collections::HashMap;
use warp::{http::Uri, Filter};

pub mod templates;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("Starting IOracle");

    let home = warp::path::end().map(move || warp::reply::html(templates::HOME));
    let answer = warp::path!("answer" / u32).map(|_a| warp::reply::html(templates::ANSWER));
    let question = warp::path!("question")
        .and(warp::post())
        // Only accept bodies smaller than 64kb...
        .and(warp::body::content_length_limit(1024 * 64))
        .and(warp::body::form())
        .map(|simple_map: HashMap<String, String>| {
            info!("question: {}", simple_map.get("question").unwrap());
            warp::redirect(Uri::from_static("/answer/42"))
        });

    warp::serve(home.or(answer).or(question))
        .run(([127, 0, 0, 1], 4444))
        .await;
}
