use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse};
use askama::Template;
use chrono::Local;

#[derive(Debug, Template)]
#[template(path = "hello_name.html")]
struct HelloNameTemplate<'a> {
    name: &'a str,
    day_of_week: String,
}

async fn hello_web(_request: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Hello, Web!")
}

pub async fn hello_name(request: HttpRequest) -> HttpResponse {
    let template = HelloNameTemplate {
        name:        request.match_info().get("name").unwrap(),
        day_of_week: Local::now().format("%A").to_string(),
    };

    match template.render() {
        Ok(content) => {
            HttpResponse::Ok().
                content_type("text/html").
                body(content)
        },
        Err(e) => {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        },
    }
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
    async fn test_hello_name_integration() {
        let mut app_service = test::init_service(build_app!()).await;
        let request = TestRequest::get().
            insert_header(("content-type", "text/plain")).
            uri("/Pesho").
            to_request();
        let response = test::call_service(&mut app_service, request).await;

        assert!(response.status().is_success());

        let body = test::read_body(response).await;
        assert!(String::from_utf8(body.to_vec()).unwrap().contains("Hello, <u>Pesho</u>!"));
    }

    #[actix_web::test]
    async fn test_unknown_url() {
        let mut app_service = test::init_service(build_app!()).await;
        let request = TestRequest::get().
            insert_header(("content-type", "text/plain")).
            uri("/something/unknown").
            to_request();
        let response = test::call_service(&mut app_service, request).await;

        assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
    }
}
