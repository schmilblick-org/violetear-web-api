use actix_web::{get};

#[get("/")]
fn index() -> &'static str {
    "You have reached a Voletear Web API."
}
