use std::sync::Mutex;
use actix_web::{ get, App, HttpServer, Result, web::Redirect, Responder, web::{self, Data}};
use lazy_static::lazy_static;
use toml::{ Table, Value };
use std::{ collections::HashMap, path };
use serde::{ Deserialize };
use std::fs;
use std::io::{ self, Write };

static CONFIG_PATH: &str = "/config/config.toml";

#[derive(Deserialize, Debug, Clone)]
struct Config {
	paths: Table,
	#[serde(default)]
	behaviors: Behaviors,
}

#[derive(Deserialize, Debug, Clone)]
struct Behaviors {
	default_page: String,
}
impl Default for Behaviors {
	fn default() -> Self {
		Behaviors { default_page: "https://github.com/CaffeinatedOpe/ricochet".to_string() }
	}
}

#[get("/{key}")]
async fn handler(path: web::Path<String>, data: Data<Mutex<Config>>) -> impl Responder {
	let key = path.into_inner();
	let mut passed_config = data.lock().unwrap();
	let mut output_path = passed_config.behaviors.default_page.clone();
	if passed_config.paths.contains_key(&key) {
		output_path = passed_config.paths[&key].as_str().unwrap().to_string();
	}
	Redirect::to(output_path).permanent()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let config_str = std::fs::read_to_string(CONFIG_PATH.to_string()).expect("invalid config");
	let config: Config = toml::from_str(&config_str).expect("invalid config");

	let data = Data::new(Mutex::new(config.clone()));

	HttpServer::new(move || { App::new()
		.app_data(Data::clone(&data))
		.service(handler)
		.service(web::redirect("/", config.behaviors.default_page.clone()))
	})
		.bind(("0.0.0.0", 8081))?
		.run().await
}
