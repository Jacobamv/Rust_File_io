mod utils;

use utils::*;


fn main(){
    create_file("data.txt");
    write_file("data.txt", b"Hello, World!");
    let a = read_file("data.txt");
    match a {
        Result::Ok(b) => println!("{}", b),
        Result::Err(e) => println!("{}", e)
    }
}