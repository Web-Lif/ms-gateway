use actix_web::{middleware::{Logger}, web, App, HttpServer };
use chrono::Local;
use futures_util::future;
use sqlx::mysql::MySqlPoolOptions;
use log::{ info };

mod services;
mod middleware;
mod config;

fn cfg_fn(cfg: &mut web::ServiceConfig) {
    services::auth::configure(cfg);
}

async fn another_func(start: i64, url: String) -> std::io::Result<()> {
    info!("Loopback: {}", url);
    info!("Startup complete {} ms", Local::now().timestamp_millis() - start);
    Ok(())
} 

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    let start = Local::now();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let data_config = config::application::read_config();
    let pool =  MySqlPoolOptions::new()
        .min_connections(data_config.sqlx.min_connections.unwrap())
        .max_connections(data_config.sqlx.max_connections.unwrap())
        .connect(data_config.sqlx.url.as_str()).await.unwrap();
    
    let data = actix_web::web::Data::new(config::app_data::AppGlobalData {
        pool,
        config: data_config.clone()
    });

    let server = HttpServer::new(move || {
        App::new()
        .wrap(Logger::new("%a \"%r\" %s %b \"%{Referer}i\" %T"))
        .wrap(middleware::verify::VerifyToken)
        .app_data(data.clone())
        .service(web::scope("/api").configure(cfg_fn))
    })
        .bind((data_config.addr.as_str(), data_config.port.unwrap()))?
        .run();
    match future::join(
        server, 
        another_func(
            start.timestamp_millis(), 
            format!("http://{}:{}", data_config.addr.as_str(), data_config.port.unwrap())
        )
    ).await {
        (_, _) => {
        },
    }
    Ok(())
}