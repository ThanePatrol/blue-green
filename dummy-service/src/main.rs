use axum::{response::Html, routing::get, Router};
use std::{thread, time::Duration};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<String> {
    let color = std::env::var("COLOR").unwrap_or("no COLOR env set".into());
    let string = format!("<h1>Hello, from {color}</h1>");
    Html(string)
}
