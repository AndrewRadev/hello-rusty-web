use std::collections::HashMap;

use actix_files::{NamedFile, Files};
use actix_session::{CookieSession, Session};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::{web, get, guard, App, HttpRequest, HttpServer, HttpResponse, Result};
use askama::Template;
use chrono::Local;
use env_logger::{self, Env};

#[derive(Debug, Template)]
#[template(path = "greeting.html")]
struct GreetingTemplate {
    name: String,
    greeting: String,
    visit_count: u32,
    day_of_week: String,
}

#[get("/")]
async fn hello_web(_request: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Hello, Web!")
}

#[get("/{greeting}/{name}")]
async fn greeting(
    path:    web::Path<(String, String)>,
    session: Session,
) -> Result<HttpResponse> {
    let web::Path((greeting, name)) = path;
    let mut counter = session.get::<HashMap<String, u32>>("counter")?.
        unwrap_or_else(HashMap::new);

    (*counter.entry(name.clone()).or_insert(0) += 1);
    let visit_count = counter[&name];
    session.set("counter", counter)?;

    let template = GreetingTemplate {
        greeting, name, visit_count,
        day_of_week: Local::now().format("%A").to_string(),
    };

    let content = template.render().
        map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;

    let response = HttpResponse::Ok().
        content_type("text/html").
        body(content);

    Ok(response)
}

async fn render_404() -> Result<NamedFile> {
    let file = NamedFile::open("static/404.html")?;
    Ok(file.set_status_code(StatusCode::NOT_FOUND))
}

async fn render_admin() -> Result<NamedFile> {
    let file = NamedFile::open("static/admin.html")?;
    Ok(file)
}

const SESSION_SECRET: &[u8; 32] = b"6d10a2873e2c4a2282eecd2d1aa3471e";

macro_rules! build_app {
    () => {
        App::new().
            wrap(Logger::default()).
            wrap(CookieSession::private(SESSION_SECRET).secure(true)).
            service(Files::new("/static", "static").show_files_listing()).
            service(hello_web).
            service(greeting).
            route("/admin", web::get().to(render_admin)).
            default_service(
                web::resource("").
                route(web::get().to(render_404)).
                route(
                    web::route().
                    guard(guard::Not(guard::Get())).
                    to(HttpResponse::MethodNotAllowed),
                ),
            )
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:7000";
    println!("Listening for requests at http://{}", addr);

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let server = HttpServer::new(|| build_app!());

    server.bind(addr)?.run().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::{self, TestRequest};
    use url::Url;

    async fn perform_get_request(uri: &str) -> (http::StatusCode, String) {
        let mut app_service = test::init_service(build_app!()).await;
        let request = TestRequest::get().
            header("content-type", "text/plain").
            uri(uri).
            to_request();
        let response = test::call_service(&mut app_service, request).await;

        let status = response.status();

        let body = test::read_body(response).await;
        let body = String::from_utf8(body.to_vec()).unwrap();

        (status, body)
    }

    #[actix_rt::test]
    async fn test_hello_name_with_ascii() {
        let (status, body) = perform_get_request("/Hello/Pesho").await;

        assert!(status.is_success());
        assert!(body.contains("Hello, <u>Pesho</u>!"));

        let (status, body) = perform_get_request("/Greetings/Penka").await;

        assert!(status.is_success());
        assert!(body.contains("Greetings, <u>Penka</u>!"));
    }

    #[actix_rt::test]
    async fn test_hello_name_with_unicode() {
        let url = Url::parse("http://test.host/Здравей/Тинчо").unwrap();
        let (status, body) = perform_get_request(url.path()).await;

        assert!(status.is_success());
        assert!(body.contains("Здравей, <u>Тинчо</u>!"));
    }

    #[actix_rt::test]
    async fn test_unknown_url() {
        let (status, _) = perform_get_request("/some/unknown/path").await;

        assert_eq!(status, http::StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_session_counter() {
        // ??? :/
        //
        // Need to juggle cookies manually, and that's not fun (T_T)
    }
}
