#[macro_use]
extern crate problem_derive;
extern crate problem;
extern crate serde;
extern crate serde_json;

use problem::{Problem, ToProblem, ProblemBuilder};


#[derive(ToProblem)]
pub enum DBError {
    TestError,
    OpenError,
    CloseError,
    #[problem(title="Fail error", detail="Test")]
    FailError
}

fn main() {
    let p = Problem::from(DBError::FailError);
    let j = serde_json::to_string(&p).unwrap();
    println!("{}", j);
}
