extern crate rustc_serialize;
extern crate http;

mod config;

use config::{Parseded, Config};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use  http::{Request, Response};

fn main() {
    let _conf: Config = Config::parse_from_json("/home/asemenov/IdeaProjects/kopilka_rp/target/debug/config/conf.json");
    let listner: TcpListener = TcpListener::bind("localhost:8000").expect("Ошибка привязки сокета!");
    for stream in listner.incoming() {
        match stream {
            Ok(stream) => {
                handle(stream);
            },
            Err(_e) => { /* connection failed */ },
        }
    }
}

fn handle(mut stream: TcpStream) {
    let req = Request::new(&mut stream);
    println!("{:?}", req);

}