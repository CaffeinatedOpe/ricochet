use actix_web::{ get, web, App, HttpServer, Result, web::Redirect, Responder };
use lazy_static::lazy_static;
use toml::{Table, Value};
use std::{ collections::HashMap, path };
use serde::{ Deserialize, Serialize };
use std::fs;
use std::io::{ self, Write };

#[derive(Deserialize, Debug)]
struct Config {
	paths: Table,
}

lazy_static! {
		static ref PATHS: Table = {
			let config_str = std::fs::read_to_string("config.toml").expect("invalid config");
			let config: Config = toml::from_str(&config_str).expect("invalid config");
			config.paths
		};
}

#[get("/{key}")]
async fn handler(path: web::Path<String>) -> impl Responder {
	let key = path.into_inner();
	Redirect::to(PATHS[&key].as_str().unwrap()).permanent()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| { App::new().service(handler) })
		.bind(("127.0.0.1", 8081))?
		.run().await
}
