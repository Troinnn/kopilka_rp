use std::fs::File;
use std::io::Read;
use rustc_serialize;

pub trait Parseded {
    fn new() -> Self;
    fn parse_from_json<T: rustc_serialize::Decodable>(path: &str) -> T {
        let mut _conf_str: String = String::new();
        File::open(path).expect("Ошибка при открытии файла!")
            .read_to_string(&mut _conf_str).expect("Ошибка при чтении файла");
        rustc_serialize::json::decode(&*_conf_str).expect("Ошибка при парсинге!")
    }
}

#[derive(Debug, RustcDecodable)]
pub struct Config {
    pub url: String,
    pub port: i32,
    pub path: String,
    pub ip_check: bool,
    pub qiwi_ip_1: String,
    pub qiwi_ip_2: String,
    pub login: String,
    pub password: String,
}

impl Parseded for Config {
    fn new() -> Self {
        Config {
            url: String::new(),
            port: 0,
            path: String::new(),
            ip_check: false,
            qiwi_ip_1: String::new(),
            qiwi_ip_2: String::new(),
            login: String::new(),
            password: String::new(),
        }
    }
}

