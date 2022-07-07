// use log::{debug, error, info, trace, warn};
use std::collections::HashMap;
use warp::{http::Uri, Filter};

#[tokio::main]
async fn main() {
    println!("starting ioracle");
    pretty_env_logger::init();
    // trace!("a trace example");
    // debug!("deboogging");
    // info!("such information");
    // warn!("o_O");
    // error!("boom");

    let home_body = r#"
    <html>
        <head>
            <title>IOracle</title>
        </head>
        <body>
            <h1>Welcome to IOracle!</h1>
            <form action="/question" method="post">
                <label for="question">Question:</label>
                <input id="question" type="text" name="question">
                <input type="submit" value="Ask">
            </form>
        </body>
    </html>
    "#;

    let home = warp::path::end().map(move || warp::reply::html(home_body));
    let answer = warp::path!("answer" / u32).map(|a| format!("answer num {}", a));
    let question = warp::path!("question")
        .and(warp::post())
        // Only accept bodies smaller than 64kb...
        .and(warp::body::content_length_limit(1024 * 64))
        .and(warp::body::form())
        .map(|simple_map: HashMap<String, String>| {
            println!("{:?}", simple_map.get("question").unwrap());
            warp::redirect(Uri::from_static("/answer/42"))
        });

    warp::serve(home.or(answer).or(question))
        .run(([127, 0, 0, 1], 4444))
        .await;
}
