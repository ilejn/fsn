mod db;
mod crypto;

use serde::{Deserialize, Serialize};
// use std::error;
// use thiserror::Error;

use actix_web::{middleware, web, App, HttpResponse, HttpServer, Result};
use actix_session::{CookieSession};
use chrono::{NaiveDate, Utc};
use std::env;
use serde_json::json;
use tinytemplate::TinyTemplate;

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
        let mut tt = TinyTemplate::new();
        tt.add_template("../template/perspage.html", include_str!("../templates/perspage.html")).unwrap();
        App::new()
            .data(tt)
            .wrap(middleware::Logger::default())
						.wrap(CookieSession::signed(&[0; 32]).secure(false))
            .configure(app_config)
    })
    .bind("0.0.0.0:8080")?
    .run()
// .workers(3)
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
            .service(web::resource("/signin").route(web::post().to(handle_signin)))
            .service(web::resource("/subscribe").route(web::post().to(handle_subscribe)))
            .service(web::resource("/show_subscriptions").route(web::post().to(handle_show_subscriptions)))
            .service(web::resource("/lookup").route(web::post().to(handle_lookup)))
            .service(web::resource("/perspage").route(web::get().to(show_perspage)))
            .service(web::resource("/perspage").route(web::post().to(handle_perspage)))
    );
}

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html")))
}

#[derive(Serialize, Deserialize)]
pub struct UpParams {
    login: String,
		password: String,
		name: String,
		surname: String,
		birthday: String,
		city: String,
		hobby: String
}

#[derive(Serialize, Deserialize)]
pub struct InParams {
    login: String,
		password: String,
}


#[derive(Serialize, Deserialize)]
pub struct SubsParams {
		id: u32,
}


async fn handle_show_subscriptions(
		session: actix_session::Session) -> Result<HttpResponse> {

		let session_str: String;

		let s = session.get::<String>("session");
		match s {
				Ok(ss) => session_str = ss.unwrap_or_default(),
        Err(_error) => return Ok(HttpResponse::NotFound().content_type("text/plain").body(
						"User not authorized"))

		}

    log::trace!("session_str {}", session_str);

		let id_result = db::get_user_by_session(&session_str);
		match id_result {
        Err(_error) => return Ok(HttpResponse::NotFound().content_type("text/plain").body(
						"User not authorized")),
				Ok(id) => {
						let subs = db::get_subscriptions(id);

						match subs {
								Ok(ss) => Ok(HttpResponse::NotFound().content_type("text/plain").body(
										format!("Subscribed to {}", ss))),
								Err(_errpr) => Ok(HttpResponse::NotFound().content_type("text/plain").body(
										"User to subscribe not found or password not matched"))

						}

				},
		}
}


async fn handle_subscribe(
		session: actix_session::Session,
		params: web::Form<SubsParams>) -> Result<HttpResponse> {

		let session_str: String;

		let s = session.get::<String>("session");
		match s {
				Ok(ss) => session_str = ss.unwrap_or_default(),
        Err(_error) => return Ok(HttpResponse::NotFound().content_type("text/plain").body(
						"User not authorized"))

		}

    log::trace!("session_str {}", session_str);

		let id_result = db::get_user_by_session(&session_str);
		match id_result {
        Err(_error) => return Ok(HttpResponse::NotFound().content_type("text/plain").body(
						"User not authorized")),
				Ok(id) => {
						let subs = db::add_subscription(id, params.id)
								.and_then(|_s| {db::get_subscriptions(id)});

						match subs {
								Ok(ss) => Ok(HttpResponse::Ok().content_type("text/plain").body(
										format!("Subscribed to {}", ss))),
								Err(_errpr) => Ok(HttpResponse::NotFound().content_type("text/plain").body(
										"User to subscribe not found or password not matched"))

						}

				},
		}
}


#[derive(Serialize, Deserialize)]
pub struct LookupParams {
		name: String,
		surname: String,
}

async fn handle_lookup(params: web::Form<LookupParams>) -> Result<HttpResponse> {
    log::debug!("top of handle_lookup");
		log::debug!("{}, {}", &params.name, &params.surname);
		let ret = db::lookup_users(&params.name,
													 &params.surname
		);
		match ret {
				Ok(ss) => Ok(HttpResponse::Ok().content_type("text/plain").body(
						format!("Found users \n {}", ss))),
				Err(_errpr) => Ok(HttpResponse::NotFound().content_type("text/plain").body(
						"Lookup error"))
		}
}

async fn handle_signup(
		session: actix_session::Session,
		params: web::Form<UpParams>) -> Result<HttpResponse> {
		let hashed_password = crypto::mk_hash(&params.password);
		let session_str = crypto::mk_random_string();
		let birthday_date = NaiveDate::parse_from_str("&params.birthday", "%Y.%m.%d").unwrap_or(Utc::now().naive_utc().date());

		let ret = db::add_user(&params.login,
													 &hashed_password,
													 &params.name,
													 &params.surname,
													 // &params.birthday,
													 &birthday_date.format("%Y-%m-%d").to_string(),
													 &params.city,
													 &params.hobby,
													 &session_str
		).unwrap();
		session.set("session", session_str)?;
    Ok(HttpResponse::Ok().content_type("text/plain").body(format!(
        "Your name is {:?}, password is {} and your are a new user, add returned {}",
        params.name, params.password, ret
    )))
}

async fn handle_signin(
		session: actix_session::Session,
		params: web::Form<InParams>) -> Result<HttpResponse> {
		let hashed_password = crypto::mk_hash(&params.password);
		let (ret, session_str) = db::check_user(&params.login, &hashed_password);
		if ret>0 {
				session.set("session", session_str)?;
				Ok(HttpResponse::Ok().content_type("text/plain").body(format!(
						"your are a known user, check returned {}",
						ret
				)))
		}
		else {
				Ok(HttpResponse::NotFound().content_type("text/plain").body(
						"User not found or password not matched"))
		}
}


#[derive(Serialize, Deserialize)]
pub struct PersPageParams {
		perspage: String
}
async fn handle_perspage(
		session: actix_session::Session,
		params: web::Form<PersPageParams>) -> Result<HttpResponse> {

		let session_str: String;

		let s = session.get::<String>("session");
		match s {
				Ok(ss) => session_str = ss.unwrap_or_default(),
        Err(_error) => return Ok(HttpResponse::NotFound().content_type("text/plain").body(
						"User not authorized"))
		}

    log::trace!("session_str {}", session_str);

		let id_result = db::get_user_by_session(&session_str);
		match id_result {
        Err(_error) => return Ok(HttpResponse::NotFound().content_type("text/plain").body(
						"User not authorized")),
				Ok(id) => {
						db::set_perspage(id, &params.perspage).unwrap();
						Ok(HttpResponse::Ok().content_type("text/plain").body(
										"Personal page saved"))
				}
		}
}


async fn show_perspage(
		session: actix_session::Session,
		tmpl: web::Data<TinyTemplate<'_>>
) -> Result<HttpResponse> {
    let ctx = json!({
        "name" : &"some name",
        "surname" : &"some surname",
				"perspage" : &"some perspage"
    });
    let s = tmpl.render("../template/perspage.html", &ctx).unwrap();
        // .map_err(|_| error::ErrorInternalServerError("Template error"));
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}


fn init() -> Result<(), fern::InitError> {
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "TRACE".into());
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
