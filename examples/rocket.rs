#![feature(proc_macro_hygiene, decl_macro)]
#![feature(uniform_paths)]
#[macro_use]
extern crate problem_derive;
extern crate problem;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

use problem::{Problem, ToProblem, ProblemBuilder};
use rocket_contrib::{json::{Json, JsonValue}};

#[derive(Debug, ToProblem)]
pub enum AppError {
    InvalidUser,
    WrongPassword,
    ResourceNotFound,
    #[problem(title="Fail error", detail="Test")]
    FailError
}

use AppError::*;

#[get("/test/<user>/<password>/<data_item>")]
fn test_route(user: String, password: String, data_item: String) -> Result<JsonValue, AppError> {
    if &user != "user" {
        return Err(InvalidUser);
    }
    if &password != "password" {
        return Err(WrongPassword);
    }
    if &data_item != "test" {
        return Err(ResourceNotFound);
    }
    Ok(json!({"value": "test value"}))
}

impl<'r> rocket::response::Responder<'r> for AppError {
    fn respond_to(self, request: &rocket::Request) -> Result<rocket::Response<'r>, rocket::http::Status> {
        let p: Problem = self.into();
        p.respond_to(request)
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![test_route])
        .launch();
}
