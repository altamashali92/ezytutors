use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// configure route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/health", 
        web::get()
            .to(health_check_handler)
    );
}

// configure handler
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

// instantiate and run server
#[actix_rt::main]
async fn main() -> io::Result<()> {
    let app = move || App::new().configure(general_routes);
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
