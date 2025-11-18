use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, web};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();

    println!("\n{}", "=".repeat(60));
    println!("\n\tServer running at : http://127.0.0.1:8000\n");
    println!("{}\n", "=".repeat(60));

    Ok(server)
}
