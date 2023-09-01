use crate::repository::prelude::WorkItemRepository;
use handler::prelude::work_items_handler;
mod entity;
mod handler;
mod repository;
mod test;
use actix_cors::Cors;
use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpServer,
};
use sea_orm::Database;
use std::env;
use actix_web::http::header;

#[derive(Debug, Clone)]
pub struct AppState {
    pub repository: WorkItemRepository,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    dotenv::dotenv().ok();
    env_logger::init();
    let conn = env::var("DATABASE_URL").expect("DATABASE_URL belirtilmeli");
    let db_conn = Database::connect(&conn).await.unwrap();
    let host = env::var("HOST").expect("HOST tanımı yapılmalı");
    let port = env::var("PORT").expect("PORT tanımı yapılmalı");
    let server_url = format!("{}:{}", host, port);

    let repository = WorkItemRepository {
        db: db_conn.clone(),
    };
    let state = AppState { repository };
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://localhost:5173/")
            .allowed_methods(vec!["GET", "POST","PUT","DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .configure(init)
    })
    .bind(&server_url)?;
    println!("Sunucu {} adresinden çalışıyor.", server_url);
    server.run().await?;
    Ok(())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(work_items_handler());
}
