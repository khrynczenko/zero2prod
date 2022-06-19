use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::routes;

pub fn run(listener: TcpListener, db_connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection = Data::new(db_connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(connection.clone())
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
