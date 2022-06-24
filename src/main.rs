use actix_web::{get, App, HttpResponse, HttpServer, Result};

#[get("/health")]
pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("success".to_string()))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Web server");
    HttpServer::new(|| App::new().service(health))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
