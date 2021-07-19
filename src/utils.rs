extern crate rustc_serialize;
use rustc_serialize::json::Json;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl <T, E> Result<T, E>  {
    fn unwrap(self) -> T{
        match self {
            Result::Ok(v) => return v,
            Result::Err(e) => panic!("Unwrap error {}"),
        }
    }
}

pub fn check_file_exists(a: &str) -> bool {
    println!("Checking File Exists");
    fs::metadata(a).is_ok()
}

pub fn create_file(a: &str) -> Result<String, String> {
    println!("Creating File");
    if !check_file_exists(a) {
        let _file = File::create(a);
        return Result::Ok(String::from("Done"))
    } else {
        return Result::Err(String::from("AlreadyExists"))
    }
}

pub fn read_file(a: &str) -> Result<String, String> {
    println!("Reading File");
    if !check_file_exists(a) {
        return Result::Err(String::from("FileNotFound"))
    }else {
        let mut file = File::open(a).unwrap();
        let mut buf = String::new();
        let _res = file.read_to_string(&mut buf);
        match _res {
            Ok(_) => return Result::Ok(buf),
            Err(e) => return Result::Err(e.to_string()),
        }
    }
}

pub fn write_file(a: &str, b: &[u8]) -> Result<String, String> {
    println!("Writing File");
    if !check_file_exists(a) {
        return Result::Err(String::from("FileNotFound"))
    }else {
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(a).unwrap();
        let _writing = file.write_all(b).unwrap();
        return Result::Ok(String::from("Done"))
    }
}

pub fn read_json(a: &str) -> Json {
    let a = read_file(a).unwrap();
    let b = Box::leak(a.into_boxed_str());

    let json = Json::from_str(b).unwrap();
    json
}
