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

async fn hello_name(request: HttpRequest) -> HttpResponse {
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

    async fn perform_get_request(uri: &str) -> (http::StatusCode, String) {
        let mut app_service = test::init_service(build_app!()).await;
        let request = TestRequest::with_uri(uri).to_request();
        let response = test::call_service(&mut app_service, request).await;

        let status = response.status();

        let body = test::read_body(response).await;
        let body = String::from_utf8(body.to_vec()).unwrap();

        (status, body)
    }

    #[actix_web::test]
    async fn test_hello_name_integration() {
        let (status, body) = perform_get_request("/Pesho").await;

        assert!(status.is_success());
        assert!(body.contains("Hello, <u>Pesho</u>!"));

        let (status, body) = perform_get_request("/Gosho").await;

        assert!(status.is_success());
        assert!(body.contains("Hello, <u>Gosho</u>!"));
    }

    #[actix_web::test]
    async fn test_unknown_url() {
        let (status, _) = perform_get_request("/unknown/path").await;

        assert_eq!(status, http::StatusCode::NOT_FOUND);
    }
}
