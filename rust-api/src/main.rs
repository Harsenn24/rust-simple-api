use crate::{data2::file_1::{Params, UserData}, data1::home::Person};
use serde_json::{json, to_value};

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

    println!("{:?}", user);

    let json_sample = json!(
        {
            "name" : "albert",
            "year_born" : 1995,
            "phones" : [
                "2190291",
                "182918"
            ]
        }
    );

    let json_to_string: Person = serde_json::from_value(json_sample).unwrap();

    println!("{:?}", json_to_string.year_born);

    let string_to_json = to_value(json_to_string).unwrap();

    println!("{:?}", string_to_json);

    let name = "dustin".to_string();
    let year_born = 1980;
    let phones = vec![
        "29189281".to_string(),
        "38928932".to_string()
    ];

    let new_object = Person{name, year_born, phones};

    println!("{:?}", new_object);

}
