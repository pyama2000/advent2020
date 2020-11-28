use std::env;

use actix_redis::RedisSession;
use actix_session::Session;
use actix_web::{
    http::header::{ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_ORIGIN},
    middleware::{DefaultHeaders, Logger},
    web, App, Error, HttpResponse, HttpServer, Responder,
};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use serde_json::json;
use uuid::Uuid;

async fn login(session: Session) -> Result<impl Responder, Error> {
    let json = match session.get::<Uuid>("user_id")? {
        Some(user_id) => json!({"user_id": &user_id,}),
        None => {
            let user_id = Uuid::new_v4();
            session.set("user_id", &user_id)?;

            json!({ "user_id": &user_id })
        }
    };

    Ok(HttpResponse::Ok().json(&json))
}

async fn change(session: Session) -> Result<impl Responder, Error> {
    if session.get::<Uuid>("user_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let user_id = Uuid::new_v4();
    session.set("user_id", &user_id)?;

    Ok(HttpResponse::Ok().json(json!({ "user_id": &user_id })))
}

async fn add(session: Session) -> Result<impl Responder, Error> {
    if session.get::<Uuid>("user_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let count = session.get::<u32>("count")?.unwrap_or(0) + 1;
    session.set("count", &count)?;

    Ok(HttpResponse::Ok().json(json!({ "count": &count })))
}

async fn reset(session: Session) -> Result<impl Responder, Error> {
    if session.get::<Uuid>("user_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    session.set("count", 0)?;

    Ok(HttpResponse::Ok().json(json!({ "count": 0 })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_redis=info");
    env_logger::init();

    let host = env::var("host").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("port").unwrap_or_else(|_| "8000".to_string());
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "0.0.0.0:6379".to_string());

    let mut csp_rng = ChaCha20Rng::from_entropy();
    let mut data = [0u8; 32];
    csp_rng.fill_bytes(&mut data);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(RedisSession::new(&redis_url, &data))
            .wrap(DefaultHeaders::new().header(ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
            .wrap(DefaultHeaders::new().header(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true"))
            .route("/login", web::post().to(login))
            .route("/change", web::patch().to(change))
            .route("/add", web::post().to(add))
            .route("/reset", web::delete().to(reset))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
