use crate::data2::data3::file_3;
use serde::{Deserialize, Serialize};

pub fn rumah() -> String {
    let file_from_folder_3: String = file_3::file3();
    let new_string: String = format!("{}", file_from_folder_3);
    new_string
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    pub name: String,
    pub year_born: i32,
    pub phones: Vec<String>,
}
 