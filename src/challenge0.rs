use poem::{handler, http::StatusCode, Response};

#[handler]
pub fn hello_bird() -> Response {
    Response::builder()
    .status(StatusCode::OK)
    .body("Hello, bird!")
}

#[handler]
pub fn seek_redirect() -> Response {
    Response::builder()
        .header("Location", "https://www.youtube.com/watch?v=9Gc4QTqslN4")
        .status(StatusCode::FOUND)
        .finish()
}