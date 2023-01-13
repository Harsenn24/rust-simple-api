use crate::data2::file_1::{Params, UserData};

mod data1;
mod data2;

fn main() {
    let result_data1 = data1::home::rumah();
    let result_data2 = data2::file_1::file1();

    println!("{}", result_data1);

    println!("{}", result_data2);

    let paramsku = Params {
        number: Some(1),
        letter: Some("namaku adalah".to_string()),
    };

    let user = UserData::<Params> {
        id: 2,
        name: "harsenn".to_string(),
        age: 28,
        anything: paramsku,
    };

    println!("{:?}", user)
}
