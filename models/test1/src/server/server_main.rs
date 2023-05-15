// PJ entry point
// first run is

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

async fn actix_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from actix_hello")
}
fn port() -> u16 {
    8080
}

#[actix_web::main]
async fn actix_main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
        .bind(("127.0.0.1", port()))?
        .run()
        .await
}

pub fn main () {
    println!("Hello from runner {}" ,port().to_string());
    actix_main();
}
/*
*
 root/
    msg/
        msgstart.rs
        msgend.rs
    server/
        server_main.rs
        db_call/
            db_call.rs
            db_call.rs
 */
