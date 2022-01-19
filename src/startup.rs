use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;

use std::net::TcpListener;

use crate::routes;

pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_connection = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(db_connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
