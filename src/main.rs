use std::{ sync::Mutex };
use actix_web::{ get, App, HttpServer, web::Redirect, Responder, web::{ self, Data } };
use toml::{ Table, Value };
use serde::{ Deserialize };
use std::fs;
use std::io::{ self };
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
	let passed_config = data.lock().unwrap();
	let mut output_path = passed_config.behaviors.default_page.clone();
	if passed_config.paths.contains_key(&key) {
		output_path = passed_config.paths[&key].as_str().unwrap().to_string();
	}
	println!("Redirecting to: {output_path}");
	Redirect::to(output_path).permanent()
}

#[derive(PartialEq)]
enum ArgAction {
	NONE,
	ConfigPath,
	SetCustomDefault,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	println!("Initializing...");

	let config_str: String;
	let config: Config;

	let mut config_map_enable = env::var("CONFIGMAP").is_ok();
	let mut config_path = env::var("CONFIG_PATH").unwrap_or("/config/config.toml".to_string());
	let mut custom_default = env::var("CUSTOM_DEFAULT").unwrap_or("".to_string());

	let args: Vec<String> = env::args().collect();
	let mut current_arg_action: ArgAction = ArgAction::NONE;

	for entry in args.clone() {
		if current_arg_action == ArgAction::NONE {
			match entry.as_str() {
				"-c" => {
					current_arg_action = ArgAction::ConfigPath;
				}
				"-C" => {
					config_map_enable = true;
				}
				"-d" => {
					current_arg_action = ArgAction::SetCustomDefault;
				}
				&_ => (),
			}
		} else {
			match current_arg_action {
				ArgAction::ConfigPath => {
					config_path = entry;
				}
				ArgAction::SetCustomDefault => {
					custom_default = entry;
				}
				ArgAction::NONE => (),
			}
		}
	}

	if config_map_enable {
		let mut paths = Table::new();
		let entries = fs
			::read_dir("/config")?
			.map(|res| res.map(|e| e.path()))
			.collect::<Result<Vec<_>, io::Error>>()?;
		println!("configs found: {:?}", entries);
		for x in entries {
			let key = x.file_name().unwrap().to_str().unwrap().to_string();
			let val: Value = Value::String(std::fs::read_to_string(&x).expect("error reading file {x}"));
			paths.insert(key, val);
		}
		config = Config {
			paths,
			behaviors: Behaviors { default_page: "".to_string() },
		};
	} else {
		config_str = std::fs::read_to_string(config_path).expect("invalid config location");
		config = toml::from_str(&config_str).expect("invalid config");
	}

	let data = Data::new(Mutex::new(config.clone()));

	if custom_default == "".to_string() {
		custom_default = config.behaviors.default_page;
	}

	println!("Running");

	HttpServer::new(move || {
		App::new()
			.app_data(Data::clone(&data))
			.service(handler)
			.service(web::redirect("/", custom_default.clone()))
	})
		.bind(("0.0.0.0", 8081))?
		.run().await
}
