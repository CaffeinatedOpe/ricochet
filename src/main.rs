use actix_web::{ get, web, App, HttpServer, Result, web::Redirect, Responder };
use lazy_static::lazy_static;
use toml::{ Table, Value };
use std::{ collections::HashMap, path };
use serde::{ Deserialize };
use std::fs;
use std::io::{ self, Write };

static config_path: &str = "/config/config.toml";

#[derive(Deserialize, Debug)]
struct Config {
	paths: Table,
	#[serde(default)]
	behaviors: Behaviors,
}

#[derive(Deserialize, Debug)]
struct Behaviors {
	default_page: String,
}
impl Default for Behaviors {
	fn default() -> Self {
		Behaviors { default_page: "https://github.com/CaffeinatedOpe/ricochet".to_string() }
	}
}

lazy_static! {
	static ref CONFIG: Config = {
		let config_str = std::fs::read_to_string(config_path.to_string()).expect("invalid config");
		let config: Config = toml::from_str(&config_str).expect("invalid config");
		config
	};
	static ref PATHS: Table = {
		let config_str = std::fs::read_to_string(config_path.to_string()).expect("invalid config");
		let config: Config = toml::from_str(&config_str).expect("invalid config");
		config.paths
	};
}

#[get("/{key}")]
async fn handler(path: web::Path<String>) -> impl Responder {
	let key = path.into_inner();
	if PATHS.contains_key(&key) {
		Redirect::to(PATHS[&key].as_str().unwrap()).permanent()
	}
	else{
		Redirect::to(CONFIG.behaviors.default_page.as_str()).permanent()
	}
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let config_str = std::fs::read_to_string(config_path.to_string()).expect("invalid config");
	let config: Config = toml::from_str(&config_str).expect("invalid config");

	HttpServer::new(|| { App::new().service(handler).service(web::redirect("/", CONFIG.behaviors.default_page.as_str())) })
		.bind(("0.0.0.0", 8081))?
		.run().await
}
