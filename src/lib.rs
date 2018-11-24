extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate derive_builder;

use serde_json;

#[derive(Builder, Debug, Serialize, Deserialize, Clone, Default)]
#[builder(public, default)]
pub struct Problem {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}

pub trait ToProblem {
    fn to_problem(&self) -> Problem;
}

impl<T> From<T> for Problem
where
    T: ToProblem,
{
    fn from(t: T) -> Problem {
        t.to_problem()
    }
}

impl Problem {
    pub fn new(title: &str) -> Problem {
        ProblemBuilder::default()
            .title(title.to_string())
            .build()
            .unwrap()
    }
}

#[cfg(feature = "rocket_responder")]
impl<'r> rocket::response::Responder<'r> for Problem {
    fn respond_to(self, _request: &rocket::Request) -> Result<rocket::Response<'r>, rocket::http::Status> {
        use rocket::http::ContentType;
        use rocket::http::Status;
        use rocket::Response;
        use std::io::Cursor;

        let response = Response::build()
            .status(self.status.and_then(Status::from_code).unwrap_or(Status::InternalServerError))
            .sized_body(Cursor::new(serde_json::to_vec(&self).unwrap()))
            .header(ContentType::new("application", "json"))
            .finalize();

        Ok(response)
    }
}
