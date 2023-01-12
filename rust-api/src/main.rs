mod data1;
mod data2;

fn main() {
    let result_data1 = data1::home::rumah();
    let result_data2= data2::file_1::file1();

    println!("{}", result_data1);

    println!("{}", result_data2)
}