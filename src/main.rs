use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    let point = Point { x: 1, y: 2 };
    serde_json::to_string(&point).unwrap()
}

#[post("/test")]
async fn check(info: web::Json<Point>) -> impl Responder {
    println!("{:?}", info);
    web::Json(Point {
        x: info.x + 2,
        y: info.y + 3,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    match builder.set_private_key_file("key.pem", SslFiletype::PEM) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    match builder.set_certificate_chain_file("cert.pem") {
        Ok(_) => (),
        Err(error) => panic!("Chain problem error occure:{error}"),
    }

    HttpServer::new(|| App::new().service(index).service(check))
        .bind_openssl("127.0.0.1:8080", builder)?
        .shutdown_timeout(10)
        .run()
        .await
}
