mod db;
mod crypto;

use serde::{Deserialize, Serialize};
// use std::error;
// use thiserror::Error;

use actix_web::{middleware, web, App, HttpResponse, HttpServer, Result};
// use actix_web::{web, App, HttpServer};
use std::env;

// use crate::db;

// struct AppState {
//     foo: String,
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
			// Box<dyn std::error::Error>> {
//		std::io::Result<()>> {
    dotenv::dotenv().ok(); // use Result (somehow)
    init().ok();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(app_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            // .data(AppState {
            //     foo: "bar".to_string(),
            // })
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/signup").route(web::post().to(handle_signup)))
            .service(web::resource("/signin").route(web::post().to(handle_signin))),
    );
}

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html")))
}

#[derive(Serialize, Deserialize)]
pub struct MyParams {
    login: String,
		password: String,
		name: String,
		surname: String,
		birthday: String,
		city: String,
		hobby: String
}

/// Simple handle POST request
async fn handle_signup(params: web::Form<MyParams>) -> Result<HttpResponse> {
		let hashed_password = crypto::mk_hash(&params.password);
		let ret = db::add_user(&params.login,
													 &hashed_password,
													 &params.name,
													 &params.surname,
													 &params.birthday,
													 &params.city,
													 &params.hobby
		).unwrap();
    Ok(HttpResponse::Ok().content_type("text/plain").body(format!(
        "Your name is {:?}, password is {} and your are a new user, add returned {}",
        params.name, params.password, ret
    )))
}

async fn handle_signin(params: web::Form<MyParams>) -> Result<HttpResponse> {
		let hashed_password = crypto::mk_hash(&params.password);
		let ret = db::check_user(&params.login, &hashed_password);
		if ret>0 {
				Ok(HttpResponse::Ok().content_type("text/plain").body(format!(
						"Your name is {}, password is {} and your are a known user, check returned {}",
						params.name, params.password, ret
				)))
		}
		else {
						Ok(HttpResponse::NotFound().content_type("text/plain").body(format!(
								"User not found or password not matched",
						)))
		}
}

fn init() -> Result<(), fern::InitError> {
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "DEBUG".into());
    let log_level = log_level.parse().unwrap_or(log::LevelFilter::Info);

    let mut builder = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stderr());

    if let Ok(log_file) = env::var("LOG_FILE") {
        let log_file = std::fs::File::create(log_file)?;
        builder = builder.chain(log_file);
    }

    builder.apply()?;

    log::trace!("TRACE output enabled");
    log::debug!("DEBUG output enabled");
    log::info!("INFO output enabled");
    log::warn!("WARN output enabled");
    log::error!("ERROR output enabled");

    Ok(())
}
