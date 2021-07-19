mod utils;

use utils::*;


fn main(){
    let a = read_json("test.json");
    println!("{}", a["message"])
}