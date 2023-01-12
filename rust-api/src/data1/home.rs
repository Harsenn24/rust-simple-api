use crate::data2::data3::file_3;

pub fn rumah() -> String {
    let file_from_folder_3: String = file_3::file3();
    let new_string: String = format!("{}", file_from_folder_3);
    new_string
}
