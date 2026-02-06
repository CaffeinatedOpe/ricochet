use actix_web::{ get, web, App, HttpServer, Result, web::Redirect, Responder };
use lazy_static::lazy_static;
use std::{ collections::HashMap, path };

lazy_static! {
		static ref PATHS: HashMap<String, String> = {
			let mut map = HashMap::new();
			map.insert("blog".to_string(), "https://caffeinatedope.net/blog".to_string());
			map.insert("qr".to_string(), "https://caffeinatedope.net".to_string());
			map
		};
}

#[get("/{key}")]
async fn handler(path: web::Path<String>) -> impl Responder {
	let key = path.into_inner();
	Redirect::to(PATHS[&key].clone()).permanent()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	println!("{}", PATHS["blog"]);
	println!("{}", PATHS["qr"]);

	HttpServer::new(|| { App::new().service(handler) })
		.bind(("127.0.0.1", 8081))?
		.run().await
}
