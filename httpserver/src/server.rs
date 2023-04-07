use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a>{
    socket_addr:&'a str,
}

impl<'a> Server<'a>{
    pub fn new(socket_addr:&'a str) -> Server<'a>{
        Server{socket_addr:socket_addr}
    }

    pub fn run(&self){
        let listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);

        for stream in listener.incoming(){
            let mut stream=stream.unwrap();
            println!("Connection established");

            let mut buffer=[0;200];
            stream.read(&mut buffer).unwrap();
            //因为为String实现了HttpRequest的From trait，所以可以直接使用into()方法
            let req:HttpRequest = String::from_utf8(buffer.to_vec()).unwrap().into();
            Router::router(req, &mut stream);
        }
    }
}
