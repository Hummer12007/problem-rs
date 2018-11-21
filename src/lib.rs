extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate derive_builder;


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
