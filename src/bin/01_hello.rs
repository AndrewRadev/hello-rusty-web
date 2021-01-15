use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse};

async fn hello_web(_request: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Hello, Web!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:7000";
    println!("Listening for requests at http://{}", addr);

    let server = HttpServer::new(|| {
        App::new().
            route("/", web::get().to(hello_web))
    });

    server.bind(addr)?.run().await
}
