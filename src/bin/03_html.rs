use askama::Template;
use gotham::helpers::http::response::{create_empty_response, create_response};
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::state::{FromState, State};
use gotham_derive::{StateData, StaticResponseExtender};
use hyper::{Body, Response, StatusCode};
use serde::Deserialize;
use time::Date;

pub fn hello_web(state: State) -> (State, &'static str) {
    (state, "Hello, Web!")
}

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct HelloNamePath {
    name: String,
}

#[derive(Debug, Template)]
#[template(path = "hello_name.html")]
struct HelloNameTemplate {
    name: String,
    day_of_week: String,
}

pub fn hello_name(state: State) -> (State, Response<Body>) {
    let path = HelloNamePath::borrow_from(&state);

    let template = HelloNameTemplate {
        name:        path.name.clone(),
        day_of_week: Date::today().format("%A"),
    };

    let response = match template.render() {
        Ok(content) => create_response(
            &state,
            StatusCode::OK,
            mime::TEXT_HTML_UTF_8,
            content.into_bytes(),
        ),
        Err(e) => {
            eprintln!("{}", e);
            create_empty_response(&state, StatusCode::INTERNAL_SERVER_ERROR)
        },
    };

    (state, response)
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(hello_web);

        route.
            get("/:name").
            with_path_extractor::<HelloNamePath>().
            to(hello_name);
    })
}

fn main() {
    let addr = "127.0.0.1:7000";
    println!("Listening for requests at http://{}", addr);

    gotham::start(addr, router())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;
    use hyper::StatusCode;

    #[test]
    fn ascii_name() {
        let test_server = TestServer::new(router()).unwrap();
        let client = test_server.client();

        let response = client.
            get("http://localhost/Pesho").
            perform().
            unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_utf8_body().unwrap();
        assert!(body.contains("Hello, <u>Pesho</u>!"));
    }

    #[test]
    fn wrong_url() {
        let test_server = TestServer::new(router()).unwrap();
        let client = test_server.client();

        let response = client.
            get("http://localhost/other/url").
            perform().
            unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = response.read_utf8_body().unwrap();
        assert_eq!(body, "");
    }
}
