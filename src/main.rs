use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn proxy() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").map(|p| p.parse().expect("Port must be a number")).unwrap_or(3000);

    let fut = HttpServer::new(|| {
        App::new()
            .default_service(web::route().to(proxy))
    })
    .bind(("127.0.0.1", port))?
    .run();

    println!("Server running at http://localhost:{port}");

    fut.await
}
