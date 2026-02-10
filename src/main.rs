use std::sync::Mutex;
use actix_web::{ get, App, HttpServer, web::Redirect, Responder, web::{ self, Data } };
use toml::{ Table };
use serde::{ Deserialize };
use std::fs;
use std::io::{ self, Write };
use std::env;

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

#[derive(PartialEq)]
enum ArgAction {
	NONE,
	CONFIG_PATH,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	println!("Initializing...");
	let mut config_path: String = "/config/config.toml".to_string();
	let mut configMap_enable: bool = false;

	configMap_enable = env::var("CONFIGMAP").is_ok();
	config_path = env::var("CONFIG_PATH").unwrap_or(config_path);

	let args: Vec<String> = env::args().collect();
	let mut current_arg_action: ArgAction = ArgAction::NONE;

	for entry in args.clone() {
		if current_arg_action == ArgAction::NONE {
			if entry == "-c".to_string() {
				current_arg_action = ArgAction::CONFIG_PATH;
			}
		} else {
			if current_arg_action == ArgAction::CONFIG_PATH {
				config_path = entry;
			}
		}
	}

	let config_str = std::fs::read_to_string(config_path).expect("invalid config location");
	let config: Config = toml::from_str(&config_str).expect("invalid config");

	let data = Data::new(Mutex::new(config.clone()));
	
	println!("Running");

	HttpServer::new(move || {
		App::new()
			.app_data(Data::clone(&data))
			.service(handler)
			.service(web::redirect("/", config.behaviors.default_page.clone()))
	})
		.bind(("0.0.0.0", 8081))?
		.run().await
}
