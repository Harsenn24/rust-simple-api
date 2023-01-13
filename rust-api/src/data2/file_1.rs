use serde::{Deserialize, Serialize};

pub fn file1() -> String {
    "ini dari dalam folder data 2".to_string()
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserData<T> {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub anything: T,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Params {
    pub number: Option<i32>,
    pub letter: Option<String>,
}
