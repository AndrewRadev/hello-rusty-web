use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse};

async fn hello_web(_request: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Hello, Web!")
}

pub async fn hello_name(request: HttpRequest) -> HttpResponse {
    let name = request.match_info().get("name").unwrap();
    let greeting = format!("Hello, {}!", name);

    HttpResponse::Ok().body(greeting)
}

macro_rules! build_app {
    () => {
        App::new().
            route("/",       web::get().to(hello_web)).
            route("/{name}", web::get().to(hello_name))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:7000";
    println!("Listening for requests at http://{}", addr);

    let server = HttpServer::new(|| build_app!());

    server.bind(addr)?.run().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::{self, TestRequest};

    #[actix_web::test]
    async fn test_hello_name_unit() {
        let request = TestRequest::get().
            insert_header(("content-type", "text/plain")).
            param("name", "Pesho").
            to_http_request();
        let response = hello_name(request).await;

        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_hello_name_integration() {
        let mut app_service = test::init_service(build_app!()).await;
        let request = TestRequest::get().
            insert_header(("content-type", "text/plain")).
            uri("/Pesho").
            to_request();
        let response = test::call_service(&mut app_service, request).await;

        assert!(response.status().is_success());

        let body = test::read_body(response).await;
        assert_eq!(body, web::Bytes::from_static(b"Hello, Pesho!"));
    }
}
