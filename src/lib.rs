extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub fn new_named(title: &str) -> Problem {
        Problem::new(title, None, None, None, None)
    }

    pub fn new(title: &str,
        status: Option<u16>,
        type_url: Option<&str>,
        detail: Option<&str>,
        instance: Option<&str>,
    ) -> Problem {
        Problem {
            title: title.to_string(),
            status,
            type_url: type_url.map(String::from),
            detail: detail.map(String::from),
            instance: instance.map(String::from)
        }
    }
}
