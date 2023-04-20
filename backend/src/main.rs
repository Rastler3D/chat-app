pub mod db;
pub mod broadcaster;
use std::collections::HashMap;
use std::env;
use actix_web::{App, HttpResponse, HttpServer, Responder, get,post, Either};
use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_session::{Session, SessionMiddleware};
use actix_session::storage::{ RedisSessionStore};
use actix_web::cookie::{Key};
use actix_web::middleware::Logger;
use actix_web::web::{Data, Json};
use actix_web_lab::sse;
use diesel::{PgConnection, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use serde_json::json;
use crate::broadcaster::{ChatBroadcaster, Connect, DbPool};
use crate::db::schema::messages::dsl::messages;
use actix_web_lab::extract::Path;
use actix_session::config::PersistentSession;
use actix_web::cookie::time::Duration;
use actix_web_lab::__reexports::tracing::info;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use uuid::Uuid;
use crate::db::models::Message;
use crate::db::sequences::user_id;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[post("/chat/send")]
async fn send_message(session: Session, message: String, broadcaster: Data<Addr<ChatBroadcaster>>) -> impl Responder{
    if let Some(user_id) = session.get::<usize>("user_id").unwrap(){
        broadcaster.send(broadcaster::SendMessage(message,user_id, session.get("user_name").unwrap().unwrap())).await.unwrap();
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[get("/chat/logout")]
async fn logout(session: Session) -> impl Responder{
    if let Some(_) = session.get::<usize>("user_id").unwrap() {
        session.purge();
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[get("/chat/login")]
async fn login(session: Session) -> impl Responder{
    if let Some(user_id) = session.get::<usize>("user_id").unwrap() {
        let user_name = session.get::<String>("user_name").unwrap().unwrap();
        HttpResponse::Ok().json(json! {
            {
                "user_id": user_id,
                "user_name": user_name
            }
        })
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[get("/chat/signup/{user_name}")]
async fn sign_up(session: Session, db: Data<DbPool>, Path(user_name): Path<String>) -> impl Responder{
    let mut conn = db.get().unwrap();
    let user_id = user_id::next_val(&mut conn);
    session.insert("user_id", user_id).unwrap();
    session.insert("user_name", user_name.clone()).unwrap();

    HttpResponse::Ok().json(json!{
        {
            "user_id": user_id,
            "user_name": user_name
        }
    })
}

#[get("/chat/history")]
async fn chat_history(db: Data<DbPool>) -> impl Responder {
    let mut conn = db.get().unwrap();
    let messages_list = messages.get_results::<Message>(&mut conn).unwrap();
    Json(messages_list)
}

#[get("/chat")]
async fn connect(session: Session, broadcaster: Data<Addr<ChatBroadcaster>>) -> impl Responder {
    if let Some(user_id) = session.get::<usize>("user_id").unwrap(){
        let (tx,rx) = sse::channel(10);
        let session = broadcaster::Session{
            id: Uuid::new_v4(),
            name: session.get::<String>("user_name").unwrap().unwrap(),
            user_id,
            channel: tx
        };
        broadcaster.send(Connect(session)).await.unwrap();
        Either::Left(rx)
    } else {
        Either::Right(HttpResponse::Unauthorized().finish())
    }

}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv().is_ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let redis_url = env::var("REDIS_URL").unwrap();
    let redis_session = RedisSessionStore::new(redis_url)
        .await
        .unwrap();
    let postgres_url = env::var("DATABASE_URL").unwrap();
    let postgres = Pool::new(ConnectionManager::<PgConnection>::new(postgres_url)).unwrap();
    run_migration(&mut postgres.get().unwrap());
    let key = Key::generate();
    let broadcaster = ChatBroadcaster{ db: postgres.clone(), sessions: HashMap::new() }.start();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(broadcaster.clone()))
            .app_data(Data::new(postgres.clone()))
            .wrap(
            SessionMiddleware::builder(
                    redis_session.clone(),
                    key.clone()
                    )
                    .cookie_secure(false)
                    .cookie_http_only(false)
                    .session_lifecycle(PersistentSession::default().session_ttl(Duration::hours(1)))
                    .build())
            .wrap(Logger::default())
            .wrap(Cors::default()
                .allow_any_header()
                .allow_any_method()
                .allow_any_origin()
                .supports_credentials()
                .expose_any_header()
            )
            .service(logout)
            .service(sign_up)
            .service(chat_history)
            .service(send_message)
            .service(login)
            .service(connect)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

fn run_migration(conn: &mut PgConnection) {
    info!("Migrations: {}", "Applying migrations");
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}


